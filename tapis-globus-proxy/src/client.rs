use crate::apis::{
    auth_api, configuration, file_operations_api, general_api, transfers_api, Error,
};
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
pub struct TapisGlobusProxy {
    config: Arc<configuration::Configuration>,
    pub auth: AuthClient,
    pub file_operations: FileOperationsClient,
    pub general: GeneralClient,
    pub transfers: TransfersClient,
}

impl TapisGlobusProxy {
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
            auth: AuthClient {
                config: config.clone(),
            },
            file_operations: FileOperationsClient {
                config: config.clone(),
            },
            general: GeneralClient {
                config: config.clone(),
            },
            transfers: TransfersClient {
                config: config.clone(),
            },
        })
    }

    pub fn config(&self) -> &configuration::Configuration {
        &self.config
    }
}

#[derive(Clone)]
pub struct AuthClient {
    config: Arc<configuration::Configuration>,
}

impl AuthClient {
    pub async fn check_tokens(
        &self,
        client_id: &str,
        endpoint_id: &str,
        access_token: &str,
        refresh_token: &str,
    ) -> Result<models::RespAuthTokens, Error<auth_api::CheckTokensError>> {
        auth_api::check_tokens(
            &self.config,
            client_id,
            endpoint_id,
            access_token,
            refresh_token,
        )
        .await
    }

    pub async fn get_auth_info(
        &self,
        client_id: &str,
        endpoint_id: &str,
    ) -> Result<models::RespGlobusAuthInfo, Error<auth_api::GetAuthInfoError>> {
        auth_api::get_auth_info(&self.config, client_id, endpoint_id).await
    }

    pub async fn get_tokens(
        &self,
        client_id: &str,
        session_id: &str,
        auth_code: &str,
    ) -> Result<models::RespAuthTokens, Error<auth_api::GetTokensError>> {
        auth_api::get_tokens(&self.config, client_id, session_id, auth_code).await
    }
}

#[derive(Clone)]
pub struct FileOperationsClient {
    config: Arc<configuration::Configuration>,
}

impl FileOperationsClient {
    pub async fn delete_path(
        &self,
        client_id: &str,
        endpoint_id: &str,
        path: &str,
        access_token: &str,
        refresh_token: &str,
        recurse: Option<bool>,
    ) -> Result<models::RespBasic, Error<file_operations_api::DeletePathError>> {
        file_operations_api::delete_path(
            &self.config,
            client_id,
            endpoint_id,
            path,
            access_token,
            refresh_token,
            recurse,
        )
        .await
    }

    pub async fn list_files(
        &self,
        client_id: &str,
        endpoint_id: &str,
        path: &str,
        access_token: &str,
        refresh_token: &str,
        limit: Option<i32>,
        offset: Option<i32>,
        filter: Option<&str>,
    ) -> Result<models::RespFileList, Error<file_operations_api::ListFilesError>> {
        file_operations_api::list_files(
            &self.config,
            client_id,
            endpoint_id,
            path,
            access_token,
            refresh_token,
            limit,
            offset,
            filter,
        )
        .await
    }

    pub async fn make_dir(
        &self,
        client_id: &str,
        endpoint_id: &str,
        path: &str,
        access_token: &str,
        refresh_token: &str,
        req_make_dir: Option<models::ReqMakeDir>,
    ) -> Result<models::RespBasic, Error<file_operations_api::MakeDirError>> {
        file_operations_api::make_dir(
            &self.config,
            client_id,
            endpoint_id,
            path,
            access_token,
            refresh_token,
            req_make_dir,
        )
        .await
    }

    pub async fn rename_path(
        &self,
        client_id: &str,
        endpoint_id: &str,
        path: &str,
        access_token: &str,
        refresh_token: &str,
        req_rename: Option<models::ReqRename>,
    ) -> Result<models::RespBasic, Error<file_operations_api::RenamePathError>> {
        file_operations_api::rename_path(
            &self.config,
            client_id,
            endpoint_id,
            path,
            access_token,
            refresh_token,
            req_rename,
        )
        .await
    }
}

#[derive(Clone)]
pub struct GeneralClient {
    config: Arc<configuration::Configuration>,
}

impl GeneralClient {
    pub async fn health_check(
        &self,
    ) -> Result<models::RespBasic, Error<general_api::HealthCheckError>> {
        general_api::health_check(&self.config).await
    }
}

#[derive(Clone)]
pub struct TransfersClient {
    config: Arc<configuration::Configuration>,
}

impl TransfersClient {
    pub async fn cancel_transfer_task(
        &self,
        client_id: &str,
        task_id: &str,
        access_token: &str,
        refresh_token: &str,
    ) -> Result<models::RespCancelTask, Error<transfers_api::CancelTransferTaskError>> {
        transfers_api::cancel_transfer_task(
            &self.config,
            client_id,
            task_id,
            access_token,
            refresh_token,
        )
        .await
    }

    pub async fn create_transfer_task(
        &self,
        client_id: &str,
        access_token: &str,
        refresh_token: &str,
        req_create_transfer: Option<models::ReqCreateTransfer>,
    ) -> Result<models::RespTransferTask, Error<transfers_api::CreateTransferTaskError>> {
        transfers_api::create_transfer_task(
            &self.config,
            client_id,
            access_token,
            refresh_token,
            req_create_transfer,
        )
        .await
    }

    pub async fn get_transfer_task(
        &self,
        client_id: &str,
        task_id: &str,
        access_token: &str,
        refresh_token: &str,
    ) -> Result<models::RespTransferTask, Error<transfers_api::GetTransferTaskError>> {
        transfers_api::get_transfer_task(
            &self.config,
            client_id,
            task_id,
            access_token,
            refresh_token,
        )
        .await
    }
}
