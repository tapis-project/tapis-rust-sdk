use crate::apis::{Error, configuration, general_api, jobs_api, sharing_api, subscriptions_api};
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
pub struct TapisJobs {
    config: Arc<configuration::Configuration>,
    pub general: GeneralClient,
    pub jobs: JobsClient,
    pub sharing: SharingClient,
    pub subscriptions: SubscriptionsClient,
}

impl TapisJobs {
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
            general: GeneralClient {
                config: config.clone(),
            },
            jobs: JobsClient {
                config: config.clone(),
            },
            sharing: SharingClient {
                config: config.clone(),
            },
            subscriptions: SubscriptionsClient {
                config: config.clone(),
            },
        })
    }

    pub fn config(&self) -> &configuration::Configuration {
        &self.config
    }
}

#[derive(Clone)]
pub struct GeneralClient {
    config: Arc<configuration::Configuration>,
}

impl GeneralClient {
    pub async fn check_health(
        &self,
    ) -> Result<models::RespProbe, Error<general_api::CheckHealthError>> {
        general_api::check_health(&self.config).await
    }

    pub async fn readycheck(
        &self,
    ) -> Result<models::RespProbe, Error<general_api::ReadycheckError>> {
        general_api::readycheck(&self.config).await
    }
}

#[derive(Clone)]
pub struct JobsClient {
    config: Arc<configuration::Configuration>,
}

impl JobsClient {
    pub async fn cancel_job(
        &self,
        job_uuid: &str,
        body: Option<serde_json::Value>,
    ) -> Result<models::RespCancelJob, Error<jobs_api::CancelJobError>> {
        jobs_api::cancel_job(&self.config, job_uuid, body).await
    }

    pub async fn get_job(
        &self,
        job_uuid: &str,
    ) -> Result<models::RespGetJob, Error<jobs_api::GetJobError>> {
        jobs_api::get_job(&self.config, job_uuid).await
    }

    pub async fn get_job_history(
        &self,
        job_uuid: &str,
        limit: Option<i32>,
        skip: Option<i32>,
    ) -> Result<models::RespJobHistory, Error<jobs_api::GetJobHistoryError>> {
        jobs_api::get_job_history(&self.config, job_uuid, limit, skip).await
    }

    pub async fn get_job_list(
        &self,
        limit: Option<i32>,
        skip: Option<i32>,
        start_after: Option<i32>,
        order_by: Option<&str>,
        compute_total: Option<bool>,
        list_type: Option<&str>,
    ) -> Result<models::RespGetJobList, Error<jobs_api::GetJobListError>> {
        jobs_api::get_job_list(
            &self.config,
            limit,
            skip,
            start_after,
            order_by,
            compute_total,
            list_type,
        )
        .await
    }

    pub async fn get_job_output_download(
        &self,
        job_uuid: &str,
        output_path: &str,
        compress: Option<bool>,
        format: Option<&str>,
        allow_if_running: Option<bool>,
    ) -> Result<reqwest::Response, Error<jobs_api::GetJobOutputDownloadError>> {
        jobs_api::get_job_output_download(
            &self.config,
            job_uuid,
            output_path,
            compress,
            format,
            allow_if_running,
        )
        .await
    }

    pub async fn get_job_output_list(
        &self,
        job_uuid: &str,
        output_path: &str,
        limit: Option<i32>,
        skip: Option<i32>,
        allow_if_running: Option<bool>,
    ) -> Result<models::RespGetJobOutputList, Error<jobs_api::GetJobOutputListError>> {
        jobs_api::get_job_output_list(
            &self.config,
            job_uuid,
            output_path,
            limit,
            skip,
            allow_if_running,
        )
        .await
    }

    pub async fn get_job_search_list(
        &self,
        limit: Option<i32>,
        skip: Option<i32>,
        start_after: Option<i32>,
        order_by: Option<&str>,
        compute_total: Option<bool>,
        select: Option<&str>,
        list_type: Option<&str>,
    ) -> Result<models::RespJobSearchAllAttributes, Error<jobs_api::GetJobSearchListError>> {
        jobs_api::get_job_search_list(
            &self.config,
            limit,
            skip,
            start_after,
            order_by,
            compute_total,
            select,
            list_type,
        )
        .await
    }

    pub async fn get_job_search_list_by_post_sql_str(
        &self,
        limit: Option<i32>,
        skip: Option<i32>,
        start_after: Option<i32>,
        order_by: Option<&str>,
        compute_total: Option<bool>,
        select: Option<&str>,
        list_type: Option<&str>,
        body: Option<serde_json::Value>,
    ) -> Result<
        models::RespJobSearchAllAttributes,
        Error<jobs_api::GetJobSearchListByPostSqlStrError>,
    > {
        jobs_api::get_job_search_list_by_post_sql_str(
            &self.config,
            limit,
            skip,
            start_after,
            order_by,
            compute_total,
            select,
            list_type,
            body,
        )
        .await
    }

    pub async fn get_job_status(
        &self,
        job_uuid: &str,
    ) -> Result<models::RespGetJobStatus, Error<jobs_api::GetJobStatusError>> {
        jobs_api::get_job_status(&self.config, job_uuid).await
    }

    pub async fn get_resubmit_request_json(
        &self,
        job_uuid: &str,
    ) -> Result<models::RespGetResubmit, Error<jobs_api::GetResubmitRequestJsonError>> {
        jobs_api::get_resubmit_request_json(&self.config, job_uuid).await
    }

    pub async fn hide_job(
        &self,
        job_uuid: &str,
        body: Option<serde_json::Value>,
    ) -> Result<models::RespHideJob, Error<jobs_api::HideJobError>> {
        jobs_api::hide_job(&self.config, job_uuid, body).await
    }

    pub async fn patch_job_annotations(
        &self,
        job_uuid: &str,
        req_job_annotation: models::ReqJobAnnotation,
    ) -> Result<models::RespJobAnnotations, Error<jobs_api::PatchJobAnnotationsError>> {
        jobs_api::patch_job_annotations(&self.config, job_uuid, req_job_annotation).await
    }

    pub async fn put_job_annotations(
        &self,
        job_uuid: &str,
        req_job_annotation: models::ReqJobAnnotation,
    ) -> Result<models::RespJobAnnotations, Error<jobs_api::PutJobAnnotationsError>> {
        jobs_api::put_job_annotations(&self.config, job_uuid, req_job_annotation).await
    }

    pub async fn resubmit_job(
        &self,
        job_uuid: &str,
        body: Option<serde_json::Value>,
    ) -> Result<models::RespSubmitJob, Error<jobs_api::ResubmitJobError>> {
        jobs_api::resubmit_job(&self.config, job_uuid, body).await
    }

    pub async fn send_event(
        &self,
        job_uuid: &str,
        req_user_event: models::ReqUserEvent,
    ) -> Result<models::RespBasic, Error<jobs_api::SendEventError>> {
        jobs_api::send_event(&self.config, job_uuid, req_user_event).await
    }

    pub async fn submit_job(
        &self,
        req_submit_job: models::ReqSubmitJob,
    ) -> Result<models::RespSubmitJob, Error<jobs_api::SubmitJobError>> {
        jobs_api::submit_job(&self.config, req_submit_job).await
    }

    pub async fn unhide_job(
        &self,
        job_uuid: &str,
        body: Option<serde_json::Value>,
    ) -> Result<models::RespHideJob, Error<jobs_api::UnhideJobError>> {
        jobs_api::unhide_job(&self.config, job_uuid, body).await
    }
}

#[derive(Clone)]
pub struct SharingClient {
    config: Arc<configuration::Configuration>,
}

impl SharingClient {
    pub async fn delete_job_share(
        &self,
        job_uuid: &str,
        user: &str,
    ) -> Result<models::RespUnShareJob, Error<sharing_api::DeleteJobShareError>> {
        sharing_api::delete_job_share(&self.config, job_uuid, user).await
    }

    pub async fn get_job_share(
        &self,
        job_uuid: &str,
        limit: Option<i32>,
        skip: Option<i32>,
    ) -> Result<models::RespGetJobShareList, Error<sharing_api::GetJobShareError>> {
        sharing_api::get_job_share(&self.config, job_uuid, limit, skip).await
    }

    pub async fn share_job(
        &self,
        job_uuid: &str,
        req_share_job: models::ReqShareJob,
    ) -> Result<models::RespShareJob, Error<sharing_api::ShareJobError>> {
        sharing_api::share_job(&self.config, job_uuid, req_share_job).await
    }
}

#[derive(Clone)]
pub struct SubscriptionsClient {
    config: Arc<configuration::Configuration>,
}

impl SubscriptionsClient {
    pub async fn delete_subscriptions(
        &self,
        uuid: &str,
    ) -> Result<models::ResultChangeCount, Error<subscriptions_api::DeleteSubscriptionsError>> {
        subscriptions_api::delete_subscriptions(&self.config, uuid).await
    }

    pub async fn get_subscriptions(
        &self,
        job_uuid: &str,
        limit: Option<i32>,
        skip: Option<i32>,
    ) -> Result<models::RespGetSubscriptions, Error<subscriptions_api::GetSubscriptionsError>> {
        subscriptions_api::get_subscriptions(&self.config, job_uuid, limit, skip).await
    }

    pub async fn subscribe(
        &self,
        job_uuid: &str,
        req_subscribe: models::ReqSubscribe,
    ) -> Result<models::RespResourceUrl, Error<subscriptions_api::SubscribeError>> {
        subscriptions_api::subscribe(&self.config, job_uuid, req_subscribe).await
    }
}
