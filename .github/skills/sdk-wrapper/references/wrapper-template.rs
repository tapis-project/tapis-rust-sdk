use crate::apis::configuration;
use http::header::{HeaderMap, HeaderValue};
use std::sync::Arc;

#[derive(Clone)]
pub struct TapisService {
    config: Arc<configuration::Configuration>,
    // pub resource: ResourceClient,
}

impl TapisService {
    pub fn new(base_url: &str, jwt_token: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut headers = HeaderMap::new();
        headers.insert("X-Tapis-Token", HeaderValue::from_str(jwt_token)?);

        let client = reqwest::Client::builder().default_headers(headers).build()?;

        let mut config = configuration::Configuration::default();
        config.base_path = base_url.to_string();
        config.client = client;
        let config = Arc::new(config);

        Ok(Self {
            config: config.clone(),
            // resource: ResourceClient { config },
        })
    }

    pub fn config(&self) -> &configuration::Configuration {
        &self.config
    }
}

#[derive(Clone)]
pub struct ResourceClient {
    config: Arc<configuration::Configuration>,
}

impl ResourceClient {
    // Copy signature from generated API and delegate:
    // pub async fn list_x(&self, ...) -> Result<..., crate::apis::Error<...>> {
    //     crate::apis::resource_api::list_x(&self.config, ...).await
    // }
}
