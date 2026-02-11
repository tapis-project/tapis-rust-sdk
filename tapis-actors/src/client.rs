use crate::apis::{configuration, Error, actors_api, aliases_api, executions_api, messages_api, nonces_api, permissions_api, search_api, state_api, workers_api};
use crate::models;
use http::header::{HeaderMap, HeaderValue};
use std::sync::Arc;

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
    pub fn new(base_url: &str, jwt_token: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut headers = HeaderMap::new();
        headers.insert("X-Tapis-Token", HeaderValue::from_str(jwt_token)?);

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        let mut config = configuration::Configuration::default();
        config.base_path = base_url.to_string();
        config.client = client;

        let config = Arc::new(config);

        Ok(Self {
            config: config.clone(),
            actors: ActorsClient { config: config.clone() },
            aliases: AliasesClient { config: config.clone() },
            executions: ExecutionsClient { config: config.clone() },
            messages: MessagesClient { config: config.clone() },
            nonces: NoncesClient { config: config.clone() },
            permissions: PermissionsClient { config: config.clone() },
            search: SearchClient { config: config.clone() },
            state: StateClient { config: config.clone() },
            workers: WorkersClient { config: config.clone() },
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
    pub async fn create_actor(&self, new_actor: models::NewActor) -> Result<models::CreateActor201Response, Error<actors_api::CreateActorError>> {
        actors_api::create_actor(&self.config, new_actor).await
    }

    pub async fn delete_actor(&self, actor_id: &str) -> Result<models::DeleteActor200Response, Error<actors_api::DeleteActorError>> {
        actors_api::delete_actor(&self.config, actor_id).await
    }

    pub async fn get_actor(&self, actor_id: &str) -> Result<models::GetActor200Response, Error<actors_api::GetActorError>> {
        actors_api::get_actor(&self.config, actor_id).await
    }

    pub async fn get_execution_result(&self, actor_id: &str, execution_id: &str) -> Result<reqwest::Response, Error<actors_api::GetExecutionResultError>> {
        actors_api::get_execution_result(&self.config, actor_id, execution_id).await
    }

    pub async fn list_actors(&self, limit: Option<i32>, offset: Option<i32>) -> Result<models::ListActors200Response, Error<actors_api::ListActorsError>> {
        actors_api::list_actors(&self.config, limit, offset).await
    }

    pub async fn update_actor(&self, actor_id: &str, new_actor: models::NewActor) -> Result<models::GetActor200Response, Error<actors_api::UpdateActorError>> {
        actors_api::update_actor(&self.config, actor_id, new_actor).await
    }

}

#[derive(Clone)]
pub struct AliasesClient {
    config: Arc<configuration::Configuration>,
}

impl AliasesClient {
    pub async fn create_alias(&self, new_alias: models::NewAlias) -> Result<models::CreateAlias201Response, Error<aliases_api::CreateAliasError>> {
        aliases_api::create_alias(&self.config, new_alias).await
    }

    pub async fn delete_alias(&self, alias: &str) -> Result<models::DeleteActor200Response, Error<aliases_api::DeleteAliasError>> {
        aliases_api::delete_alias(&self.config, alias).await
    }

    pub async fn get_alias(&self, alias: &str) -> Result<models::CreateAlias201Response, Error<aliases_api::GetAliasError>> {
        aliases_api::get_alias(&self.config, alias).await
    }

    pub async fn list_aliases(&self, limit: Option<i32>, offset: Option<i32>) -> Result<models::ListAliases200Response, Error<aliases_api::ListAliasesError>> {
        aliases_api::list_aliases(&self.config, limit, offset).await
    }

    pub async fn update_actor_alias(&self, alias: &str, new_alias: models::NewAlias) -> Result<models::CreateAlias201Response, Error<aliases_api::UpdateActorAliasError>> {
        aliases_api::update_actor_alias(&self.config, alias, new_alias).await
    }

}

#[derive(Clone)]
pub struct ExecutionsClient {
    config: Arc<configuration::Configuration>,
}

impl ExecutionsClient {
    pub async fn get_execution(&self, actor_id: &str, execution_id: &str) -> Result<models::GetExecution200Response, Error<executions_api::GetExecutionError>> {
        executions_api::get_execution(&self.config, actor_id, execution_id).await
    }

    pub async fn get_execution_logs(&self, actor_id: &str, execution_id: &str) -> Result<models::GetExecutionLogs200Response, Error<executions_api::GetExecutionLogsError>> {
        executions_api::get_execution_logs(&self.config, actor_id, execution_id).await
    }

    pub async fn get_execution_result(&self, actor_id: &str, execution_id: &str) -> Result<reqwest::Response, Error<executions_api::GetExecutionResultError>> {
        executions_api::get_execution_result(&self.config, actor_id, execution_id).await
    }

    pub async fn list_executions(&self, actor_id: &str) -> Result<models::ListExecutions200Response, Error<executions_api::ListExecutionsError>> {
        executions_api::list_executions(&self.config, actor_id).await
    }

}

#[derive(Clone)]
pub struct MessagesClient {
    config: Arc<configuration::Configuration>,
}

impl MessagesClient {
    pub async fn delete_pending_messages(&self, actor_id: &str) -> Result<models::DeleteActor200Response, Error<messages_api::DeletePendingMessagesError>> {
        messages_api::delete_pending_messages(&self.config, actor_id).await
    }

    pub async fn get_messages(&self, actor_id: &str) -> Result<models::GetMessages200Response, Error<messages_api::GetMessagesError>> {
        messages_api::get_messages(&self.config, actor_id).await
    }

    pub async fn send_binary_message(&self, actor_id: &str, binary_message: models::BinaryMessage, _abaco_synchronous: Option<&str>) -> Result<models::SendMessage200Response, Error<messages_api::SendBinaryMessageError>> {
        messages_api::send_binary_message(&self.config, actor_id, binary_message, _abaco_synchronous).await
    }

    pub async fn send_json_message(&self, actor_id: &str, json_message: models::JsonMessage, _abaco_synchronous: Option<&str>) -> Result<models::SendMessage200Response, Error<messages_api::SendJsonMessageError>> {
        messages_api::send_json_message(&self.config, actor_id, json_message, _abaco_synchronous).await
    }

    pub async fn send_message(&self, actor_id: &str, json_message: models::JsonMessage, _abaco_synchronous: Option<&str>) -> Result<models::SendMessage200Response, Error<messages_api::SendMessageError>> {
        messages_api::send_message(&self.config, actor_id, json_message, _abaco_synchronous).await
    }

}

#[derive(Clone)]
pub struct NoncesClient {
    config: Arc<configuration::Configuration>,
}

impl NoncesClient {
    pub async fn create_nonce(&self, actor_id: &str, new_actor_nonce: models::NewActorNonce) -> Result<models::ListNonces200Response, Error<nonces_api::CreateNonceError>> {
        nonces_api::create_nonce(&self.config, actor_id, new_actor_nonce).await
    }

    pub async fn delete_nonce(&self, actor_id: &str, nonce_id: &str) -> Result<models::DeleteActor200Response, Error<nonces_api::DeleteNonceError>> {
        nonces_api::delete_nonce(&self.config, actor_id, nonce_id).await
    }

    pub async fn get_nonce(&self, actor_id: &str, nonce_id: &str) -> Result<models::GetNonce200Response, Error<nonces_api::GetNonceError>> {
        nonces_api::get_nonce(&self.config, actor_id, nonce_id).await
    }

    pub async fn list_nonces(&self, actor_id: &str) -> Result<models::ListNonces200Response, Error<nonces_api::ListNoncesError>> {
        nonces_api::list_nonces(&self.config, actor_id).await
    }

}

#[derive(Clone)]
pub struct PermissionsClient {
    config: Arc<configuration::Configuration>,
}

impl PermissionsClient {
    pub async fn list_permissions(&self, actor_id: &str) -> Result<models::ListPermissions200Response, Error<permissions_api::ListPermissionsError>> {
        permissions_api::list_permissions(&self.config, actor_id).await
    }

    pub async fn update_permissions(&self, actor_id: &str, actor_permission: models::ActorPermission) -> Result<models::ListPermissions200Response, Error<permissions_api::UpdatePermissionsError>> {
        permissions_api::update_permissions(&self.config, actor_id, actor_permission).await
    }

}

#[derive(Clone)]
pub struct SearchClient {
    config: Arc<configuration::Configuration>,
}

impl SearchClient {
    pub async fn search_database(&self, search_type: &str, search: Option<&str>, limit: Option<i32>, skip: Option<i32>) -> Result<models::BasicResponse, Error<search_api::SearchDatabaseError>> {
        search_api::search_database(&self.config, search_type, search, limit, skip).await
    }

}

#[derive(Clone)]
pub struct StateClient {
    config: Arc<configuration::Configuration>,
}

impl StateClient {
    pub async fn get_state(&self, actor_id: &str) -> Result<models::GetState200Response, Error<state_api::GetStateError>> {
        state_api::get_state(&self.config, actor_id).await
    }

    pub async fn update_state(&self, actor_id: &str, body: serde_json::Value) -> Result<models::ListNonces200Response, Error<state_api::UpdateStateError>> {
        state_api::update_state(&self.config, actor_id, body).await
    }

}

#[derive(Clone)]
pub struct WorkersClient {
    config: Arc<configuration::Configuration>,
}

impl WorkersClient {
    pub async fn list_workers(&self, actor_id: &str) -> Result<models::ActorWorkerResponse, Error<workers_api::ListWorkersError>> {
        workers_api::list_workers(&self.config, actor_id).await
    }

    pub async fn manage_worker_pool_size(&self, actor_id: &str, manage_worker_pool_size_request: models::ManageWorkerPoolSizeRequest) -> Result<models::SendMessage200Response, Error<workers_api::ManageWorkerPoolSizeError>> {
        workers_api::manage_worker_pool_size(&self.config, actor_id, manage_worker_pool_size_request).await
    }

}

