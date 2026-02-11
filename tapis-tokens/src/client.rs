use crate::apis::{configuration, health_check_api, keys_api, tokens_api, Error};
use crate::models;
use http::header::{HeaderMap, HeaderValue};
use reqwest::{Client, Request, Response};
use reqwest_middleware::{ClientBuilder, Middleware, Next, Result as MiddlewareResult};
use std::sync::Arc;

#[derive(Debug)]
struct LoggingMiddleware;

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

#[derive(Clone)]
pub struct TapisTokens {
    config: Arc<configuration::Configuration>,
    pub health_check: HealthCheckClient,
    pub keys: KeysClient,
    pub tokens: TokensClient,
}

impl TapisTokens {
    pub fn new(
        base_url: &str,
        jwt_token: Option<&str>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut headers = HeaderMap::new();
        if let Some(token) = jwt_token {
            headers.insert("X-Tapis-Token", HeaderValue::from_str(token)?);
        }

        let reqwest_client = Client::builder().default_headers(headers).build()?;

        let client = ClientBuilder::new(reqwest_client)
            .with(LoggingMiddleware)
            .build();

        let mut config = configuration::Configuration::default();
        config.base_path = base_url.to_string();
        config.client = client;

        let config = Arc::new(config);

        Ok(Self {
            config: config.clone(),
            health_check: HealthCheckClient {
                config: config.clone(),
            },
            keys: KeysClient {
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
pub struct KeysClient {
    config: Arc<configuration::Configuration>,
}

impl KeysClient {
    pub async fn update_keys(
        &self,
        new_signing_keys_request: models::NewSigningKeysRequest,
    ) -> Result<models::UpdateKeys201Response, Error<keys_api::UpdateKeysError>> {
        keys_api::update_keys(&self.config, new_signing_keys_request).await
    }
}

#[derive(Clone)]
pub struct TokensClient {
    config: Arc<configuration::Configuration>,
}

impl TokensClient {
    pub async fn create_token(
        &self,
        new_token_request: models::NewTokenRequest,
    ) -> Result<models::RefreshToken201Response, Error<tokens_api::CreateTokenError>> {
        tokens_api::create_token(&self.config, new_token_request).await
    }

    pub async fn refresh_token(
        &self,
        refresh_token_request: models::RefreshTokenRequest,
    ) -> Result<models::RefreshToken201Response, Error<tokens_api::RefreshTokenError>> {
        tokens_api::refresh_token(&self.config, refresh_token_request).await
    }

    pub async fn revoke_token(
        &self,
        revoke_token_request: models::RevokeTokenRequest,
    ) -> Result<models::BasicResponse, Error<tokens_api::RevokeTokenError>> {
        tokens_api::revoke_token(&self.config, revoke_token_request).await
    }

    pub async fn update_keys(
        &self,
        new_signing_keys_request: models::NewSigningKeysRequest,
    ) -> Result<models::UpdateKeys201Response, Error<tokens_api::UpdateKeysError>> {
        tokens_api::update_keys(&self.config, new_signing_keys_request).await
    }
}
