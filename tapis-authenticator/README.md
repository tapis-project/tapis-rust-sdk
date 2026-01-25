# Rust API client for tapis_authenticator

REST API and web server providing authentication for a Tapis v3 instance.

For more information, please visit [https://tapis-project.org](https://tapis-project.org)

## Overview

This SDK provides both low-level generated API bindings and a high-level ergonomic client wrapper for the Tapis Authenticator API.

- API version: 1
- Package version: 1.0.0
- Generator version: 7.18.0
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `tapis_authenticator` and add the following to `Cargo.toml` under `[dependencies]`:

```toml
tapis_authenticator = { path = "./tapis_authenticator" }
```

## Quick Start

### Using the High-Level Client (Recommended)

The `TapisAuthenticator` client provides an ergonomic interface with automatic JWT authentication:

```rust
use tapis_authenticator::client::TapisAuthenticator;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client with automatic JWT authentication
    let client = TapisAuthenticator::new(
        "https://tacc.tapis.io/v3/authenticator",
        "your_jwt_token"
    )?;

    // Get user information
    let userinfo = client.profiles.get_userinfo().await?;
    println!("User: {:#?}", userinfo);

    // List OAuth2 clients
    let clients = client.clients.list_clients(Some(10), Some(0)).await?;
    println!("Clients: {:#?}", clients);

    // Check server health
    let health = client.health_check.hello().await?;
    println!("Health: {:#?}", health);

    Ok(())
}
```

### Available API Modules

The client provides the following sub-clients:

- **`client.admin`** - Tenant configuration management
  - `get_config()` - Get tenant configuration
  - `update_config()` - Update tenant configuration

- **`client.clients`** - OAuth2 client management
  - `list_clients()` - List all OAuth2 clients
  - `get_client()` - Get specific client details
  - `create_client()` - Create new OAuth2 client
  - `update_client()` - Update OAuth2 client
  - `delete_client()` - Delete OAuth2 client

- **`client.health_check`** - Service health checks
  - `hello()` - Basic health check
  - `ready()` - Readiness check

- **`client.metadata`** - Server metadata
  - `get_server_metadata()` - Get server version and metadata

- **`client.profiles`** - User profile management
  - `get_userinfo()` - Get current user information
  - `get_profile()` - Get specific user profile
  - `list_profiles()` - List all profiles

- **`client.tokens`** - Token management
  - `create_token()` - Generate Tapis JWT
  - `create_v2_token()` - Create v2 bearer token
  - `generate_device_code()` - Generate device code for device flow
  - `revoke_token()` - Revoke authentication token

### Running the Example

```bash
# Set your JWT token
export TAPIS_TOKEN="your_jwt_token"

# Run the example
cargo run --example authenticator_example
```

## Documentation for API Endpoints

All URIs are relative to *http://localhost:5000*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*AdminApi* | [**get_config**](docs/AdminApi.md#get_config) | **GET** /v3/oauth2/admin/config | 
*AdminApi* | [**update_config**](docs/AdminApi.md#update_config) | **PUT** /v3/oauth2/admin/config | 
*ClientsApi* | [**create_client**](docs/ClientsApi.md#create_client) | **POST** /v3/oauth2/clients | 
*ClientsApi* | [**delete_client**](docs/ClientsApi.md#delete_client) | **DELETE** /v3/oauth2/clients/{client_id} | Permanently set a client to inactive.
*ClientsApi* | [**get_client**](docs/ClientsApi.md#get_client) | **GET** /v3/oauth2/clients/{client_id} | Get client details
*ClientsApi* | [**list_clients**](docs/ClientsApi.md#list_clients) | **GET** /v3/oauth2/clients | 
*ClientsApi* | [**update_client**](docs/ClientsApi.md#update_client) | **PUT** /v3/oauth2/clients/{client_id} | Update client details
*HealthCheckApi* | [**hello**](docs/HealthCheckApi.md#hello) | **GET** /v3/oauth2/hello | 
*HealthCheckApi* | [**ready**](docs/HealthCheckApi.md#ready) | **GET** /v3/oauth2/ready | 
*MetadataApi* | [**get_server_metadata**](docs/MetadataApi.md#get_server_metadata) | **GET** /v3/oauth2/.well-known/oauth-authorization-server | 
*ProfilesApi* | [**get_profile**](docs/ProfilesApi.md#get_profile) | **GET** /v3/oauth2/profiles/{username} | 
*ProfilesApi* | [**get_userinfo**](docs/ProfilesApi.md#get_userinfo) | **GET** /v3/oauth2/userinfo | 
*ProfilesApi* | [**list_profiles**](docs/ProfilesApi.md#list_profiles) | **GET** /v3/oauth2/profiles | 
*TokensApi* | [**create_token**](docs/TokensApi.md#create_token) | **POST** /v3/oauth2/tokens | Generate a Tapis JWT
*TokensApi* | [**create_v2_token**](docs/TokensApi.md#create_v2_token) | **POST** /v3/oauth2/v2/token | Create a v2 bearer token from a Tapis v3 JWT.
*TokensApi* | [**generate_device_code**](docs/TokensApi.md#generate_device_code) | **POST** /v3/oauth2/device/code | Generate a device code.
*TokensApi* | [**revoke_token**](docs/TokensApi.md#revoke_token) | **POST** /v3/oauth2/tokens/revoke | Revoke a token.


## Documentation For Models

 - [BasicResponse](docs/BasicResponse.md)
 - [Client](docs/Client.md)
 - [CreateClient201Response](docs/CreateClient201Response.md)
 - [CreateToken201Response](docs/CreateToken201Response.md)
 - [CreateV2Token200Response](docs/CreateV2Token200Response.md)
 - [DeleteClient200Response](docs/DeleteClient200Response.md)
 - [DeviceCodeResposne](docs/DeviceCodeResposne.md)
 - [GenerateDeviceCode200Response](docs/GenerateDeviceCode200Response.md)
 - [GetConfig200Response](docs/GetConfig200Response.md)
 - [GetServerMetadata200Response](docs/GetServerMetadata200Response.md)
 - [GetUserinfo200Response](docs/GetUserinfo200Response.md)
 - [ListClients200Response](docs/ListClients200Response.md)
 - [ListProfiles200Response](docs/ListProfiles200Response.md)
 - [NewClient](docs/NewClient.md)
 - [NewDeviceCode](docs/NewDeviceCode.md)
 - [NewTenantConfig](docs/NewTenantConfig.md)
 - [NewToken](docs/NewToken.md)
 - [OAuth2Metadata](docs/OAuth2Metadata.md)
 - [Profile](docs/Profile.md)
 - [RevokeTokenRequest](docs/RevokeTokenRequest.md)
 - [TenantConfig](docs/TenantConfig.md)
 - [TokenResponse](docs/TokenResponse.md)
 - [TokenResponseAccessToken](docs/TokenResponseAccessToken.md)
 - [TokenResponseRefreshToken](docs/TokenResponseRefreshToken.md)
 - [UpdateClient](docs/UpdateClient.md)
 - [V2Token](docs/V2Token.md)
 - [V2TokenResponse](docs/V2TokenResponse.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author

cicsupport@tacc.utexas.edu

