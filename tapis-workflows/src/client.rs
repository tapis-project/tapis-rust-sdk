use crate::apis::{
    archives_api, cicd_api, configuration, etl_api, general_api, group_secrets_api, groups_api,
    identities_api, pipeline_archives_api, pipeline_locks_api, pipeline_runs_api, pipelines_api,
    secrets_api, task_executions_api, tasks_api, users_api, Error,
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
pub struct TapisWorkflows {
    config: Arc<configuration::Configuration>,
    pub archives: ArchivesClient,
    pub cicd: CicdClient,
    pub etl: EtlClient,
    pub general: GeneralClient,
    pub group_secrets: GroupSecretsClient,
    pub groups: GroupsClient,
    pub identities: IdentitiesClient,
    pub pipeline_archives: PipelineArchivesClient,
    pub pipeline_locks: PipelineLocksClient,
    pub pipeline_runs: PipelineRunsClient,
    pub pipelines: PipelinesClient,
    pub secrets: SecretsClient,
    pub task_executions: TaskExecutionsClient,
    pub tasks: TasksClient,
    pub users: UsersClient,
}

impl TapisWorkflows {
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
            archives: ArchivesClient {
                config: config.clone(),
            },
            cicd: CicdClient {
                config: config.clone(),
            },
            etl: EtlClient {
                config: config.clone(),
            },
            general: GeneralClient {
                config: config.clone(),
            },
            group_secrets: GroupSecretsClient {
                config: config.clone(),
            },
            groups: GroupsClient {
                config: config.clone(),
            },
            identities: IdentitiesClient {
                config: config.clone(),
            },
            pipeline_archives: PipelineArchivesClient {
                config: config.clone(),
            },
            pipeline_locks: PipelineLocksClient {
                config: config.clone(),
            },
            pipeline_runs: PipelineRunsClient {
                config: config.clone(),
            },
            pipelines: PipelinesClient {
                config: config.clone(),
            },
            secrets: SecretsClient {
                config: config.clone(),
            },
            task_executions: TaskExecutionsClient {
                config: config.clone(),
            },
            tasks: TasksClient {
                config: config.clone(),
            },
            users: UsersClient {
                config: config.clone(),
            },
        })
    }

    pub fn config(&self) -> &configuration::Configuration {
        &self.config
    }
}

#[derive(Clone)]
pub struct ArchivesClient {
    config: Arc<configuration::Configuration>,
}

impl ArchivesClient {
    pub async fn create_archive(
        &self,
        group_id: &str,
        req_archive: models::ReqArchive,
    ) -> Result<models::RespResourceUrl, Error<archives_api::CreateArchiveError>> {
        archives_api::create_archive(&self.config, group_id, req_archive).await
    }

    pub async fn get_archive(
        &self,
        group_id: &str,
        archive_id: &str,
    ) -> Result<models::RespArchive, Error<archives_api::GetArchiveError>> {
        archives_api::get_archive(&self.config, group_id, archive_id).await
    }

    pub async fn list_archives(
        &self,
        group_id: &str,
    ) -> Result<models::RespArchiveList, Error<archives_api::ListArchivesError>> {
        archives_api::list_archives(&self.config, group_id).await
    }
}

#[derive(Clone)]
pub struct CicdClient {
    config: Arc<configuration::Configuration>,
}

impl CicdClient {
    pub async fn create_ci_pipeline(
        &self,
        group_id: &str,
        req_ci_pipeline: models::ReqCiPipeline,
    ) -> Result<models::RespResourceUrl, Error<cicd_api::CreateCiPipelineError>> {
        cicd_api::create_ci_pipeline(&self.config, group_id, req_ci_pipeline).await
    }
}

#[derive(Clone)]
pub struct EtlClient {
    config: Arc<configuration::Configuration>,
}

impl EtlClient {
    pub async fn create_etl_pipeline(
        &self,
        group_id: &str,
        req_create_etl_pipeline: models::ReqCreateEtlPipeline,
    ) -> Result<models::RespResourceUrl, Error<etl_api::CreateEtlPipelineError>> {
        etl_api::create_etl_pipeline(&self.config, group_id, req_create_etl_pipeline).await
    }
}

#[derive(Clone)]
pub struct GeneralClient {
    config: Arc<configuration::Configuration>,
}

impl GeneralClient {
    pub async fn health_check(
        &self,
    ) -> Result<models::RespBase, Error<general_api::HealthCheckError>> {
        general_api::health_check(&self.config).await
    }
}

#[derive(Clone)]
pub struct GroupSecretsClient {
    config: Arc<configuration::Configuration>,
}

impl GroupSecretsClient {
    pub async fn add_group_secret(
        &self,
        group_id: &str,
        req_group_secret: models::ReqGroupSecret,
    ) -> Result<models::RespGroupSecret, Error<group_secrets_api::AddGroupSecretError>> {
        group_secrets_api::add_group_secret(&self.config, group_id, req_group_secret).await
    }

    pub async fn get_group_secret(
        &self,
        group_id: &str,
        group_secret_id: &str,
    ) -> Result<models::RespGroupSecret, Error<group_secrets_api::GetGroupSecretError>> {
        group_secrets_api::get_group_secret(&self.config, group_id, group_secret_id).await
    }

    pub async fn list_group_secrets(
        &self,
        group_id: &str,
    ) -> Result<models::RespGroupSecretList, Error<group_secrets_api::ListGroupSecretsError>> {
        group_secrets_api::list_group_secrets(&self.config, group_id).await
    }

    pub async fn remove_group_secret(
        &self,
        group_id: &str,
        group_secret_id: &str,
    ) -> Result<models::RespBase, Error<group_secrets_api::RemoveGroupSecretError>> {
        group_secrets_api::remove_group_secret(&self.config, group_id, group_secret_id).await
    }
}

#[derive(Clone)]
pub struct GroupsClient {
    config: Arc<configuration::Configuration>,
}

impl GroupsClient {
    pub async fn create_group(
        &self,
        req_group: models::ReqGroup,
    ) -> Result<models::RespResourceUrl, Error<groups_api::CreateGroupError>> {
        groups_api::create_group(&self.config, req_group).await
    }

    pub async fn delete_group(
        &self,
        group_id: &str,
    ) -> Result<models::RespString, Error<groups_api::DeleteGroupError>> {
        groups_api::delete_group(&self.config, group_id).await
    }

    pub async fn get_group(
        &self,
        group_id: &str,
    ) -> Result<models::RespGroupDetail, Error<groups_api::GetGroupError>> {
        groups_api::get_group(&self.config, group_id).await
    }

    pub async fn list_groups(
        &self,
    ) -> Result<models::RespGroupList, Error<groups_api::ListGroupsError>> {
        groups_api::list_groups(&self.config).await
    }
}

#[derive(Clone)]
pub struct IdentitiesClient {
    config: Arc<configuration::Configuration>,
}

impl IdentitiesClient {
    pub async fn create_identity(
        &self,
        req_identity: models::ReqIdentity,
    ) -> Result<models::RespResourceUrl, Error<identities_api::CreateIdentityError>> {
        identities_api::create_identity(&self.config, req_identity).await
    }

    pub async fn delete_identity(
        &self,
        identity_uuid: &str,
    ) -> Result<models::RespString, Error<identities_api::DeleteIdentityError>> {
        identities_api::delete_identity(&self.config, identity_uuid).await
    }

    pub async fn get_identity(
        &self,
        identity_uuid: &str,
    ) -> Result<models::RespIdentity, Error<identities_api::GetIdentityError>> {
        identities_api::get_identity(&self.config, identity_uuid).await
    }

    pub async fn list_identities(
        &self,
    ) -> Result<models::RespIdentityList, Error<identities_api::ListIdentitiesError>> {
        identities_api::list_identities(&self.config).await
    }
}

#[derive(Clone)]
pub struct PipelineArchivesClient {
    config: Arc<configuration::Configuration>,
}

impl PipelineArchivesClient {
    pub async fn list_pipeline_archives(
        &self,
        group_id: &str,
        pipeline_id: &str,
    ) -> Result<models::RespArchiveList, Error<pipeline_archives_api::ListPipelineArchivesError>>
    {
        pipeline_archives_api::list_pipeline_archives(&self.config, group_id, pipeline_id).await
    }
}

#[derive(Clone)]
pub struct PipelineLocksClient {
    config: Arc<configuration::Configuration>,
}

impl PipelineLocksClient {
    pub async fn get_pipeline_lock(
        &self,
        group_id: &str,
        pipeline_id: &str,
        pipeline_lock_uuid: &str,
    ) -> Result<models::RespPipelineLock, Error<pipeline_locks_api::GetPipelineLockError>> {
        pipeline_locks_api::get_pipeline_lock(
            &self.config,
            group_id,
            pipeline_id,
            pipeline_lock_uuid,
        )
        .await
    }

    pub async fn list_pipeline_locks(
        &self,
        group_id: &str,
        pipeline_id: &str,
    ) -> Result<models::RespPipelineLockList, Error<pipeline_locks_api::ListPipelineLocksError>>
    {
        pipeline_locks_api::list_pipeline_locks(&self.config, group_id, pipeline_id).await
    }

    pub async fn release_pipeline_lock(
        &self,
        group_id: &str,
        pipeline_id: &str,
        pipeline_run_uuid: &str,
    ) -> Result<(), Error<pipeline_locks_api::ReleasePipelineLockError>> {
        pipeline_locks_api::release_pipeline_lock(
            &self.config,
            group_id,
            pipeline_id,
            pipeline_run_uuid,
        )
        .await
    }
}

#[derive(Clone)]
pub struct PipelineRunsClient {
    config: Arc<configuration::Configuration>,
}

impl PipelineRunsClient {
    pub async fn acquire_pipeline_lock(
        &self,
        group_id: &str,
        pipeline_id: &str,
        pipeline_run_uuid: &str,
        req_pipeline_lock: models::ReqPipelineLock,
    ) -> Result<
        models::RespPipelineLockAcquisition,
        Error<pipeline_runs_api::AcquirePipelineLockError>,
    > {
        pipeline_runs_api::acquire_pipeline_lock(
            &self.config,
            group_id,
            pipeline_id,
            pipeline_run_uuid,
            req_pipeline_lock,
        )
        .await
    }

    pub async fn get_pipeline_run(
        &self,
        group_id: &str,
        pipeline_id: &str,
        pipeline_run_uuid: &str,
    ) -> Result<models::RespPipelineRun, Error<pipeline_runs_api::GetPipelineRunError>> {
        pipeline_runs_api::get_pipeline_run(&self.config, group_id, pipeline_id, pipeline_run_uuid)
            .await
    }

    pub async fn list_pipeline_runs(
        &self,
        group_id: &str,
        pipeline_id: &str,
    ) -> Result<models::RespPipelineRunList, Error<pipeline_runs_api::ListPipelineRunsError>> {
        pipeline_runs_api::list_pipeline_runs(&self.config, group_id, pipeline_id).await
    }

    pub async fn terminate_pipeline(
        &self,
        group_id: &str,
        pipeline_id: &str,
        pipeline_run_uuid: &str,
    ) -> Result<models::RespPipelineRun, Error<pipeline_runs_api::TerminatePipelineError>> {
        pipeline_runs_api::terminate_pipeline(
            &self.config,
            group_id,
            pipeline_id,
            pipeline_run_uuid,
        )
        .await
    }

    pub async fn update_pipeline_run_status(
        &self,
        x_workflow_executor_token: &str,
        pipeline_run_uuid: &str,
        status: models::EnumRunStatus,
        req_patch_pipeline_run: Option<models::ReqPatchPipelineRun>,
    ) -> Result<models::RespString, Error<pipeline_runs_api::UpdatePipelineRunStatusError>> {
        pipeline_runs_api::update_pipeline_run_status(
            &self.config,
            x_workflow_executor_token,
            pipeline_run_uuid,
            status,
            req_patch_pipeline_run,
        )
        .await
    }
}

#[derive(Clone)]
pub struct PipelinesClient {
    config: Arc<configuration::Configuration>,
}

impl PipelinesClient {
    pub async fn add_pipeline_archive(
        &self,
        group_id: &str,
        pipeline_id: &str,
    ) -> Result<models::RespBase, Error<pipelines_api::AddPipelineArchiveError>> {
        pipelines_api::add_pipeline_archive(&self.config, group_id, pipeline_id).await
    }

    pub async fn change_pipeline_owner(
        &self,
        group_id: &str,
        pipeline_id: &str,
        username: &str,
    ) -> Result<models::RespBase, Error<pipelines_api::ChangePipelineOwnerError>> {
        pipelines_api::change_pipeline_owner(&self.config, group_id, pipeline_id, username).await
    }

    pub async fn create_pipeline(
        &self,
        group_id: &str,
        req_pipeline: models::ReqPipeline,
    ) -> Result<models::RespResourceUrl, Error<pipelines_api::CreatePipelineError>> {
        pipelines_api::create_pipeline(&self.config, group_id, req_pipeline).await
    }

    pub async fn delete_pipeline(
        &self,
        group_id: &str,
        pipeline_id: &str,
    ) -> Result<models::RespString, Error<pipelines_api::DeletePipelineError>> {
        pipelines_api::delete_pipeline(&self.config, group_id, pipeline_id).await
    }

    pub async fn get_pipeline(
        &self,
        group_id: &str,
        pipeline_id: &str,
    ) -> Result<models::RespPipeline, Error<pipelines_api::GetPipelineError>> {
        pipelines_api::get_pipeline(&self.config, group_id, pipeline_id).await
    }

    pub async fn list_pipelines(
        &self,
        group_id: &str,
    ) -> Result<models::RespPipelineList, Error<pipelines_api::ListPipelinesError>> {
        pipelines_api::list_pipelines(&self.config, group_id).await
    }

    pub async fn remove_pipeline_archive(
        &self,
        group_id: &str,
        pipeline_id: &str,
    ) -> Result<models::RespBase, Error<pipelines_api::RemovePipelineArchiveError>> {
        pipelines_api::remove_pipeline_archive(&self.config, group_id, pipeline_id).await
    }

    pub async fn run_pipeline(
        &self,
        group_id: &str,
        pipeline_id: &str,
        req_run_pipeline: models::ReqRunPipeline,
    ) -> Result<models::RespPipelineRun, Error<pipelines_api::RunPipelineError>> {
        pipelines_api::run_pipeline(&self.config, group_id, pipeline_id, req_run_pipeline).await
    }
}

#[derive(Clone)]
pub struct SecretsClient {
    config: Arc<configuration::Configuration>,
}

impl SecretsClient {
    pub async fn create_secret(
        &self,
        req_create_secret: models::ReqCreateSecret,
    ) -> Result<models::RespSecret, Error<secrets_api::CreateSecretError>> {
        secrets_api::create_secret(&self.config, req_create_secret).await
    }

    pub async fn delete_secret(
        &self,
        secret_id: &str,
    ) -> Result<models::RespString, Error<secrets_api::DeleteSecretError>> {
        secrets_api::delete_secret(&self.config, secret_id).await
    }

    pub async fn get_secret(
        &self,
        secret_id: &str,
    ) -> Result<models::RespSecret, Error<secrets_api::GetSecretError>> {
        secrets_api::get_secret(&self.config, secret_id).await
    }

    pub async fn list_secrets(
        &self,
    ) -> Result<models::RespSecretList, Error<secrets_api::ListSecretsError>> {
        secrets_api::list_secrets(&self.config).await
    }
}

#[derive(Clone)]
pub struct TaskExecutionsClient {
    config: Arc<configuration::Configuration>,
}

impl TaskExecutionsClient {
    pub async fn create_task_execution(
        &self,
        x_workflow_executor_token: &str,
        pipeline_run_uuid: &str,
        req_create_task_execution: models::ReqCreateTaskExecution,
    ) -> Result<models::RespResourceUrl, Error<task_executions_api::CreateTaskExecutionError>> {
        task_executions_api::create_task_execution(
            &self.config,
            x_workflow_executor_token,
            pipeline_run_uuid,
            req_create_task_execution,
        )
        .await
    }

    pub async fn get_task_execution(
        &self,
        group_id: &str,
        pipeline_id: &str,
        pipeline_run_uuid: &str,
        task_execution_uuid: &str,
    ) -> Result<models::RespTaskExecution, Error<task_executions_api::GetTaskExecutionError>> {
        task_executions_api::get_task_execution(
            &self.config,
            group_id,
            pipeline_id,
            pipeline_run_uuid,
            task_execution_uuid,
        )
        .await
    }

    pub async fn list_task_executions(
        &self,
        group_id: &str,
        pipeline_id: &str,
        pipeline_run_uuid: &str,
    ) -> Result<models::RespTaskExecutionList, Error<task_executions_api::ListTaskExecutionsError>>
    {
        task_executions_api::list_task_executions(
            &self.config,
            group_id,
            pipeline_id,
            pipeline_run_uuid,
        )
        .await
    }

    pub async fn update_task_execution_status(
        &self,
        x_workflow_executor_token: &str,
        task_execution_uuid: &str,
        status: models::EnumRunStatus,
        req_patch_task_execution: Option<models::ReqPatchTaskExecution>,
    ) -> Result<models::RespString, Error<task_executions_api::UpdateTaskExecutionStatusError>>
    {
        task_executions_api::update_task_execution_status(
            &self.config,
            x_workflow_executor_token,
            task_execution_uuid,
            status,
            req_patch_task_execution,
        )
        .await
    }
}

#[derive(Clone)]
pub struct TasksClient {
    config: Arc<configuration::Configuration>,
}

impl TasksClient {
    pub async fn create_task(
        &self,
        group_id: &str,
        pipeline_id: &str,
        req_task: models::ReqTask,
    ) -> Result<models::RespResourceUrl, Error<tasks_api::CreateTaskError>> {
        tasks_api::create_task(&self.config, group_id, pipeline_id, req_task).await
    }

    pub async fn delete_task(
        &self,
        group_id: &str,
        pipeline_id: &str,
        task_id: &str,
    ) -> Result<models::RespString, Error<tasks_api::DeleteTaskError>> {
        tasks_api::delete_task(&self.config, group_id, pipeline_id, task_id).await
    }

    pub async fn get_task(
        &self,
        group_id: &str,
        pipeline_id: &str,
        task_id: &str,
    ) -> Result<models::RespTask, Error<tasks_api::GetTaskError>> {
        tasks_api::get_task(&self.config, group_id, pipeline_id, task_id).await
    }

    pub async fn list_tasks(
        &self,
        group_id: &str,
        pipeline_id: &str,
    ) -> Result<models::RespTaskList, Error<tasks_api::ListTasksError>> {
        tasks_api::list_tasks(&self.config, group_id, pipeline_id).await
    }

    pub async fn patch_task(
        &self,
        group_id: &str,
        pipeline_id: &str,
        task_id: &str,
        task: models::Task,
    ) -> Result<models::RespTask, Error<tasks_api::PatchTaskError>> {
        tasks_api::patch_task(&self.config, group_id, pipeline_id, task_id, task).await
    }
}

#[derive(Clone)]
pub struct UsersClient {
    config: Arc<configuration::Configuration>,
}

impl UsersClient {
    pub async fn add_group_user(
        &self,
        group_id: &str,
        req_group_user: models::ReqGroupUser,
    ) -> Result<models::RespResourceUrl, Error<users_api::AddGroupUserError>> {
        users_api::add_group_user(&self.config, group_id, req_group_user).await
    }

    pub async fn get_group_user(
        &self,
        group_id: &str,
        username: &str,
    ) -> Result<models::RespGroupUser, Error<users_api::GetGroupUserError>> {
        users_api::get_group_user(&self.config, group_id, username).await
    }

    pub async fn list_group_users(
        &self,
        group_id: &str,
    ) -> Result<models::RespGroupUserList, Error<users_api::ListGroupUsersError>> {
        users_api::list_group_users(&self.config, group_id).await
    }

    pub async fn remove_group_user(
        &self,
        group_id: &str,
        username: &str,
    ) -> Result<models::RespGroupUser, Error<users_api::RemoveGroupUserError>> {
        users_api::remove_group_user(&self.config, group_id, username).await
    }

    pub async fn update_group_user(
        &self,
        group_id: &str,
        username: &str,
        req_update_group_user: models::ReqUpdateGroupUser,
    ) -> Result<models::RespGroupUser, Error<users_api::UpdateGroupUserError>> {
        users_api::update_group_user(&self.config, group_id, username, req_update_group_user).await
    }
}
