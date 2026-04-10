use crate::apis::{configuration, ldaps_api, owners_api, sites_api, tenants_api, Error};
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
pub struct TapisTenants {
    config: Arc<configuration::Configuration>,
    pub ldaps: LdapsClient,
    pub owners: OwnersClient,
    pub sites: SitesClient,
    pub tenants: TenantsClient,
}

impl TapisTenants {
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
            ldaps: LdapsClient {
                config: config.clone(),
            },
            owners: OwnersClient {
                config: config.clone(),
            },
            sites: SitesClient {
                config: config.clone(),
            },
            tenants: TenantsClient {
                config: config.clone(),
            },
        })
    }

    pub fn config(&self) -> &configuration::Configuration {
        &self.config
    }
}

#[derive(Clone)]
pub struct LdapsClient {
    config: Arc<configuration::Configuration>,
}

impl LdapsClient {
    pub async fn create_ldap(
        &self,
        new_ldap: models::NewLdap,
    ) -> Result<models::CreateLdap201Response, Error<ldaps_api::CreateLdapError>> {
        ldaps_api::create_ldap(&self.config, new_ldap).await
    }

    pub async fn delete_ldap(
        &self,
        ldap_id: &str,
    ) -> Result<models::DeleteSite200Response, Error<ldaps_api::DeleteLdapError>> {
        ldaps_api::delete_ldap(&self.config, ldap_id).await
    }

    pub async fn get_ldap(
        &self,
        ldap_id: &str,
    ) -> Result<models::CreateLdap201Response, Error<ldaps_api::GetLdapError>> {
        ldaps_api::get_ldap(&self.config, ldap_id).await
    }

    pub async fn list_ldaps(
        &self,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<models::ListLdaps200Response, Error<ldaps_api::ListLdapsError>> {
        ldaps_api::list_ldaps(&self.config, limit, offset).await
    }
}

#[derive(Clone)]
pub struct OwnersClient {
    config: Arc<configuration::Configuration>,
}

impl OwnersClient {
    pub async fn create_owner(
        &self,
        owner: models::Owner,
    ) -> Result<models::CreateOwner201Response, Error<owners_api::CreateOwnerError>> {
        owners_api::create_owner(&self.config, owner).await
    }

    pub async fn delete_owner(
        &self,
        email: &str,
    ) -> Result<models::DeleteSite200Response, Error<owners_api::DeleteOwnerError>> {
        owners_api::delete_owner(&self.config, email).await
    }

    pub async fn get_owner(
        &self,
        email: &str,
    ) -> Result<models::CreateOwner201Response, Error<owners_api::GetOwnerError>> {
        owners_api::get_owner(&self.config, email).await
    }

    pub async fn list_owners(
        &self,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<models::ListOwners200Response, Error<owners_api::ListOwnersError>> {
        owners_api::list_owners(&self.config, limit, offset).await
    }
}

#[derive(Clone)]
pub struct SitesClient {
    config: Arc<configuration::Configuration>,
}

impl SitesClient {
    pub async fn create_site(
        &self,
        new_site: models::NewSite,
    ) -> Result<models::CreateSite201Response, Error<sites_api::CreateSiteError>> {
        sites_api::create_site(&self.config, new_site).await
    }

    pub async fn delete_site(
        &self,
        site_id: &str,
    ) -> Result<models::DeleteSite200Response, Error<sites_api::DeleteSiteError>> {
        sites_api::delete_site(&self.config, site_id).await
    }

    pub async fn get_site(
        &self,
        site_id: &str,
    ) -> Result<models::CreateSite201Response, Error<sites_api::GetSiteError>> {
        sites_api::get_site(&self.config, site_id).await
    }

    pub async fn list_sites(
        &self,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<models::ListSites200Response, Error<sites_api::ListSitesError>> {
        sites_api::list_sites(&self.config, limit, offset).await
    }
}

#[derive(Clone)]
pub struct TenantsClient {
    config: Arc<configuration::Configuration>,
}

impl TenantsClient {
    pub async fn create_ldap(
        &self,
        new_ldap: models::NewLdap,
    ) -> Result<models::CreateLdap201Response, Error<tenants_api::CreateLdapError>> {
        tenants_api::create_ldap(&self.config, new_ldap).await
    }

    pub async fn create_owner(
        &self,
        owner: models::Owner,
    ) -> Result<models::CreateOwner201Response, Error<tenants_api::CreateOwnerError>> {
        tenants_api::create_owner(&self.config, owner).await
    }

    pub async fn create_tenant(
        &self,
        new_tenant: models::NewTenant,
    ) -> Result<models::CreateTenant201Response, Error<tenants_api::CreateTenantError>> {
        tenants_api::create_tenant(&self.config, new_tenant).await
    }

    pub async fn delete_ldap(
        &self,
        ldap_id: &str,
    ) -> Result<models::DeleteSite200Response, Error<tenants_api::DeleteLdapError>> {
        tenants_api::delete_ldap(&self.config, ldap_id).await
    }

    pub async fn delete_owner(
        &self,
        email: &str,
    ) -> Result<models::DeleteSite200Response, Error<tenants_api::DeleteOwnerError>> {
        tenants_api::delete_owner(&self.config, email).await
    }

    pub async fn get_ldap(
        &self,
        ldap_id: &str,
    ) -> Result<models::CreateLdap201Response, Error<tenants_api::GetLdapError>> {
        tenants_api::get_ldap(&self.config, ldap_id).await
    }

    pub async fn get_owner(
        &self,
        email: &str,
    ) -> Result<models::CreateOwner201Response, Error<tenants_api::GetOwnerError>> {
        tenants_api::get_owner(&self.config, email).await
    }

    pub async fn get_tenant(
        &self,
        tenant_id: &str,
    ) -> Result<models::CreateTenant201Response, Error<tenants_api::GetTenantError>> {
        tenants_api::get_tenant(&self.config, tenant_id).await
    }

    pub async fn get_tenant_history(
        &self,
        tenant_id: &str,
    ) -> Result<models::GetTenantHistory200Response, Error<tenants_api::GetTenantHistoryError>>
    {
        tenants_api::get_tenant_history(&self.config, tenant_id).await
    }

    pub async fn list_ldaps(
        &self,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<models::ListLdaps200Response, Error<tenants_api::ListLdapsError>> {
        tenants_api::list_ldaps(&self.config, limit, offset).await
    }

    pub async fn list_owners(
        &self,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<models::ListOwners200Response, Error<tenants_api::ListOwnersError>> {
        tenants_api::list_owners(&self.config, limit, offset).await
    }

    pub async fn list_tenants(
        &self,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<models::ListTenants200Response, Error<tenants_api::ListTenantsError>> {
        tenants_api::list_tenants(&self.config, limit, offset).await
    }

    pub async fn update_tenant(
        &self,
        tenant_id: &str,
        update_tenant: models::UpdateTenant,
    ) -> Result<models::CreateTenant201Response, Error<tenants_api::UpdateTenantError>> {
        tenants_api::update_tenant(&self.config, tenant_id, update_tenant).await
    }
}
