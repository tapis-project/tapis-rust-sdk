use crate::apis::{
    Error, admin_api, clients_api, configuration, health_check_api, metadata_api, profiles_api,
    tokens_api,
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
pub struct TapisAuthenticator {
    config: Arc<configuration::Configuration>,
    pub admin: AdminClient,
    pub clients: ClientsClient,
    pub health_check: HealthCheckClient,
    pub metadata: MetadataClient,
    pub profiles: ProfilesClient,
    pub tokens: TokensClient,
}

impl TapisAuthenticator {
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
            admin: AdminClient {
                config: config.clone(),
            },
            clients: ClientsClient {
                config: config.clone(),
            },
            health_check: HealthCheckClient {
                config: config.clone(),
            },
            metadata: MetadataClient {
                config: config.clone(),
            },
            profiles: ProfilesClient {
                config: config.clone(),
            },
            tokens: TokensClient {
                config: config.clone(),
            },
        })
    }

    pub fn config(&self) -> &configuration::Configuration {
        &self.config
    }
}

#[derive(Clone)]
pub struct AdminClient {
    config: Arc<configuration::Configuration>,
}

impl AdminClient {
    pub async fn get_config(
        &self,
    ) -> Result<models::GetConfig200Response, Error<admin_api::GetConfigError>> {
        admin_api::get_config(&self.config).await
    }

    pub async fn update_config(
        &self,
        new_tenant_config: models::NewTenantConfig,
    ) -> Result<models::GetConfig200Response, Error<admin_api::UpdateConfigError>> {
        admin_api::update_config(&self.config, new_tenant_config).await
    }
}

#[derive(Clone)]
pub struct ClientsClient {
    config: Arc<configuration::Configuration>,
}

impl ClientsClient {
    pub async fn create_client(
        &self,
        new_client: models::NewClient,
    ) -> Result<models::CreateClient201Response, Error<clients_api::CreateClientError>> {
        clients_api::create_client(&self.config, new_client).await
    }

    pub async fn delete_client(
        &self,
        client_id: &str,
    ) -> Result<models::DeleteClient200Response, Error<clients_api::DeleteClientError>> {
        clients_api::delete_client(&self.config, client_id).await
    }

    pub async fn get_client(
        &self,
        client_id: &str,
    ) -> Result<models::CreateClient201Response, Error<clients_api::GetClientError>> {
        clients_api::get_client(&self.config, client_id).await
    }

    pub async fn list_clients(
        &self,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<models::ListClients200Response, Error<clients_api::ListClientsError>> {
        clients_api::list_clients(&self.config, limit, offset).await
    }

    pub async fn update_client(
        &self,
        client_id: &str,
        update_client: models::UpdateClient,
    ) -> Result<models::CreateClient201Response, Error<clients_api::UpdateClientError>> {
        clients_api::update_client(&self.config, client_id, update_client).await
    }
}

#[derive(Clone)]
pub struct HealthCheckClient {
    config: Arc<configuration::Configuration>,
}

impl HealthCheckClient {
    pub async fn hello(
        &self,
    ) -> Result<models::BasicResponse, Error<health_check_api::HelloError>> {
        health_check_api::hello(&self.config).await
    }

    pub async fn ready(
        &self,
    ) -> Result<models::BasicResponse, Error<health_check_api::ReadyError>> {
        health_check_api::ready(&self.config).await
    }
}

#[derive(Clone)]
pub struct MetadataClient {
    config: Arc<configuration::Configuration>,
}

impl MetadataClient {
    pub async fn get_server_metadata(
        &self,
    ) -> Result<models::GetServerMetadata200Response, Error<metadata_api::GetServerMetadataError>>
    {
        metadata_api::get_server_metadata(&self.config).await
    }
}

#[derive(Clone)]
pub struct ProfilesClient {
    config: Arc<configuration::Configuration>,
}

impl ProfilesClient {
    pub async fn get_profile(
        &self,
        username: &str,
    ) -> Result<models::GetUserinfo200Response, Error<profiles_api::GetProfileError>> {
        profiles_api::get_profile(&self.config, username).await
    }

    pub async fn get_userinfo(
        &self,
    ) -> Result<models::GetUserinfo200Response, Error<profiles_api::GetUserinfoError>> {
        profiles_api::get_userinfo(&self.config).await
    }

    pub async fn list_profiles(
        &self,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<models::ListProfiles200Response, Error<profiles_api::ListProfilesError>> {
        profiles_api::list_profiles(&self.config, limit, offset).await
    }
}

#[derive(Clone)]
pub struct TokensClient {
    config: Arc<configuration::Configuration>,
}

impl TokensClient {
    pub async fn create_token(
        &self,
        new_token: models::NewToken,
    ) -> Result<models::CreateToken201Response, Error<tokens_api::CreateTokenError>> {
        tokens_api::create_token(&self.config, new_token).await
    }

    pub async fn create_v2_token(
        &self,
        v2_token: models::V2Token,
    ) -> Result<models::CreateV2Token200Response, Error<tokens_api::CreateV2TokenError>> {
        tokens_api::create_v2_token(&self.config, v2_token).await
    }

    pub async fn generate_device_code(
        &self,
        new_device_code: models::NewDeviceCode,
    ) -> Result<models::GenerateDeviceCode200Response, Error<tokens_api::GenerateDeviceCodeError>>
    {
        tokens_api::generate_device_code(&self.config, new_device_code).await
    }

    pub async fn revoke_token(
        &self,
        revoke_token_request: models::RevokeTokenRequest,
    ) -> Result<models::BasicResponse, Error<tokens_api::RevokeTokenError>> {
        tokens_api::revoke_token(&self.config, revoke_token_request).await
    }
}
