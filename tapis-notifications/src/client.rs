use crate::apis::{Error, configuration, events_api, general_api, subscriptions_api, test_api};
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
pub struct TapisNotifications {
    config: Arc<configuration::Configuration>,
    pub events: EventsClient,
    pub general: GeneralClient,
    pub subscriptions: SubscriptionsClient,
    pub test: TestClient,
}

impl TapisNotifications {
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
            events: EventsClient {
                config: config.clone(),
            },
            general: GeneralClient {
                config: config.clone(),
            },
            subscriptions: SubscriptionsClient {
                config: config.clone(),
            },
            test: TestClient {
                config: config.clone(),
            },
        })
    }

    pub fn config(&self) -> &configuration::Configuration {
        &self.config
    }
}

#[derive(Clone)]
pub struct EventsClient {
    config: Arc<configuration::Configuration>,
}

impl EventsClient {
    pub async fn end_event_series(
        &self,
        event_series: models::EventSeries,
        tenant: Option<&str>,
    ) -> Result<models::RespBasic, Error<events_api::EndEventSeriesError>> {
        events_api::end_event_series(&self.config, event_series, tenant).await
    }

    pub async fn post_event(
        &self,
        event: models::Event,
        tenant: Option<&str>,
    ) -> Result<models::RespBasic, Error<events_api::PostEventError>> {
        events_api::post_event(&self.config, event, tenant).await
    }

    pub async fn publish_event(
        &self,
        event: models::Event,
        tenant: Option<&str>,
    ) -> Result<models::RespBasic, Error<events_api::PublishEventError>> {
        events_api::publish_event(&self.config, event, tenant).await
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
pub struct SubscriptionsClient {
    config: Arc<configuration::Configuration>,
}

impl SubscriptionsClient {
    pub async fn delete_subscription_by_name(
        &self,
        name: &str,
        owned_by: Option<&str>,
    ) -> Result<models::RespChangeCount, Error<subscriptions_api::DeleteSubscriptionByNameError>>
    {
        subscriptions_api::delete_subscription_by_name(&self.config, name, owned_by).await
    }

    pub async fn delete_subscription_by_uuid(
        &self,
        uuid: &str,
    ) -> Result<models::RespChangeCount, Error<subscriptions_api::DeleteSubscriptionByUuidError>>
    {
        subscriptions_api::delete_subscription_by_uuid(&self.config, uuid).await
    }

    pub async fn delete_subscriptions_by_subject(
        &self,
        subject: &str,
        owned_by: Option<&str>,
        any_owner: Option<bool>,
    ) -> Result<models::RespChangeCount, Error<subscriptions_api::DeleteSubscriptionsBySubjectError>>
    {
        subscriptions_api::delete_subscriptions_by_subject(
            &self.config,
            subject,
            owned_by,
            any_owner,
        )
        .await
    }

    pub async fn disable_subscription(
        &self,
        name: &str,
        owned_by: Option<&str>,
    ) -> Result<models::RespChangeCount, Error<subscriptions_api::DisableSubscriptionError>> {
        subscriptions_api::disable_subscription(&self.config, name, owned_by).await
    }

    pub async fn enable_subscription(
        &self,
        name: &str,
        owned_by: Option<&str>,
    ) -> Result<models::RespChangeCount, Error<subscriptions_api::EnableSubscriptionError>> {
        subscriptions_api::enable_subscription(&self.config, name, owned_by).await
    }

    pub async fn get_subscription_by_name(
        &self,
        name: &str,
        select: Option<&str>,
        owned_by: Option<&str>,
    ) -> Result<models::RespSubscription, Error<subscriptions_api::GetSubscriptionByNameError>>
    {
        subscriptions_api::get_subscription_by_name(&self.config, name, select, owned_by).await
    }

    pub async fn get_subscription_by_uuid(
        &self,
        uuid: &str,
        select: Option<&str>,
    ) -> Result<models::RespSubscription, Error<subscriptions_api::GetSubscriptionByUuidError>>
    {
        subscriptions_api::get_subscription_by_uuid(&self.config, uuid, select).await
    }

    pub async fn get_subscriptions(
        &self,
        search: Option<&str>,
        limit: Option<i32>,
        order_by: Option<&str>,
        skip: Option<i32>,
        start_after: Option<&str>,
        compute_total: Option<bool>,
        select: Option<&str>,
        owned_by: Option<&str>,
        any_owner: Option<bool>,
    ) -> Result<models::RespSubscriptions, Error<subscriptions_api::GetSubscriptionsError>> {
        subscriptions_api::get_subscriptions(
            &self.config,
            search,
            limit,
            order_by,
            skip,
            start_after,
            compute_total,
            select,
            owned_by,
            any_owner,
        )
        .await
    }

    pub async fn is_enabled(
        &self,
        name: &str,
        owned_by: Option<&str>,
    ) -> Result<models::RespBoolean, Error<subscriptions_api::IsEnabledError>> {
        subscriptions_api::is_enabled(&self.config, name, owned_by).await
    }

    pub async fn patch_subscription_by_name(
        &self,
        name: &str,
        req_patch_subscription: models::ReqPatchSubscription,
        owned_by: Option<&str>,
    ) -> Result<models::RespResourceUrl, Error<subscriptions_api::PatchSubscriptionByNameError>>
    {
        subscriptions_api::patch_subscription_by_name(
            &self.config,
            name,
            req_patch_subscription,
            owned_by,
        )
        .await
    }

    pub async fn post_subscription(
        &self,
        req_post_subscription: models::ReqPostSubscription,
    ) -> Result<models::RespResourceUrl, Error<subscriptions_api::PostSubscriptionError>> {
        subscriptions_api::post_subscription(&self.config, req_post_subscription).await
    }

    pub async fn search_subscriptions_query_parameters(
        &self,
        free_form_parameter_name: Option<std::collections::HashMap<String, String>>,
        limit: Option<i32>,
        order_by: Option<&str>,
        skip: Option<i32>,
        start_after: Option<&str>,
        compute_total: Option<bool>,
        select: Option<&str>,
        owned_by: Option<&str>,
    ) -> Result<
        models::RespSubscriptions,
        Error<subscriptions_api::SearchSubscriptionsQueryParametersError>,
    > {
        subscriptions_api::search_subscriptions_query_parameters(
            &self.config,
            free_form_parameter_name,
            limit,
            order_by,
            skip,
            start_after,
            compute_total,
            select,
            owned_by,
        )
        .await
    }

    pub async fn search_subscriptions_request_body(
        &self,
        req_search_subscriptions: models::ReqSearchSubscriptions,
        limit: Option<i32>,
        order_by: Option<&str>,
        skip: Option<i32>,
        start_after: Option<&str>,
        compute_total: Option<bool>,
        select: Option<&str>,
        owned_by: Option<&str>,
    ) -> Result<
        models::RespSubscriptions,
        Error<subscriptions_api::SearchSubscriptionsRequestBodyError>,
    > {
        subscriptions_api::search_subscriptions_request_body(
            &self.config,
            req_search_subscriptions,
            limit,
            order_by,
            skip,
            start_after,
            compute_total,
            select,
            owned_by,
        )
        .await
    }

    pub async fn update_ttl(
        &self,
        name: &str,
        ttl_minutes: i32,
        owned_by: Option<&str>,
    ) -> Result<models::RespChangeCount, Error<subscriptions_api::UpdateTtlError>> {
        subscriptions_api::update_ttl(&self.config, name, ttl_minutes, owned_by).await
    }
}

#[derive(Clone)]
pub struct TestClient {
    config: Arc<configuration::Configuration>,
}

impl TestClient {
    pub async fn begin_test_sequence(
        &self,
        subscription_ttl: Option<i32>,
        number_of_events: Option<i32>,
        end_series: Option<bool>,
        body: Option<serde_json::Value>,
    ) -> Result<models::RespSubscription, Error<test_api::BeginTestSequenceError>> {
        test_api::begin_test_sequence(
            &self.config,
            subscription_ttl,
            number_of_events,
            end_series,
            body,
        )
        .await
    }

    pub async fn delete_test_sequence(
        &self,
        name: &str,
    ) -> Result<models::RespChangeCount, Error<test_api::DeleteTestSequenceError>> {
        test_api::delete_test_sequence(&self.config, name).await
    }

    pub async fn get_test_sequence(
        &self,
        name: &str,
    ) -> Result<models::RespTestSequence, Error<test_api::GetTestSequenceError>> {
        test_api::get_test_sequence(&self.config, name).await
    }

    pub async fn record_test_notification(
        &self,
        name: &str,
        notification: models::Notification,
    ) -> Result<models::RespBasic, Error<test_api::RecordTestNotificationError>> {
        test_api::record_test_notification(&self.config, name, notification).await
    }
}
