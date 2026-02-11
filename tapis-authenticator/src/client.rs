use crate::apis::{
    admin_api, clients_api, configuration, health_check_api, metadata_api, profiles_api,
    tokens_api, Error,
};
use crate::models;
use http::header::{HeaderMap, HeaderValue};
use std::sync::Arc;

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
        let mut headers = HeaderMap::new();
        if let Some(token) = jwt_token {
            headers.insert("X-Tapis-Token", HeaderValue::from_str(token)?);
        }

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        let mut config = configuration::Configuration::default();
        config.base_path = base_url.to_string();
        config.client = client;

        let config = Arc::new(config);

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
