use crate::apis::{
    Error, actors_api, aliases_api, configuration, executions_api, messages_api, nonces_api,
    permissions_api, search_api, state_api, workers_api,
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
pub struct TapisActors {
    config: Arc<configuration::Configuration>,
    pub actors: ActorsClient,
    pub aliases: AliasesClient,
    pub executions: ExecutionsClient,
    pub messages: MessagesClient,
    pub nonces: NoncesClient,
    pub permissions: PermissionsClient,
    pub search: SearchClient,
    pub state: StateClient,
    pub workers: WorkersClient,
}

impl TapisActors {
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
            actors: ActorsClient {
                config: config.clone(),
            },
            aliases: AliasesClient {
                config: config.clone(),
            },
            executions: ExecutionsClient {
                config: config.clone(),
            },
            messages: MessagesClient {
                config: config.clone(),
            },
            nonces: NoncesClient {
                config: config.clone(),
            },
            permissions: PermissionsClient {
                config: config.clone(),
            },
            search: SearchClient {
                config: config.clone(),
            },
            state: StateClient {
                config: config.clone(),
            },
            workers: WorkersClient {
                config: config.clone(),
            },
        })
    }

    pub fn config(&self) -> &configuration::Configuration {
        &self.config
    }
}

#[derive(Clone)]
pub struct ActorsClient {
    config: Arc<configuration::Configuration>,
}

impl ActorsClient {
    pub async fn create_actor(
        &self,
        new_actor: models::NewActor,
    ) -> Result<models::CreateActor201Response, Error<actors_api::CreateActorError>> {
        actors_api::create_actor(&self.config, new_actor).await
    }

    pub async fn delete_actor(
        &self,
        actor_id: &str,
    ) -> Result<models::DeleteActor200Response, Error<actors_api::DeleteActorError>> {
        actors_api::delete_actor(&self.config, actor_id).await
    }

    pub async fn get_actor(
        &self,
        actor_id: &str,
    ) -> Result<models::GetActor200Response, Error<actors_api::GetActorError>> {
        actors_api::get_actor(&self.config, actor_id).await
    }

    pub async fn get_execution_result(
        &self,
        actor_id: &str,
        execution_id: &str,
    ) -> Result<reqwest::Response, Error<actors_api::GetExecutionResultError>> {
        actors_api::get_execution_result(&self.config, actor_id, execution_id).await
    }

    pub async fn list_actors(
        &self,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<models::ListActors200Response, Error<actors_api::ListActorsError>> {
        actors_api::list_actors(&self.config, limit, offset).await
    }

    pub async fn update_actor(
        &self,
        actor_id: &str,
        new_actor: models::NewActor,
    ) -> Result<models::GetActor200Response, Error<actors_api::UpdateActorError>> {
        actors_api::update_actor(&self.config, actor_id, new_actor).await
    }
}

#[derive(Clone)]
pub struct AliasesClient {
    config: Arc<configuration::Configuration>,
}

impl AliasesClient {
    pub async fn create_alias(
        &self,
        new_alias: models::NewAlias,
    ) -> Result<models::CreateAlias201Response, Error<aliases_api::CreateAliasError>> {
        aliases_api::create_alias(&self.config, new_alias).await
    }

    pub async fn delete_alias(
        &self,
        alias: &str,
    ) -> Result<models::DeleteActor200Response, Error<aliases_api::DeleteAliasError>> {
        aliases_api::delete_alias(&self.config, alias).await
    }

    pub async fn get_alias(
        &self,
        alias: &str,
    ) -> Result<models::CreateAlias201Response, Error<aliases_api::GetAliasError>> {
        aliases_api::get_alias(&self.config, alias).await
    }

    pub async fn list_aliases(
        &self,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<models::ListAliases200Response, Error<aliases_api::ListAliasesError>> {
        aliases_api::list_aliases(&self.config, limit, offset).await
    }

    pub async fn update_actor_alias(
        &self,
        alias: &str,
        new_alias: models::NewAlias,
    ) -> Result<models::CreateAlias201Response, Error<aliases_api::UpdateActorAliasError>> {
        aliases_api::update_actor_alias(&self.config, alias, new_alias).await
    }
}

#[derive(Clone)]
pub struct ExecutionsClient {
    config: Arc<configuration::Configuration>,
}

impl ExecutionsClient {
    pub async fn get_execution(
        &self,
        actor_id: &str,
        execution_id: &str,
    ) -> Result<models::GetExecution200Response, Error<executions_api::GetExecutionError>> {
        executions_api::get_execution(&self.config, actor_id, execution_id).await
    }

    pub async fn get_execution_logs(
        &self,
        actor_id: &str,
        execution_id: &str,
    ) -> Result<models::GetExecutionLogs200Response, Error<executions_api::GetExecutionLogsError>>
    {
        executions_api::get_execution_logs(&self.config, actor_id, execution_id).await
    }

    pub async fn get_execution_result(
        &self,
        actor_id: &str,
        execution_id: &str,
    ) -> Result<reqwest::Response, Error<executions_api::GetExecutionResultError>> {
        executions_api::get_execution_result(&self.config, actor_id, execution_id).await
    }

    pub async fn list_executions(
        &self,
        actor_id: &str,
    ) -> Result<models::ListExecutions200Response, Error<executions_api::ListExecutionsError>> {
        executions_api::list_executions(&self.config, actor_id).await
    }
}

#[derive(Clone)]
pub struct MessagesClient {
    config: Arc<configuration::Configuration>,
}

impl MessagesClient {
    pub async fn delete_pending_messages(
        &self,
        actor_id: &str,
    ) -> Result<models::DeleteActor200Response, Error<messages_api::DeletePendingMessagesError>>
    {
        messages_api::delete_pending_messages(&self.config, actor_id).await
    }

    pub async fn get_messages(
        &self,
        actor_id: &str,
    ) -> Result<models::GetMessages200Response, Error<messages_api::GetMessagesError>> {
        messages_api::get_messages(&self.config, actor_id).await
    }

    pub async fn send_binary_message(
        &self,
        actor_id: &str,
        binary_message: models::BinaryMessage,
        _abaco_synchronous: Option<&str>,
    ) -> Result<models::SendMessage200Response, Error<messages_api::SendBinaryMessageError>> {
        messages_api::send_binary_message(
            &self.config,
            actor_id,
            binary_message,
            _abaco_synchronous,
        )
        .await
    }

    pub async fn send_json_message(
        &self,
        actor_id: &str,
        json_message: models::JsonMessage,
        _abaco_synchronous: Option<&str>,
    ) -> Result<models::SendMessage200Response, Error<messages_api::SendJsonMessageError>> {
        messages_api::send_json_message(&self.config, actor_id, json_message, _abaco_synchronous)
            .await
    }

    pub async fn send_message(
        &self,
        actor_id: &str,
        json_message: models::JsonMessage,
        _abaco_synchronous: Option<&str>,
    ) -> Result<models::SendMessage200Response, Error<messages_api::SendMessageError>> {
        messages_api::send_message(&self.config, actor_id, json_message, _abaco_synchronous).await
    }
}

#[derive(Clone)]
pub struct NoncesClient {
    config: Arc<configuration::Configuration>,
}

impl NoncesClient {
    pub async fn create_nonce(
        &self,
        actor_id: &str,
        new_actor_nonce: models::NewActorNonce,
    ) -> Result<models::ListNonces200Response, Error<nonces_api::CreateNonceError>> {
        nonces_api::create_nonce(&self.config, actor_id, new_actor_nonce).await
    }

    pub async fn delete_nonce(
        &self,
        actor_id: &str,
        nonce_id: &str,
    ) -> Result<models::DeleteActor200Response, Error<nonces_api::DeleteNonceError>> {
        nonces_api::delete_nonce(&self.config, actor_id, nonce_id).await
    }

    pub async fn get_nonce(
        &self,
        actor_id: &str,
        nonce_id: &str,
    ) -> Result<models::GetNonce200Response, Error<nonces_api::GetNonceError>> {
        nonces_api::get_nonce(&self.config, actor_id, nonce_id).await
    }

    pub async fn list_nonces(
        &self,
        actor_id: &str,
    ) -> Result<models::ListNonces200Response, Error<nonces_api::ListNoncesError>> {
        nonces_api::list_nonces(&self.config, actor_id).await
    }
}

#[derive(Clone)]
pub struct PermissionsClient {
    config: Arc<configuration::Configuration>,
}

impl PermissionsClient {
    pub async fn list_permissions(
        &self,
        actor_id: &str,
    ) -> Result<models::ListPermissions200Response, Error<permissions_api::ListPermissionsError>>
    {
        permissions_api::list_permissions(&self.config, actor_id).await
    }

    pub async fn update_permissions(
        &self,
        actor_id: &str,
        actor_permission: models::ActorPermission,
    ) -> Result<models::ListPermissions200Response, Error<permissions_api::UpdatePermissionsError>>
    {
        permissions_api::update_permissions(&self.config, actor_id, actor_permission).await
    }
}

#[derive(Clone)]
pub struct SearchClient {
    config: Arc<configuration::Configuration>,
}

impl SearchClient {
    pub async fn search_database(
        &self,
        search_type: &str,
        search: Option<&str>,
        limit: Option<i32>,
        skip: Option<i32>,
    ) -> Result<models::BasicResponse, Error<search_api::SearchDatabaseError>> {
        search_api::search_database(&self.config, search_type, search, limit, skip).await
    }
}

#[derive(Clone)]
pub struct StateClient {
    config: Arc<configuration::Configuration>,
}

impl StateClient {
    pub async fn get_state(
        &self,
        actor_id: &str,
    ) -> Result<models::GetState200Response, Error<state_api::GetStateError>> {
        state_api::get_state(&self.config, actor_id).await
    }

    pub async fn update_state(
        &self,
        actor_id: &str,
        body: serde_json::Value,
    ) -> Result<models::ListNonces200Response, Error<state_api::UpdateStateError>> {
        state_api::update_state(&self.config, actor_id, body).await
    }
}

#[derive(Clone)]
pub struct WorkersClient {
    config: Arc<configuration::Configuration>,
}

impl WorkersClient {
    pub async fn list_workers(
        &self,
        actor_id: &str,
    ) -> Result<models::ActorWorkerResponse, Error<workers_api::ListWorkersError>> {
        workers_api::list_workers(&self.config, actor_id).await
    }

    pub async fn manage_worker_pool_size(
        &self,
        actor_id: &str,
        manage_worker_pool_size_request: models::ManageWorkerPoolSizeRequest,
    ) -> Result<models::SendMessage200Response, Error<workers_api::ManageWorkerPoolSizeError>> {
        workers_api::manage_worker_pool_size(
            &self.config,
            actor_id,
            manage_worker_pool_size_request,
        )
        .await
    }
}
