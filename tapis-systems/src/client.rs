use crate::apis::{
    child_systems_api, configuration, credentials_api, general_api, permissions_api,
    scheduler_profiles_api, sharing_api, systems_api, Error,
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
pub struct TapisSystems {
    config: Arc<configuration::Configuration>,
    pub child_systems: ChildSystemsClient,
    pub credentials: CredentialsClient,
    pub general: GeneralClient,
    pub permissions: PermissionsClient,
    pub scheduler_profiles: SchedulerProfilesClient,
    pub sharing: SharingClient,
    pub systems: SystemsClient,
}

impl TapisSystems {
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
            child_systems: ChildSystemsClient {
                config: config.clone(),
            },
            credentials: CredentialsClient {
                config: config.clone(),
            },
            general: GeneralClient {
                config: config.clone(),
            },
            permissions: PermissionsClient {
                config: config.clone(),
            },
            scheduler_profiles: SchedulerProfilesClient {
                config: config.clone(),
            },
            sharing: SharingClient {
                config: config.clone(),
            },
            systems: SystemsClient {
                config: config.clone(),
            },
        })
    }

    pub fn config(&self) -> &configuration::Configuration {
        &self.config
    }
}

#[derive(Clone)]
pub struct ChildSystemsClient {
    config: Arc<configuration::Configuration>,
}

impl ChildSystemsClient {
    pub async fn create_child_system(
        &self,
        parent_id: &str,
        req_post_child_system: models::ReqPostChildSystem,
    ) -> Result<models::RespResourceUrl, Error<child_systems_api::CreateChildSystemError>> {
        child_systems_api::create_child_system(&self.config, parent_id, req_post_child_system).await
    }

    pub async fn unlink_children(
        &self,
        parent_system_id: &str,
        all: Option<bool>,
        req_unlink_children: Option<models::ReqUnlinkChildren>,
    ) -> Result<models::RespChangeCount, Error<child_systems_api::UnlinkChildrenError>> {
        child_systems_api::unlink_children(&self.config, parent_system_id, all, req_unlink_children)
            .await
    }

    pub async fn unlink_from_parent(
        &self,
        child_system_id: &str,
    ) -> Result<models::RespChangeCount, Error<child_systems_api::UnlinkFromParentError>> {
        child_systems_api::unlink_from_parent(&self.config, child_system_id).await
    }
}

#[derive(Clone)]
pub struct CredentialsClient {
    config: Arc<configuration::Configuration>,
}

impl CredentialsClient {
    pub async fn check_user_credential(
        &self,
        system_id: &str,
        user_name: &str,
        authn_method: Option<&str>,
    ) -> Result<models::RespBasic, Error<credentials_api::CheckUserCredentialError>> {
        credentials_api::check_user_credential(&self.config, system_id, user_name, authn_method)
            .await
    }

    pub async fn create_user_credential(
        &self,
        system_id: &str,
        user_name: &str,
        req_update_credential: models::ReqUpdateCredential,
        create_tms_keys: Option<bool>,
        skip_credential_check: Option<bool>,
    ) -> Result<models::RespBasic, Error<credentials_api::CreateUserCredentialError>> {
        credentials_api::create_user_credential(
            &self.config,
            system_id,
            user_name,
            req_update_credential,
            create_tms_keys,
            skip_credential_check,
        )
        .await
    }

    pub async fn generate_globus_tokens(
        &self,
        system_id: &str,
        user_name: &str,
        auth_code: &str,
        session_id: &str,
    ) -> Result<models::RespBasic, Error<credentials_api::GenerateGlobusTokensError>> {
        credentials_api::generate_globus_tokens(
            &self.config,
            system_id,
            user_name,
            auth_code,
            session_id,
        )
        .await
    }

    pub async fn get_globus_auth_url(
        &self,
        system_id: &str,
    ) -> Result<models::RespGlobusAuthUrl, Error<credentials_api::GetGlobusAuthUrlError>> {
        credentials_api::get_globus_auth_url(&self.config, system_id).await
    }

    pub async fn get_user_credential(
        &self,
        system_id: &str,
        user_name: &str,
        authn_method: Option<&str>,
    ) -> Result<models::RespCredential, Error<credentials_api::GetUserCredentialError>> {
        credentials_api::get_user_credential(&self.config, system_id, user_name, authn_method).await
    }

    pub async fn remove_user_credential(
        &self,
        system_id: &str,
        user_name: &str,
    ) -> Result<models::RespBasic, Error<credentials_api::RemoveUserCredentialError>> {
        credentials_api::remove_user_credential(&self.config, system_id, user_name).await
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
        system_id: &str,
        user_name: &str,
    ) -> Result<models::RespNameArray, Error<permissions_api::GetUserPermsError>> {
        permissions_api::get_user_perms(&self.config, system_id, user_name).await
    }

    pub async fn grant_user_perms(
        &self,
        system_id: &str,
        user_name: &str,
        req_perms: models::ReqPerms,
    ) -> Result<models::RespBasic, Error<permissions_api::GrantUserPermsError>> {
        permissions_api::grant_user_perms(&self.config, system_id, user_name, req_perms).await
    }

    pub async fn revoke_user_perm(
        &self,
        system_id: &str,
        user_name: &str,
        permission: &str,
    ) -> Result<models::RespBasic, Error<permissions_api::RevokeUserPermError>> {
        permissions_api::revoke_user_perm(&self.config, system_id, user_name, permission).await
    }

    pub async fn revoke_user_perms(
        &self,
        system_id: &str,
        user_name: &str,
        req_perms: models::ReqPerms,
    ) -> Result<models::RespBasic, Error<permissions_api::RevokeUserPermsError>> {
        permissions_api::revoke_user_perms(&self.config, system_id, user_name, req_perms).await
    }
}

#[derive(Clone)]
pub struct SchedulerProfilesClient {
    config: Arc<configuration::Configuration>,
}

impl SchedulerProfilesClient {
    pub async fn create_scheduler_profile(
        &self,
        req_post_scheduler_profile: models::ReqPostSchedulerProfile,
    ) -> Result<models::RespResourceUrl, Error<scheduler_profiles_api::CreateSchedulerProfileError>>
    {
        scheduler_profiles_api::create_scheduler_profile(&self.config, req_post_scheduler_profile)
            .await
    }

    pub async fn delete_scheduler_profile(
        &self,
        name: &str,
    ) -> Result<models::RespChangeCount, Error<scheduler_profiles_api::DeleteSchedulerProfileError>>
    {
        scheduler_profiles_api::delete_scheduler_profile(&self.config, name).await
    }

    pub async fn get_scheduler_profile(
        &self,
        name: &str,
    ) -> Result<models::RespSchedulerProfile, Error<scheduler_profiles_api::GetSchedulerProfileError>>
    {
        scheduler_profiles_api::get_scheduler_profile(&self.config, name).await
    }

    pub async fn get_scheduler_profiles(
        &self,
    ) -> Result<
        models::RespSchedulerProfiles,
        Error<scheduler_profiles_api::GetSchedulerProfilesError>,
    > {
        scheduler_profiles_api::get_scheduler_profiles(&self.config).await
    }
}

#[derive(Clone)]
pub struct SharingClient {
    config: Arc<configuration::Configuration>,
}

impl SharingClient {
    pub async fn get_share_info(
        &self,
        system_id: &str,
    ) -> Result<models::RespShareInfo, Error<sharing_api::GetShareInfoError>> {
        sharing_api::get_share_info(&self.config, system_id).await
    }

    pub async fn share_system(
        &self,
        system_id: &str,
        req_share_update: models::ReqShareUpdate,
    ) -> Result<models::RespBasic, Error<sharing_api::ShareSystemError>> {
        sharing_api::share_system(&self.config, system_id, req_share_update).await
    }

    pub async fn share_system_public(
        &self,
        system_id: &str,
    ) -> Result<models::RespBasic, Error<sharing_api::ShareSystemPublicError>> {
        sharing_api::share_system_public(&self.config, system_id).await
    }

    pub async fn un_share_system(
        &self,
        system_id: &str,
        req_share_update: models::ReqShareUpdate,
    ) -> Result<models::RespBasic, Error<sharing_api::UnShareSystemError>> {
        sharing_api::un_share_system(&self.config, system_id, req_share_update).await
    }

    pub async fn un_share_system_public(
        &self,
        system_id: &str,
    ) -> Result<models::RespBasic, Error<sharing_api::UnShareSystemPublicError>> {
        sharing_api::un_share_system_public(&self.config, system_id).await
    }
}

#[derive(Clone)]
pub struct SystemsClient {
    config: Arc<configuration::Configuration>,
}

impl SystemsClient {
    pub async fn change_system_owner(
        &self,
        system_id: &str,
        user_name: &str,
    ) -> Result<models::RespChangeCount, Error<systems_api::ChangeSystemOwnerError>> {
        systems_api::change_system_owner(&self.config, system_id, user_name).await
    }

    pub async fn create_system(
        &self,
        req_post_system: models::ReqPostSystem,
        skip_credential_check: Option<bool>,
    ) -> Result<models::RespResourceUrl, Error<systems_api::CreateSystemError>> {
        systems_api::create_system(&self.config, req_post_system, skip_credential_check).await
    }

    pub async fn delete_system(
        &self,
        system_id: &str,
    ) -> Result<models::RespChangeCount, Error<systems_api::DeleteSystemError>> {
        systems_api::delete_system(&self.config, system_id).await
    }

    pub async fn disable_system(
        &self,
        system_id: &str,
    ) -> Result<models::RespChangeCount, Error<systems_api::DisableSystemError>> {
        systems_api::disable_system(&self.config, system_id).await
    }

    pub async fn enable_system(
        &self,
        system_id: &str,
    ) -> Result<models::RespChangeCount, Error<systems_api::EnableSystemError>> {
        systems_api::enable_system(&self.config, system_id).await
    }

    pub async fn get_history(
        &self,
        system_id: &str,
    ) -> Result<models::RespSystemHistory, Error<systems_api::GetHistoryError>> {
        systems_api::get_history(&self.config, system_id).await
    }

    pub async fn get_system(
        &self,
        system_id: &str,
        authn_method: Option<&str>,
        require_exec_perm: Option<bool>,
        select: Option<&str>,
        return_credentials: Option<bool>,
        impersonation_id: Option<&str>,
        shared_app_ctx: Option<&str>,
        resource_tenant: Option<&str>,
    ) -> Result<models::RespSystem, Error<systems_api::GetSystemError>> {
        systems_api::get_system(
            &self.config,
            system_id,
            authn_method,
            require_exec_perm,
            select,
            return_credentials,
            impersonation_id,
            shared_app_ctx,
            resource_tenant,
        )
        .await
    }

    pub async fn get_systems(
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
        has_credentials: Option<bool>,
    ) -> Result<models::RespSystems, Error<systems_api::GetSystemsError>> {
        systems_api::get_systems(
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
            has_credentials,
        )
        .await
    }

    pub async fn host_eval(
        &self,
        system_id: &str,
        env_var_name: &str,
    ) -> Result<models::RespName, Error<systems_api::HostEvalError>> {
        systems_api::host_eval(&self.config, system_id, env_var_name).await
    }

    pub async fn is_enabled(
        &self,
        system_id: &str,
    ) -> Result<models::RespBoolean, Error<systems_api::IsEnabledError>> {
        systems_api::is_enabled(&self.config, system_id).await
    }

    pub async fn match_constraints(
        &self,
        req_match_constraints: models::ReqMatchConstraints,
    ) -> Result<models::RespSystems, Error<systems_api::MatchConstraintsError>> {
        systems_api::match_constraints(&self.config, req_match_constraints).await
    }

    pub async fn patch_system(
        &self,
        system_id: &str,
        req_patch_system: models::ReqPatchSystem,
    ) -> Result<models::RespResourceUrl, Error<systems_api::PatchSystemError>> {
        systems_api::patch_system(&self.config, system_id, req_patch_system).await
    }

    pub async fn put_system(
        &self,
        system_id: &str,
        req_put_system: models::ReqPutSystem,
        skip_credential_check: Option<bool>,
    ) -> Result<models::RespResourceUrl, Error<systems_api::PutSystemError>> {
        systems_api::put_system(
            &self.config,
            system_id,
            req_put_system,
            skip_credential_check,
        )
        .await
    }

    pub async fn search_systems_query_parameters(
        &self,
        free_form_parameter_name: Option<std::collections::HashMap<String, String>>,
        list_type: Option<models::ListTypeEnum>,
        limit: Option<i32>,
        order_by: Option<&str>,
        skip: Option<i32>,
        start_after: Option<&str>,
        compute_total: Option<bool>,
        select: Option<&str>,
        has_credentials: Option<bool>,
    ) -> Result<models::RespSystems, Error<systems_api::SearchSystemsQueryParametersError>> {
        systems_api::search_systems_query_parameters(
            &self.config,
            free_form_parameter_name,
            list_type,
            limit,
            order_by,
            skip,
            start_after,
            compute_total,
            select,
            has_credentials,
        )
        .await
    }

    pub async fn search_systems_request_body(
        &self,
        req_search_systems: models::ReqSearchSystems,
        list_type: Option<models::ListTypeEnum>,
        limit: Option<i32>,
        order_by: Option<&str>,
        skip: Option<i32>,
        start_after: Option<&str>,
        compute_total: Option<bool>,
        select: Option<&str>,
        has_credentials: Option<bool>,
    ) -> Result<models::RespSystems, Error<systems_api::SearchSystemsRequestBodyError>> {
        systems_api::search_systems_request_body(
            &self.config,
            req_search_systems,
            list_type,
            limit,
            order_by,
            skip,
            start_after,
            compute_total,
            select,
            has_credentials,
        )
        .await
    }

    pub async fn undelete_system(
        &self,
        system_id: &str,
    ) -> Result<models::RespChangeCount, Error<systems_api::UndeleteSystemError>> {
        systems_api::undelete_system(&self.config, system_id).await
    }
}
