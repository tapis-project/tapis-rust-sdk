use crate::apis::{
    Error, configuration, content_api, file_operations_api, general_api, permissions_api,
    post_its_api, sharing_api, transfers_api,
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
            if needs_refresh && let Some(new_token) = self.token_provider.get_token().await {
                let value = HeaderValue::from_str(&new_token)
                    .map_err(|e| reqwest_middleware::Error::Middleware(anyhow::anyhow!(e)))?;
                req.headers_mut().insert("x-tapis-token", value);
            }
        }
        next.run(req, extensions).await
    }
}

#[derive(Clone)]
pub struct TapisFiles {
    config: Arc<configuration::Configuration>,
    pub content: ContentClient,
    pub file_operations: FileOperationsClient,
    pub general: GeneralClient,
    pub permissions: PermissionsClient,
    pub post_its: PostItsClient,
    pub sharing: SharingClient,
    pub transfers: TransfersClient,
}

impl TapisFiles {
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
            content: ContentClient {
                config: config.clone(),
            },
            file_operations: FileOperationsClient {
                config: config.clone(),
            },
            general: GeneralClient {
                config: config.clone(),
            },
            permissions: PermissionsClient {
                config: config.clone(),
            },
            post_its: PostItsClient {
                config: config.clone(),
            },
            sharing: SharingClient {
                config: config.clone(),
            },
            transfers: TransfersClient {
                config: config.clone(),
            },
        })
    }

    pub fn config(&self) -> &configuration::Configuration {
        &self.config
    }
}

#[derive(Clone)]
pub struct ContentClient {
    config: Arc<configuration::Configuration>,
}

impl ContentClient {
    pub async fn get_contents(
        &self,
        system_id: &str,
        path: &str,
        range: Option<models::HeaderByteRange>,
        zip: Option<bool>,
        more: Option<i64>,
        impersonation_id: Option<&str>,
        shared_ctx: Option<&str>,
    ) -> Result<(), Error<content_api::GetContentsError>> {
        content_api::get_contents(
            &self.config,
            system_id,
            path,
            range,
            zip,
            more,
            impersonation_id,
            shared_ctx,
        )
        .await
    }
}

#[derive(Clone)]
pub struct FileOperationsClient {
    config: Arc<configuration::Configuration>,
}

impl FileOperationsClient {
    pub async fn delete(
        &self,
        system_id: &str,
        path: &str,
    ) -> Result<models::FileStringResponse, Error<file_operations_api::DeleteError>> {
        file_operations_api::delete(&self.config, system_id, path).await
    }

    pub async fn get_facl(
        &self,
        system_id: &str,
        path: &str,
    ) -> Result<models::NativeLinuxGetFaclResponse, Error<file_operations_api::GetFaclError>> {
        file_operations_api::get_facl(&self.config, system_id, path).await
    }

    pub async fn get_stat_info(
        &self,
        system_id: &str,
        path: &str,
        follow_links: Option<bool>,
    ) -> Result<models::FileStatInfoResponse, Error<file_operations_api::GetStatInfoError>> {
        file_operations_api::get_stat_info(&self.config, system_id, path, follow_links).await
    }

    pub async fn insert(
        &self,
        system_id: &str,
        path: &str,
        file: std::path::PathBuf,
    ) -> Result<models::FileStringResponse, Error<file_operations_api::InsertError>> {
        file_operations_api::insert(&self.config, system_id, path, file).await
    }

    pub async fn list_files(
        &self,
        system_id: &str,
        path: &str,
        pattern: Option<&str>,
        limit: Option<i32>,
        offset: Option<i64>,
        recurse: Option<bool>,
        impersonation_id: Option<&str>,
        shared_ctx: Option<&str>,
    ) -> Result<models::FileListingResponse, Error<file_operations_api::ListFilesError>> {
        file_operations_api::list_files(
            &self.config,
            system_id,
            path,
            pattern,
            limit,
            offset,
            recurse,
            impersonation_id,
            shared_ctx,
        )
        .await
    }

    pub async fn mkdir(
        &self,
        system_id: &str,
        shared_ctx: Option<&str>,
        mkdir_request: Option<models::MkdirRequest>,
    ) -> Result<models::FileStringResponse, Error<file_operations_api::MkdirError>> {
        file_operations_api::mkdir(&self.config, system_id, shared_ctx, mkdir_request).await
    }

    pub async fn move_copy(
        &self,
        system_id: &str,
        path: &str,
        move_copy_request: Option<models::MoveCopyRequest>,
    ) -> Result<models::FileStringResponse, Error<file_operations_api::MoveCopyError>> {
        file_operations_api::move_copy(&self.config, system_id, path, move_copy_request).await
    }

    pub async fn run_linux_native_op(
        &self,
        system_id: &str,
        path: &str,
        recursive: Option<bool>,
        native_linux_op_request: Option<models::NativeLinuxOpRequest>,
    ) -> Result<
        models::NativeLinuxOpResultResponse,
        Error<file_operations_api::RunLinuxNativeOpError>,
    > {
        file_operations_api::run_linux_native_op(
            &self.config,
            system_id,
            path,
            recursive,
            native_linux_op_request,
        )
        .await
    }

    pub async fn set_facl(
        &self,
        system_id: &str,
        path: &str,
        native_linux_set_facl_request: models::NativeLinuxSetFaclRequest,
    ) -> Result<models::NativeLinuxSetFaclResponse, Error<file_operations_api::SetFaclError>> {
        file_operations_api::set_facl(&self.config, system_id, path, native_linux_set_facl_request)
            .await
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
    pub async fn delete_permissions(
        &self,
        system_id: &str,
        path: &str,
        username: &str,
    ) -> Result<models::StringResponse, Error<permissions_api::DeletePermissionsError>> {
        permissions_api::delete_permissions(&self.config, system_id, path, username).await
    }

    pub async fn get_permissions(
        &self,
        system_id: &str,
        path: &str,
        username: Option<&str>,
    ) -> Result<models::FilePermissionResponse, Error<permissions_api::GetPermissionsError>> {
        permissions_api::get_permissions(&self.config, system_id, path, username).await
    }

    pub async fn grant_permissions(
        &self,
        system_id: &str,
        path: &str,
        create_permission_request: models::CreatePermissionRequest,
    ) -> Result<models::FilePermissionResponse, Error<permissions_api::GrantPermissionsError>> {
        permissions_api::grant_permissions(&self.config, system_id, path, create_permission_request)
            .await
    }
}

#[derive(Clone)]
pub struct PostItsClient {
    config: Arc<configuration::Configuration>,
}

impl PostItsClient {
    pub async fn create_post_it(
        &self,
        system_id: &str,
        path: &str,
        create_post_it_request: models::CreatePostItRequest,
    ) -> Result<models::PostItResponse, Error<post_its_api::CreatePostItError>> {
        post_its_api::create_post_it(&self.config, system_id, path, create_post_it_request).await
    }

    pub async fn delete_post_it(
        &self,
        postit_id: &str,
    ) -> Result<models::RespChangeCount, Error<post_its_api::DeletePostItError>> {
        post_its_api::delete_post_it(&self.config, postit_id).await
    }

    pub async fn get_post_it(
        &self,
        postit_id: &str,
    ) -> Result<models::PostItResponse, Error<post_its_api::GetPostItError>> {
        post_its_api::get_post_it(&self.config, postit_id).await
    }

    pub async fn list_post_its(
        &self,
        list_type: Option<models::ListTypeEnum>,
        limit: Option<i32>,
        order_by: Option<&str>,
        skip: Option<i32>,
        start_after: Option<&str>,
        select: Option<&str>,
    ) -> Result<models::PostItListResponse, Error<post_its_api::ListPostItsError>> {
        post_its_api::list_post_its(
            &self.config,
            list_type,
            limit,
            order_by,
            skip,
            start_after,
            select,
        )
        .await
    }

    pub async fn redeem_post_it(
        &self,
        postit_id: &str,
        zip: Option<bool>,
        download: Option<bool>,
    ) -> Result<(), Error<post_its_api::RedeemPostItError>> {
        post_its_api::redeem_post_it(&self.config, postit_id, zip, download).await
    }

    pub async fn update_post_it(
        &self,
        postit_id: &str,
        update_post_it_request: models::UpdatePostItRequest,
    ) -> Result<models::PostItResponse, Error<post_its_api::UpdatePostItError>> {
        post_its_api::update_post_it(&self.config, postit_id, update_post_it_request).await
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
        path: &str,
    ) -> Result<models::RespShareInfo, Error<sharing_api::GetShareInfoError>> {
        sharing_api::get_share_info(&self.config, system_id, path).await
    }

    pub async fn share_path(
        &self,
        system_id: &str,
        path: &str,
        req_share_update: models::ReqShareUpdate,
    ) -> Result<models::RespBasic, Error<sharing_api::SharePathError>> {
        sharing_api::share_path(&self.config, system_id, path, req_share_update).await
    }

    pub async fn share_path_public(
        &self,
        system_id: &str,
        path: &str,
    ) -> Result<models::RespBasic, Error<sharing_api::SharePathPublicError>> {
        sharing_api::share_path_public(&self.config, system_id, path).await
    }

    pub async fn un_share_path(
        &self,
        system_id: &str,
        path: &str,
        req_share_update: models::ReqShareUpdate,
    ) -> Result<models::RespBasic, Error<sharing_api::UnSharePathError>> {
        sharing_api::un_share_path(&self.config, system_id, path, req_share_update).await
    }

    pub async fn un_share_path_all(
        &self,
        system_id: &str,
        path: &str,
        recurse: Option<bool>,
    ) -> Result<models::RespBasic, Error<sharing_api::UnSharePathAllError>> {
        sharing_api::un_share_path_all(&self.config, system_id, path, recurse).await
    }

    pub async fn un_share_path_public(
        &self,
        system_id: &str,
        path: &str,
    ) -> Result<models::RespBasic, Error<sharing_api::UnSharePathPublicError>> {
        sharing_api::un_share_path_public(&self.config, system_id, path).await
    }
}

#[derive(Clone)]
pub struct TransfersClient {
    config: Arc<configuration::Configuration>,
}

impl TransfersClient {
    pub async fn cancel_transfer_task(
        &self,
        transfer_task_id: &str,
    ) -> Result<models::StringResponse, Error<transfers_api::CancelTransferTaskError>> {
        transfers_api::cancel_transfer_task(&self.config, transfer_task_id).await
    }

    pub async fn create_transfer_task(
        &self,
        req_transfer: models::ReqTransfer,
    ) -> Result<models::TransferTaskResponse, Error<transfers_api::CreateTransferTaskError>> {
        transfers_api::create_transfer_task(&self.config, req_transfer).await
    }

    pub async fn get_recent_transfer_tasks(
        &self,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<models::TransferTaskListResponse, Error<transfers_api::GetRecentTransferTasksError>>
    {
        transfers_api::get_recent_transfer_tasks(&self.config, limit, offset).await
    }

    pub async fn get_transfer_task(
        &self,
        transfer_task_id: &str,
        include_summary: Option<bool>,
        impersonation_id: Option<&str>,
    ) -> Result<models::TransferTaskResponse, Error<transfers_api::GetTransferTaskError>> {
        transfers_api::get_transfer_task(
            &self.config,
            transfer_task_id,
            include_summary,
            impersonation_id,
        )
        .await
    }

    pub async fn get_transfer_task_details(
        &self,
        transfer_task_id: &str,
        impersonation_id: Option<&str>,
    ) -> Result<models::TransferTaskResponse, Error<transfers_api::GetTransferTaskDetailsError>>
    {
        transfers_api::get_transfer_task_details(&self.config, transfer_task_id, impersonation_id)
            .await
    }
}
