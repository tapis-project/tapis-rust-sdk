use crate::apis::{
    Error, archives_api, cicd_api, configuration, etl_api, general_api, group_secrets_api,
    groups_api, identities_api, pipeline_archives_api, pipeline_locks_api, pipeline_runs_api,
    pipelines_api, secrets_api, task_executions_api, tasks_api, users_api,
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
