/// TapisAuthenticator client - A high-level wrapper around the Tapis Authenticator API
///
/// # Example
///
/// ```rust,no_run
/// use tapis_authenticator::client::TapisAuthenticator;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     // With authentication
///     let client = TapisAuthenticator::new(
///         "https://api.tapis.io/v3/authenticator",
///         Some("eyJhbGc..."),
///     )?;
///
///     // Without authentication (for public endpoints like token creation)
///     let public_client = TapisAuthenticator::new(
///         "https://api.tapis.io/v3/authenticator",
///         None,
///     )?;
///
///     Ok(())
/// }
/// ```
use crate::apis::{
    admin_api, clients_api, configuration, health_check_api, metadata_api, profiles_api,
    tokens_api,
};
use http::header::{HeaderMap, HeaderValue};
use std::sync::Arc;

/// Main client for accessing the Tapis Authenticator API
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
    /// Create a new TapisAuthenticator client
    ///
    /// # Arguments
    ///
    /// * `base_url` - The base URL of the Tapis Authenticator API
    /// * `jwt_token` - Optional JWT token for authentication. Use `None` for public endpoints
    ///   like `/oauth2/tokens` that don't require authentication.
    ///
    /// # Returns
    ///
    /// A Result containing the TapisAuthenticator client or an error
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use tapis_authenticator::client::TapisAuthenticator;
    ///
    /// // For authenticated requests
    /// let client = TapisAuthenticator::new(
    ///     "https://api.tapis.io/v3/authenticator",
    ///     Some("your_jwt_token")
    /// )?;
    ///
    /// // For public endpoints (token creation, etc.)
    /// let public_client = TapisAuthenticator::new(
    ///     "https://api.tapis.io/v3/authenticator",
    ///     None
    /// )?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn new(base_url: &str, jwt_token: Option<&str>) -> Result<Self, Box<dyn std::error::Error>> {
        // Create default headers, optionally with X-Tapis-Token
        let mut headers = HeaderMap::new();
        if let Some(token) = jwt_token {
            headers.insert("X-Tapis-Token", HeaderValue::from_str(token)?);
        }

        // Create reqwest client with default headers
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        // Create configuration
        let mut config = configuration::Configuration::default();
        config.base_path = base_url.to_string();
        config.client = client;

        let config = Arc::new(config);

        Ok(TapisAuthenticator {
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

    /// Get the underlying configuration
    pub fn config(&self) -> &configuration::Configuration {
        &self.config
    }
}

/// Admin API client
#[derive(Clone)]
pub struct AdminClient {
    config: Arc<configuration::Configuration>,
}

impl AdminClient {
    /// Get the tenant configuration
    pub async fn get_config(
        &self,
    ) -> Result<
        crate::models::GetConfig200Response,
        crate::apis::Error<admin_api::GetConfigError>,
    > {
        admin_api::get_config(&self.config).await
    }

    /// Update the tenant configuration
    pub async fn update_config(
        &self,
        new_tenant_config: crate::models::NewTenantConfig,
    ) -> Result<
        crate::models::GetConfig200Response,
        crate::apis::Error<admin_api::UpdateConfigError>,
    > {
        admin_api::update_config(&self.config, new_tenant_config).await
    }
}

/// Clients API client
#[derive(Clone)]
pub struct ClientsClient {
    config: Arc<configuration::Configuration>,
}

impl ClientsClient {
    /// Create a new OAuth2 client
    pub async fn create_client(
        &self,
        new_client: crate::models::NewClient,
    ) -> Result<
        crate::models::CreateClient201Response,
        crate::apis::Error<clients_api::CreateClientError>,
    > {
        clients_api::create_client(&self.config, new_client).await
    }

    /// Delete an OAuth2 client
    pub async fn delete_client(
        &self,
        client_id: &str,
    ) -> Result<
        crate::models::DeleteClient200Response,
        crate::apis::Error<clients_api::DeleteClientError>,
    > {
        clients_api::delete_client(&self.config, client_id).await
    }

    /// Get details for a specific OAuth2 client
    pub async fn get_client(
        &self,
        client_id: &str,
    ) -> Result<
        crate::models::CreateClient201Response,
        crate::apis::Error<clients_api::GetClientError>,
    > {
        clients_api::get_client(&self.config, client_id).await
    }

    /// List all OAuth2 clients
    pub async fn list_clients(
        &self,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<
        crate::models::ListClients200Response,
        crate::apis::Error<clients_api::ListClientsError>,
    > {
        clients_api::list_clients(&self.config, limit, offset).await
    }

    /// Update an OAuth2 client
    pub async fn update_client(
        &self,
        client_id: &str,
        update_client: crate::models::UpdateClient,
    ) -> Result<
        crate::models::CreateClient201Response,
        crate::apis::Error<clients_api::UpdateClientError>,
    > {
        clients_api::update_client(&self.config, client_id, update_client).await
    }
}

/// Health Check API client
#[derive(Clone)]
pub struct HealthCheckClient {
    config: Arc<configuration::Configuration>,
}

impl HealthCheckClient {
    /// Basic health check endpoint
    pub async fn hello(
        &self,
    ) -> Result<crate::models::BasicResponse, crate::apis::Error<health_check_api::HelloError>>
    {
        health_check_api::hello(&self.config).await
    }

    /// Readiness check endpoint
    pub async fn ready(
        &self,
    ) -> Result<crate::models::BasicResponse, crate::apis::Error<health_check_api::ReadyError>>
    {
        health_check_api::ready(&self.config).await
    }
}

/// Metadata API client
#[derive(Clone)]
pub struct MetadataClient {
    config: Arc<configuration::Configuration>,
}

impl MetadataClient {
    /// Get server metadata including version information
    pub async fn get_server_metadata(
        &self,
    ) -> Result<
        crate::models::GetServerMetadata200Response,
        crate::apis::Error<metadata_api::GetServerMetadataError>,
    > {
        metadata_api::get_server_metadata(&self.config).await
    }
}

/// Profiles API client
#[derive(Clone)]
pub struct ProfilesClient {
    config: Arc<configuration::Configuration>,
}

impl ProfilesClient {
    /// Get profile information for a specific user
    pub async fn get_profile(
        &self,
        username: &str,
    ) -> Result<
        crate::models::GetUserinfo200Response,
        crate::apis::Error<profiles_api::GetProfileError>,
    > {
        profiles_api::get_profile(&self.config, username).await
    }

    /// Get user information for the authenticated user
    pub async fn get_userinfo(
        &self,
    ) -> Result<
        crate::models::GetUserinfo200Response,
        crate::apis::Error<profiles_api::GetUserinfoError>,
    > {
        profiles_api::get_userinfo(&self.config).await
    }

    /// List all user profiles
    pub async fn list_profiles(
        &self,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<
        crate::models::ListProfiles200Response,
        crate::apis::Error<profiles_api::ListProfilesError>,
    > {
        profiles_api::list_profiles(&self.config, limit, offset).await
    }
}

/// Tokens API client
#[derive(Clone)]
pub struct TokensClient {
    config: Arc<configuration::Configuration>,
}

impl TokensClient {
    /// Create a new authentication token
    pub async fn create_token(
        &self,
        new_token: crate::models::NewToken,
    ) -> Result<
        crate::models::CreateToken201Response,
        crate::apis::Error<tokens_api::CreateTokenError>,
    > {
        tokens_api::create_token(&self.config, new_token).await
    }

    /// Create a new V2 authentication token
    pub async fn create_v2_token(
        &self,
        v2_token: crate::models::V2Token,
    ) -> Result<
        crate::models::CreateV2Token200Response,
        crate::apis::Error<tokens_api::CreateV2TokenError>,
    > {
        tokens_api::create_v2_token(&self.config, v2_token).await
    }

    /// Generate a device code for device flow authentication
    pub async fn generate_device_code(
        &self,
        new_device_code: crate::models::NewDeviceCode,
    ) -> Result<
        crate::models::GenerateDeviceCode200Response,
        crate::apis::Error<tokens_api::GenerateDeviceCodeError>,
    > {
        tokens_api::generate_device_code(&self.config, new_device_code).await
    }

    /// Revoke an authentication token
    pub async fn revoke_token(
        &self,
        revoke_token_request: crate::models::RevokeTokenRequest,
    ) -> Result<crate::models::BasicResponse, crate::apis::Error<tokens_api::RevokeTokenError>>
    {
        tokens_api::revoke_token(&self.config, revoke_token_request).await
    }
}
