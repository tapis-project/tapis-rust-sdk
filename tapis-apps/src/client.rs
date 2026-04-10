use crate::apis::{
    applications_api, configuration, general_api, permissions_api, sharing_api, Error,
};
use crate::models;
use http::header::{HeaderMap, HeaderValue};
use reqwest::{Client, Request, Response};
use reqwest_middleware::{ClientBuilder, Middleware, Next, Result as MiddlewareResult};
use std::sync::Arc;
use tapis_core::TokenProvider;

tokio::task_local! {
    /// Extra headers to inject into every request within a [`with_headers`] scope.
    static EXTRA_HEADERS: HeaderMap;
}

/// Run an async call with additional HTTP headers injected into every request
/// made within the future `f`. Headers are scoped to this task only, so
/// concurrent calls with different headers are safe.
pub async fn with_headers<F, T>(headers: HeaderMap, f: F) -> T
where
    F: std::future::Future<Output = T>,
{
    EXTRA_HEADERS.scope(headers, f).await
}

#[derive(Debug)]
struct LoggingMiddleware;

#[derive(Debug)]
struct HeaderInjectionMiddleware;

#[async_trait::async_trait]
impl Middleware for LoggingMiddleware {
    async fn handle(
        &self,
        req: Request,
        extensions: &mut http::Extensions,
        next: Next<'_>,
    ) -> MiddlewareResult<Response> {
        let method = req.method().clone();
        let url = req.url().clone();
        println!("Tapis SDK request: {} {}", method, url);
        next.run(req, extensions).await
    }
}

#[async_trait::async_trait]
impl Middleware for HeaderInjectionMiddleware {
    async fn handle(
        &self,
        mut req: Request,
        extensions: &mut http::Extensions,
        next: Next<'_>,
    ) -> MiddlewareResult<Response> {
        let _ = EXTRA_HEADERS.try_with(|headers| {
            for (k, v) in headers {
                req.headers_mut().insert(k, v.clone());
            }
        });
        next.run(req, extensions).await
    }
}

fn validate_tracking_id(tracking_id: &str) -> Result<(), String> {
    if !tracking_id.is_ascii() {
        return Err("X-Tapis-Tracking-ID must be an entirely ASCII string.".to_string());
    }
    if tracking_id.len() > 126 {
        return Err("X-Tapis-Tracking-ID must be less than 126 characters.".to_string());
    }
    if tracking_id.matches('.').count() != 1 {
        return Err("X-Tapis-Tracking-ID must contain exactly one '.' (format: <namespace>.<unique_identifier>).".to_string());
    }
    if tracking_id.starts_with('.') || tracking_id.ends_with('.') {
        return Err("X-Tapis-Tracking-ID cannot start or end with '.'.".to_string());
    }
    let (namespace, unique_id) = tracking_id.split_once('.').unwrap();
    if !namespace.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Err("X-Tapis-Tracking-ID namespace must contain only alphanumeric characters and underscores.".to_string());
    }
    if !unique_id.chars().all(|c| c.is_alphanumeric() || c == '-') {
        return Err("X-Tapis-Tracking-ID unique identifier must contain only alphanumeric characters and hyphens.".to_string());
    }
    Ok(())
}

#[derive(Debug)]
struct TrackingIdMiddleware;

#[async_trait::async_trait]
impl Middleware for TrackingIdMiddleware {
    async fn handle(
        &self,
        mut req: Request,
        extensions: &mut http::Extensions,
        next: Next<'_>,
    ) -> MiddlewareResult<Response> {
        let tracking_key = req
            .headers()
            .keys()
            .find(|k| {
                let s = k.as_str();
                s.eq_ignore_ascii_case("x-tapis-tracking-id")
                    || s.eq_ignore_ascii_case("x_tapis_tracking_id")
            })
            .cloned();
        if let Some(key) = tracking_key {
            let tracking_id = req
                .headers()
                .get(&key)
                .and_then(|v| v.to_str().ok())
                .map(|s| s.to_owned());
            if let Some(id) = tracking_id {
                req.headers_mut().remove(&key);
                validate_tracking_id(&id)
                    .map_err(|e| reqwest_middleware::Error::Middleware(anyhow::anyhow!(e)))?;
                let name = reqwest::header::HeaderName::from_static("x-tapis-tracking-id");
                let value = reqwest::header::HeaderValue::from_str(&id)
                    .map_err(|e| reqwest_middleware::Error::Middleware(anyhow::anyhow!(e)))?;
                req.headers_mut().insert(name, value);
            }
        }
        next.run(req, extensions).await
    }
}

/// Decode a base64url-encoded segment (no padding required) into raw bytes.
fn decode_base64url(s: &str) -> Option<Vec<u8>> {
    fn val(c: u8) -> Option<u8> {
        match c {
            b'A'..=b'Z' => Some(c - b'A'),
            b'a'..=b'z' => Some(c - b'a' + 26),
            b'0'..=b'9' => Some(c - b'0' + 52),
            b'-' | b'+' => Some(62),
            b'_' | b'/' => Some(63),
            _ => None,
        }
    }
    let chars: Vec<u8> = s.bytes().filter(|&b| b != b'=').collect();
    let mut out = Vec::with_capacity(chars.len() * 3 / 4 + 1);
    let mut i = 0;
    while i < chars.len() {
        let a = val(chars[i])?;
        let b = val(*chars.get(i + 1)?)?;
        out.push((a << 2) | (b >> 4));
        if let Some(&c3) = chars.get(i + 2) {
            let c = val(c3)?;
            out.push(((b & 0x0f) << 4) | (c >> 2));
            if let Some(&c4) = chars.get(i + 3) {
                let d = val(c4)?;
                out.push(((c & 0x03) << 6) | d);
            }
        }
        i += 4;
    }
    Some(out)
}

/// Extract the `exp` (expiration) claim from a JWT without verifying the signature.
fn extract_jwt_exp(token: &str) -> Option<i64> {
    let payload_b64 = token.split('.').nth(1)?;
    let bytes = decode_base64url(payload_b64)?;
    let claims: serde_json::Value = serde_json::from_slice(&bytes).ok()?;
    claims.get("exp")?.as_i64()
}

struct RefreshMiddleware {
    token_provider: Arc<dyn TokenProvider>,
}

#[async_trait::async_trait]
impl Middleware for RefreshMiddleware {
    async fn handle(
        &self,
        mut req: Request,
        extensions: &mut http::Extensions,
        next: Next<'_>,
    ) -> MiddlewareResult<Response> {
        let is_token_endpoint = {
            let url = req.url().as_str();
            url.contains("/oauth2/tokens") || url.contains("/v3/tokens")
        };
        if !is_token_endpoint {
            let needs_refresh = req
                .headers()
                .get("x-tapis-token")
                .and_then(|v| v.to_str().ok())
                .and_then(extract_jwt_exp)
                .map(|exp| {
                    let now = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .map(|d| d.as_secs() as i64)
                        .unwrap_or(0);
                    exp - now < 5
                })
                .unwrap_or(false);
            if needs_refresh {
                if let Some(new_token) = self.token_provider.get_token().await {
                    let value = HeaderValue::from_str(&new_token)
                        .map_err(|e| reqwest_middleware::Error::Middleware(anyhow::anyhow!(e)))?;
                    req.headers_mut().insert("x-tapis-token", value);
                }
            }
        }
        next.run(req, extensions).await
    }
}

#[derive(Clone)]
pub struct TapisApps {
    config: Arc<configuration::Configuration>,
    pub applications: ApplicationsClient,
    pub general: GeneralClient,
    pub permissions: PermissionsClient,
    pub sharing: SharingClient,
}

impl TapisApps {
    pub fn new(
        base_url: &str,
        jwt_token: Option<&str>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Self::build(base_url, jwt_token, None)
    }

    /// Create a client with a [`TokenProvider`] for automatic token refresh.
    /// `RefreshMiddleware` is added to the middleware chain and will call
    /// `provider.get_token()` transparently whenever the JWT is about to expire.
    pub fn with_token_provider(
        base_url: &str,
        jwt_token: Option<&str>,
        provider: Arc<dyn TokenProvider>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Self::build(base_url, jwt_token, Some(provider))
    }

    fn build(
        base_url: &str,
        jwt_token: Option<&str>,
        token_provider: Option<Arc<dyn TokenProvider>>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut headers = HeaderMap::new();
        if let Some(token) = jwt_token {
            headers.insert("X-Tapis-Token", HeaderValue::from_str(token)?);
        }

        let reqwest_client = Client::builder().default_headers(headers).build()?;

        let mut builder = ClientBuilder::new(reqwest_client)
            .with(LoggingMiddleware)
            .with(HeaderInjectionMiddleware)
            .with(TrackingIdMiddleware);

        if let Some(provider) = token_provider {
            builder = builder.with(RefreshMiddleware {
                token_provider: provider,
            });
        }

        let client = builder.build();

        let config = Arc::new(configuration::Configuration {
            base_path: base_url.to_string(),
            client,
            ..Default::default()
        });

        Ok(Self {
            config: config.clone(),
            applications: ApplicationsClient {
                config: config.clone(),
            },
            general: GeneralClient {
                config: config.clone(),
            },
            permissions: PermissionsClient {
                config: config.clone(),
            },
            sharing: SharingClient {
                config: config.clone(),
            },
        })
    }

    pub fn config(&self) -> &configuration::Configuration {
        &self.config
    }
}

#[derive(Clone)]
pub struct ApplicationsClient {
    config: Arc<configuration::Configuration>,
}

impl ApplicationsClient {
    pub async fn change_app_owner(
        &self,
        app_id: &str,
        user_name: &str,
    ) -> Result<models::RespChangeCount, Error<applications_api::ChangeAppOwnerError>> {
        applications_api::change_app_owner(&self.config, app_id, user_name).await
    }

    pub async fn create_app_version(
        &self,
        req_post_app: models::ReqPostApp,
    ) -> Result<models::RespResourceUrl, Error<applications_api::CreateAppVersionError>> {
        applications_api::create_app_version(&self.config, req_post_app).await
    }

    pub async fn delete_app(
        &self,
        app_id: &str,
    ) -> Result<models::RespChangeCount, Error<applications_api::DeleteAppError>> {
        applications_api::delete_app(&self.config, app_id).await
    }

    pub async fn disable_app(
        &self,
        app_id: &str,
    ) -> Result<models::RespChangeCount, Error<applications_api::DisableAppError>> {
        applications_api::disable_app(&self.config, app_id).await
    }

    pub async fn disable_app_version(
        &self,
        app_id: &str,
        app_version: &str,
    ) -> Result<models::RespChangeCount, Error<applications_api::DisableAppVersionError>> {
        applications_api::disable_app_version(&self.config, app_id, app_version).await
    }

    pub async fn enable_app(
        &self,
        app_id: &str,
    ) -> Result<models::RespChangeCount, Error<applications_api::EnableAppError>> {
        applications_api::enable_app(&self.config, app_id).await
    }

    pub async fn enable_app_version(
        &self,
        app_id: &str,
        app_version: &str,
    ) -> Result<models::RespChangeCount, Error<applications_api::EnableAppVersionError>> {
        applications_api::enable_app_version(&self.config, app_id, app_version).await
    }

    pub async fn get_app(
        &self,
        app_id: &str,
        app_version: &str,
        require_exec_perm: Option<bool>,
        impersonation_id: Option<&str>,
        select: Option<&str>,
        resource_tenant: Option<&str>,
    ) -> Result<models::RespApp, Error<applications_api::GetAppError>> {
        applications_api::get_app(
            &self.config,
            app_id,
            app_version,
            require_exec_perm,
            impersonation_id,
            select,
            resource_tenant,
        )
        .await
    }

    pub async fn get_app_latest_version(
        &self,
        app_id: &str,
        require_exec_perm: Option<bool>,
        select: Option<&str>,
        resource_tenant: Option<&str>,
        impersonation_id: Option<&str>,
    ) -> Result<models::RespApp, Error<applications_api::GetAppLatestVersionError>> {
        applications_api::get_app_latest_version(
            &self.config,
            app_id,
            require_exec_perm,
            select,
            resource_tenant,
            impersonation_id,
        )
        .await
    }

    pub async fn get_apps(
        &self,
        search: Option<&str>,
        list_type: Option<models::ListTypeEnum>,
        limit: Option<i32>,
        order_by: Option<&str>,
        skip: Option<i32>,
        start_after: Option<&str>,
        compute_total: Option<bool>,
        select: Option<&str>,
        show_deleted: Option<bool>,
        impersonation_id: Option<&str>,
    ) -> Result<models::RespApps, Error<applications_api::GetAppsError>> {
        applications_api::get_apps(
            &self.config,
            search,
            list_type,
            limit,
            order_by,
            skip,
            start_after,
            compute_total,
            select,
            show_deleted,
            impersonation_id,
        )
        .await
    }

    pub async fn get_history(
        &self,
        app_id: &str,
    ) -> Result<models::RespAppHistory, Error<applications_api::GetHistoryError>> {
        applications_api::get_history(&self.config, app_id).await
    }

    pub async fn is_enabled(
        &self,
        app_id: &str,
        version: Option<&str>,
    ) -> Result<models::RespBoolean, Error<applications_api::IsEnabledError>> {
        applications_api::is_enabled(&self.config, app_id, version).await
    }

    pub async fn lock_app(
        &self,
        app_id: &str,
        app_version: &str,
    ) -> Result<models::RespChangeCount, Error<applications_api::LockAppError>> {
        applications_api::lock_app(&self.config, app_id, app_version).await
    }

    pub async fn patch_app(
        &self,
        app_id: &str,
        app_version: &str,
        req_patch_app: models::ReqPatchApp,
    ) -> Result<models::RespResourceUrl, Error<applications_api::PatchAppError>> {
        applications_api::patch_app(&self.config, app_id, app_version, req_patch_app).await
    }

    pub async fn put_app(
        &self,
        app_id: &str,
        app_version: &str,
        req_put_app: models::ReqPutApp,
    ) -> Result<models::RespResourceUrl, Error<applications_api::PutAppError>> {
        applications_api::put_app(&self.config, app_id, app_version, req_put_app).await
    }

    pub async fn search_apps_query_parameters(
        &self,
        list_type: Option<models::ListTypeEnum>,
        limit: Option<i32>,
        order_by: Option<&str>,
        skip: Option<i32>,
        start_after: Option<&str>,
        compute_total: Option<bool>,
        select: Option<&str>,
    ) -> Result<models::RespApps, Error<applications_api::SearchAppsQueryParametersError>> {
        applications_api::search_apps_query_parameters(
            &self.config,
            list_type,
            limit,
            order_by,
            skip,
            start_after,
            compute_total,
            select,
        )
        .await
    }

    pub async fn search_apps_request_body(
        &self,
        req_search_apps: models::ReqSearchApps,
        list_type: Option<models::ListTypeEnum>,
        limit: Option<i32>,
        order_by: Option<&str>,
        skip: Option<i32>,
        start_after: Option<&str>,
        compute_total: Option<bool>,
        select: Option<&str>,
    ) -> Result<models::RespApps, Error<applications_api::SearchAppsRequestBodyError>> {
        applications_api::search_apps_request_body(
            &self.config,
            req_search_apps,
            list_type,
            limit,
            order_by,
            skip,
            start_after,
            compute_total,
            select,
        )
        .await
    }

    pub async fn undelete_app(
        &self,
        app_id: &str,
    ) -> Result<models::RespChangeCount, Error<applications_api::UndeleteAppError>> {
        applications_api::undelete_app(&self.config, app_id).await
    }

    pub async fn unlock_app(
        &self,
        app_id: &str,
        app_version: &str,
    ) -> Result<models::RespChangeCount, Error<applications_api::UnlockAppError>> {
        applications_api::unlock_app(&self.config, app_id, app_version).await
    }
}

#[derive(Clone)]
pub struct GeneralClient {
    config: Arc<configuration::Configuration>,
}

impl GeneralClient {
    pub async fn health_check(
        &self,
    ) -> Result<models::RespBasic, Error<general_api::HealthCheckError>> {
        general_api::health_check(&self.config).await
    }

    pub async fn ready_check(
        &self,
    ) -> Result<models::RespBasic, Error<general_api::ReadyCheckError>> {
        general_api::ready_check(&self.config).await
    }
}

#[derive(Clone)]
pub struct PermissionsClient {
    config: Arc<configuration::Configuration>,
}

impl PermissionsClient {
    pub async fn get_user_perms(
        &self,
        app_id: &str,
        user_name: &str,
    ) -> Result<models::RespNameArray, Error<permissions_api::GetUserPermsError>> {
        permissions_api::get_user_perms(&self.config, app_id, user_name).await
    }

    pub async fn grant_user_perms(
        &self,
        app_id: &str,
        user_name: &str,
        req_perms: models::ReqPerms,
    ) -> Result<models::RespBasic, Error<permissions_api::GrantUserPermsError>> {
        permissions_api::grant_user_perms(&self.config, app_id, user_name, req_perms).await
    }

    pub async fn revoke_user_perm(
        &self,
        app_id: &str,
        user_name: &str,
        permission: &str,
    ) -> Result<models::RespBasic, Error<permissions_api::RevokeUserPermError>> {
        permissions_api::revoke_user_perm(&self.config, app_id, user_name, permission).await
    }

    pub async fn revoke_user_perms(
        &self,
        app_id: &str,
        user_name: &str,
        req_perms: models::ReqPerms,
    ) -> Result<models::RespBasic, Error<permissions_api::RevokeUserPermsError>> {
        permissions_api::revoke_user_perms(&self.config, app_id, user_name, req_perms).await
    }
}

#[derive(Clone)]
pub struct SharingClient {
    config: Arc<configuration::Configuration>,
}

impl SharingClient {
    pub async fn get_share_info(
        &self,
        app_id: &str,
    ) -> Result<models::RespShareInfo, Error<sharing_api::GetShareInfoError>> {
        sharing_api::get_share_info(&self.config, app_id).await
    }

    pub async fn share_app(
        &self,
        app_id: &str,
        req_share_update: models::ReqShareUpdate,
    ) -> Result<models::RespBasic, Error<sharing_api::ShareAppError>> {
        sharing_api::share_app(&self.config, app_id, req_share_update).await
    }

    pub async fn share_app_public(
        &self,
        app_id: &str,
    ) -> Result<models::RespBasic, Error<sharing_api::ShareAppPublicError>> {
        sharing_api::share_app_public(&self.config, app_id).await
    }

    pub async fn un_share_app(
        &self,
        app_id: &str,
        req_share_update: models::ReqShareUpdate,
    ) -> Result<models::RespBasic, Error<sharing_api::UnShareAppError>> {
        sharing_api::un_share_app(&self.config, app_id, req_share_update).await
    }

    pub async fn un_share_app_public(
        &self,
        app_id: &str,
    ) -> Result<models::RespBasic, Error<sharing_api::UnShareAppPublicError>> {
        sharing_api::un_share_app_public(&self.config, app_id).await
    }
}
