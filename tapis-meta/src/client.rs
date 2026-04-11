use crate::apis::{
    Error, aggregation_api, collection_api, configuration, db_api, document_api, general_api,
    index_api, root_api,
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
pub struct TapisMeta {
    config: Arc<configuration::Configuration>,
    pub aggregation: AggregationClient,
    pub collection: CollectionClient,
    pub db: DbClient,
    pub document: DocumentClient,
    pub general: GeneralClient,
    pub index: IndexClient,
    pub root: RootClient,
}

impl TapisMeta {
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
            aggregation: AggregationClient {
                config: config.clone(),
            },
            collection: CollectionClient {
                config: config.clone(),
            },
            db: DbClient {
                config: config.clone(),
            },
            document: DocumentClient {
                config: config.clone(),
            },
            general: GeneralClient {
                config: config.clone(),
            },
            index: IndexClient {
                config: config.clone(),
            },
            root: RootClient {
                config: config.clone(),
            },
        })
    }

    pub fn config(&self) -> &configuration::Configuration {
        &self.config
    }
}

#[derive(Clone)]
pub struct AggregationClient {
    config: Arc<configuration::Configuration>,
}

impl AggregationClient {
    pub async fn add_aggregation(
        &self,
        db: &str,
        collection: &str,
        aggregation: &str,
        body: Option<serde_json::Value>,
    ) -> Result<(), Error<aggregation_api::AddAggregationError>> {
        aggregation_api::add_aggregation(&self.config, db, collection, aggregation, body).await
    }

    pub async fn delete_aggregation(
        &self,
        db: &str,
        collection: &str,
        aggregation: &str,
    ) -> Result<(), Error<aggregation_api::DeleteAggregationError>> {
        aggregation_api::delete_aggregation(&self.config, db, collection, aggregation).await
    }

    pub async fn submit_large_aggregation(
        &self,
        db: &str,
        collection: &str,
        aggregation: &str,
        page: Option<i32>,
        pagesize: Option<i32>,
        keys: Option<Vec<String>>,
        body: Option<serde_json::Value>,
    ) -> Result<serde_json::Value, Error<aggregation_api::SubmitLargeAggregationError>> {
        aggregation_api::submit_large_aggregation(
            &self.config,
            db,
            collection,
            aggregation,
            page,
            pagesize,
            keys,
            body,
        )
        .await
    }

    pub async fn use_aggregation(
        &self,
        db: &str,
        collection: &str,
        aggregation: &str,
    ) -> Result<(), Error<aggregation_api::UseAggregationError>> {
        aggregation_api::use_aggregation(&self.config, db, collection, aggregation).await
    }
}

#[derive(Clone)]
pub struct CollectionClient {
    config: Arc<configuration::Configuration>,
}

impl CollectionClient {
    pub async fn create_collection(
        &self,
        db: &str,
        collection: &str,
    ) -> Result<(), Error<collection_api::CreateCollectionError>> {
        collection_api::create_collection(&self.config, db, collection).await
    }

    pub async fn delete_collection(
        &self,
        if_match: &str,
        db: &str,
        collection: &str,
    ) -> Result<(), Error<collection_api::DeleteCollectionError>> {
        collection_api::delete_collection(&self.config, if_match, db, collection).await
    }

    pub async fn get_collection_metadata(
        &self,
        db: &str,
        collection: &str,
    ) -> Result<serde_json::Value, Error<collection_api::GetCollectionMetadataError>> {
        collection_api::get_collection_metadata(&self.config, db, collection).await
    }

    pub async fn get_collection_size(
        &self,
        db: &str,
        collection: &str,
    ) -> Result<String, Error<collection_api::GetCollectionSizeError>> {
        collection_api::get_collection_size(&self.config, db, collection).await
    }

    pub async fn list_documents(
        &self,
        db: &str,
        collection: &str,
        page: Option<i32>,
        pagesize: Option<i32>,
        filter: Option<serde_json::Value>,
        sort: Option<serde_json::Value>,
        keys: Option<Vec<String>>,
    ) -> Result<Vec<serde_json::Value>, Error<collection_api::ListDocumentsError>> {
        collection_api::list_documents(
            &self.config,
            db,
            collection,
            page,
            pagesize,
            filter,
            sort,
            keys,
        )
        .await
    }

    pub async fn submit_large_query(
        &self,
        db: &str,
        collection: &str,
        page: Option<i32>,
        pagesize: Option<i32>,
        sort: Option<serde_json::Value>,
        keys: Option<Vec<String>>,
        body: Option<serde_json::Value>,
    ) -> Result<Vec<serde_json::Value>, Error<collection_api::SubmitLargeQueryError>> {
        collection_api::submit_large_query(
            &self.config,
            db,
            collection,
            page,
            pagesize,
            sort,
            keys,
            body,
        )
        .await
    }
}

#[derive(Clone)]
pub struct DbClient {
    config: Arc<configuration::Configuration>,
}

impl DbClient {
    pub async fn create_db(&self, db: &str) -> Result<(), Error<db_api::CreateDbError>> {
        db_api::create_db(&self.config, db).await
    }

    pub async fn delete_db(
        &self,
        if_match: &str,
        db: &str,
    ) -> Result<(), Error<db_api::DeleteDbError>> {
        db_api::delete_db(&self.config, if_match, db).await
    }

    pub async fn get_db_metadata(
        &self,
        db: &str,
    ) -> Result<serde_json::Value, Error<db_api::GetDbMetadataError>> {
        db_api::get_db_metadata(&self.config, db).await
    }

    pub async fn list_collection_names(
        &self,
        db: &str,
    ) -> Result<Vec<String>, Error<db_api::ListCollectionNamesError>> {
        db_api::list_collection_names(&self.config, db).await
    }
}

#[derive(Clone)]
pub struct DocumentClient {
    config: Arc<configuration::Configuration>,
}

impl DocumentClient {
    pub async fn create_document(
        &self,
        db: &str,
        collection: &str,
        basic: Option<bool>,
        body: Option<serde_json::Value>,
    ) -> Result<serde_json::Value, Error<document_api::CreateDocumentError>> {
        document_api::create_document(&self.config, db, collection, basic, body).await
    }

    pub async fn delete_document(
        &self,
        db: &str,
        collection: &str,
        doc_id: &str,
    ) -> Result<(), Error<document_api::DeleteDocumentError>> {
        document_api::delete_document(&self.config, db, collection, doc_id).await
    }

    pub async fn get_document(
        &self,
        db: &str,
        collection: &str,
        doc_id: &str,
    ) -> Result<serde_json::Value, Error<document_api::GetDocumentError>> {
        document_api::get_document(&self.config, db, collection, doc_id).await
    }

    pub async fn modify_document(
        &self,
        db: &str,
        collection: &str,
        doc_id: &str,
        np: Option<bool>,
        body: Option<serde_json::Value>,
    ) -> Result<(), Error<document_api::ModifyDocumentError>> {
        document_api::modify_document(&self.config, db, collection, doc_id, np, body).await
    }

    pub async fn replace_document(
        &self,
        db: &str,
        collection: &str,
        doc_id: &str,
        body: Option<serde_json::Value>,
    ) -> Result<(), Error<document_api::ReplaceDocumentError>> {
        document_api::replace_document(&self.config, db, collection, doc_id, body).await
    }
}

#[derive(Clone)]
pub struct GeneralClient {
    config: Arc<configuration::Configuration>,
}

impl GeneralClient {
    pub async fn healthcheck(
        &self,
    ) -> Result<serde_json::Value, Error<general_api::HealthcheckError>> {
        general_api::healthcheck(&self.config).await
    }
}

#[derive(Clone)]
pub struct IndexClient {
    config: Arc<configuration::Configuration>,
}

impl IndexClient {
    pub async fn create_index(
        &self,
        db: &str,
        collection: &str,
        index_name: &str,
        body: Option<serde_json::Value>,
    ) -> Result<(), Error<index_api::CreateIndexError>> {
        index_api::create_index(&self.config, db, collection, index_name, body).await
    }

    pub async fn delete_index(
        &self,
        db: &str,
        collection: &str,
        index_name: &str,
    ) -> Result<(), Error<index_api::DeleteIndexError>> {
        index_api::delete_index(&self.config, db, collection, index_name).await
    }

    pub async fn list_indexes(
        &self,
        db: &str,
        collection: &str,
    ) -> Result<Vec<serde_json::Value>, Error<index_api::ListIndexesError>> {
        index_api::list_indexes(&self.config, db, collection).await
    }
}

#[derive(Clone)]
pub struct RootClient {
    config: Arc<configuration::Configuration>,
}

impl RootClient {
    pub async fn list_db_names(&self) -> Result<Vec<String>, Error<root_api::ListDbNamesError>> {
        root_api::list_db_names(&self.config).await
    }
}
