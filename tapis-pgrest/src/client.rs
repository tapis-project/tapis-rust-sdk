use crate::apis::{
    Error, configuration, manage_roles_api, manage_tables_api, manage_views_api, tables_api,
    views_api,
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
pub struct TapisPgrest {
    config: Arc<configuration::Configuration>,
    pub manage_roles: ManageRolesClient,
    pub manage_tables: ManageTablesClient,
    pub manage_views: ManageViewsClient,
    pub tables: TablesClient,
    pub views: ViewsClient,
}

impl TapisPgrest {
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
            manage_roles: ManageRolesClient {
                config: config.clone(),
            },
            manage_tables: ManageTablesClient {
                config: config.clone(),
            },
            manage_views: ManageViewsClient {
                config: config.clone(),
            },
            tables: TablesClient {
                config: config.clone(),
            },
            views: ViewsClient {
                config: config.clone(),
            },
        })
    }

    pub fn config(&self) -> &configuration::Configuration {
        &self.config
    }
}

#[derive(Clone)]
pub struct ManageRolesClient {
    config: Arc<configuration::Configuration>,
}

impl ManageRolesClient {
    pub async fn create_role(
        &self,
        new_role: models::NewRole,
    ) -> Result<models::CreateRole200Response, Error<manage_roles_api::CreateRoleError>> {
        manage_roles_api::create_role(&self.config, new_role).await
    }

    pub async fn get_role(
        &self,
        role_name: &str,
    ) -> Result<models::GetRole200Response, Error<manage_roles_api::GetRoleError>> {
        manage_roles_api::get_role(&self.config, role_name).await
    }

    pub async fn list_roles(
        &self,
    ) -> Result<models::ListRoles200Response, Error<manage_roles_api::ListRolesError>> {
        manage_roles_api::list_roles(&self.config).await
    }

    pub async fn manage_role(
        &self,
        role_name: &str,
        manage_role: models::ManageRole,
    ) -> Result<models::CreateRole200Response, Error<manage_roles_api::ManageRoleError>> {
        manage_roles_api::manage_role(&self.config, role_name, manage_role).await
    }
}

#[derive(Clone)]
pub struct ManageTablesClient {
    config: Arc<configuration::Configuration>,
}

impl ManageTablesClient {
    pub async fn create_table(
        &self,
        new_table: models::NewTable,
    ) -> Result<models::CreateTable201Response, Error<manage_tables_api::CreateTableError>> {
        manage_tables_api::create_table(&self.config, new_table).await
    }

    pub async fn delete_table(
        &self,
        table_id: &str,
    ) -> Result<models::BasicResponse, Error<manage_tables_api::DeleteTableError>> {
        manage_tables_api::delete_table(&self.config, table_id).await
    }

    pub async fn get_table(
        &self,
        table_id: &str,
        details: Option<bool>,
    ) -> Result<models::CreateTable201Response, Error<manage_tables_api::GetTableError>> {
        manage_tables_api::get_table(&self.config, table_id, details).await
    }

    pub async fn list_tables(
        &self,
    ) -> Result<models::ListTables200Response, Error<manage_tables_api::ListTablesError>> {
        manage_tables_api::list_tables(&self.config).await
    }

    pub async fn update_table(
        &self,
        table_id: &str,
        update_table: models::UpdateTable,
    ) -> Result<models::CreateTable201Response, Error<manage_tables_api::UpdateTableError>> {
        manage_tables_api::update_table(&self.config, table_id, update_table).await
    }
}

#[derive(Clone)]
pub struct ManageViewsClient {
    config: Arc<configuration::Configuration>,
}

impl ManageViewsClient {
    pub async fn create_view(
        &self,
        new_view: models::NewView,
    ) -> Result<models::CreateView201Response, Error<manage_views_api::CreateViewError>> {
        manage_views_api::create_view(&self.config, new_view).await
    }

    pub async fn delete_view(
        &self,
        view_name: &str,
    ) -> Result<models::BasicResponse, Error<manage_views_api::DeleteViewError>> {
        manage_views_api::delete_view(&self.config, view_name).await
    }

    pub async fn get_manage_view(
        &self,
        view_name: &str,
        details: Option<bool>,
    ) -> Result<models::CreateView201Response, Error<manage_views_api::GetManageViewError>> {
        manage_views_api::get_manage_view(&self.config, view_name, details).await
    }

    pub async fn list_views(
        &self,
    ) -> Result<models::ListViews200Response, Error<manage_views_api::ListViewsError>> {
        manage_views_api::list_views(&self.config).await
    }

    pub async fn refresh_materialized_view(
        &self,
        view_name: &str,
    ) -> Result<models::BasicResponse, Error<manage_views_api::RefreshMaterializedViewError>> {
        manage_views_api::refresh_materialized_view(&self.config, view_name).await
    }
}

#[derive(Clone)]
pub struct TablesClient {
    config: Arc<configuration::Configuration>,
}

impl TablesClient {
    pub async fn add_table_row(
        &self,
        root_url: &str,
        new_table_row: models::NewTableRow,
    ) -> Result<models::AddTableRow201Response, Error<tables_api::AddTableRowError>> {
        tables_api::add_table_row(&self.config, root_url, new_table_row).await
    }

    pub async fn add_table_rows(
        &self,
        root_url: &str,
        new_table_rows: models::NewTableRows,
    ) -> Result<models::GetTableRows200Response, Error<tables_api::AddTableRowsError>> {
        tables_api::add_table_rows(&self.config, root_url, new_table_rows).await
    }

    pub async fn delete_table_row(
        &self,
        root_url: &str,
        item: &str,
    ) -> Result<models::BasicResponse, Error<tables_api::DeleteTableRowError>> {
        tables_api::delete_table_row(&self.config, root_url, item).await
    }

    pub async fn get_table_row(
        &self,
        root_url: &str,
        item: &str,
    ) -> Result<models::AddTableRow201Response, Error<tables_api::GetTableRowError>> {
        tables_api::get_table_row(&self.config, root_url, item).await
    }

    pub async fn get_table_rows(
        &self,
        root_url: &str,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<models::GetTableRows200Response, Error<tables_api::GetTableRowsError>> {
        tables_api::get_table_rows(&self.config, root_url, limit, offset).await
    }

    pub async fn update_table_row(
        &self,
        root_url: &str,
        item: &str,
        body: serde_json::Value,
    ) -> Result<models::AddTableRow201Response, Error<tables_api::UpdateTableRowError>> {
        tables_api::update_table_row(&self.config, root_url, item, body).await
    }

    pub async fn update_table_rows(
        &self,
        root_url: &str,
        update_multiple_table_rows: models::UpdateMultipleTableRows,
    ) -> Result<models::BasicResponse, Error<tables_api::UpdateTableRowsError>> {
        tables_api::update_table_rows(&self.config, root_url, update_multiple_table_rows).await
    }
}

#[derive(Clone)]
pub struct ViewsClient {
    config: Arc<configuration::Configuration>,
}

impl ViewsClient {
    pub async fn get_view(
        &self,
        view_name: &str,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<models::GetTableRows200Response, Error<views_api::GetViewError>> {
        views_api::get_view(&self.config, view_name, limit, offset).await
    }
}
