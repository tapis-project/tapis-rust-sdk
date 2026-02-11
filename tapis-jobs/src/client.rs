use crate::apis::{configuration, general_api, jobs_api, sharing_api, subscriptions_api, Error};
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
pub struct TapisJobs {
    config: Arc<configuration::Configuration>,
    pub general: GeneralClient,
    pub jobs: JobsClient,
    pub sharing: SharingClient,
    pub subscriptions: SubscriptionsClient,
}

impl TapisJobs {
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
            general: GeneralClient {
                config: config.clone(),
            },
            jobs: JobsClient {
                config: config.clone(),
            },
            sharing: SharingClient {
                config: config.clone(),
            },
            subscriptions: SubscriptionsClient {
                config: config.clone(),
            },
        })
    }

    pub fn config(&self) -> &configuration::Configuration {
        &self.config
    }
}

#[derive(Clone)]
pub struct GeneralClient {
    config: Arc<configuration::Configuration>,
}

impl GeneralClient {
    pub async fn check_health(
        &self,
    ) -> Result<models::RespProbe, Error<general_api::CheckHealthError>> {
        general_api::check_health(&self.config).await
    }

    pub async fn readycheck(
        &self,
    ) -> Result<models::RespProbe, Error<general_api::ReadycheckError>> {
        general_api::readycheck(&self.config).await
    }
}

#[derive(Clone)]
pub struct JobsClient {
    config: Arc<configuration::Configuration>,
}

impl JobsClient {
    pub async fn cancel_job(
        &self,
        job_uuid: &str,
        body: Option<serde_json::Value>,
    ) -> Result<models::RespCancelJob, Error<jobs_api::CancelJobError>> {
        jobs_api::cancel_job(&self.config, job_uuid, body).await
    }

    pub async fn get_job(
        &self,
        job_uuid: &str,
    ) -> Result<models::RespGetJob, Error<jobs_api::GetJobError>> {
        jobs_api::get_job(&self.config, job_uuid).await
    }

    pub async fn get_job_history(
        &self,
        job_uuid: &str,
        limit: Option<i32>,
        skip: Option<i32>,
    ) -> Result<models::RespJobHistory, Error<jobs_api::GetJobHistoryError>> {
        jobs_api::get_job_history(&self.config, job_uuid, limit, skip).await
    }

    pub async fn get_job_list(
        &self,
        limit: Option<i32>,
        skip: Option<i32>,
        start_after: Option<i32>,
        order_by: Option<&str>,
        compute_total: Option<bool>,
        list_type: Option<&str>,
    ) -> Result<models::RespGetJobList, Error<jobs_api::GetJobListError>> {
        jobs_api::get_job_list(
            &self.config,
            limit,
            skip,
            start_after,
            order_by,
            compute_total,
            list_type,
        )
        .await
    }

    pub async fn get_job_output_download(
        &self,
        job_uuid: &str,
        output_path: &str,
        compress: Option<bool>,
        format: Option<&str>,
        allow_if_running: Option<bool>,
    ) -> Result<reqwest::Response, Error<jobs_api::GetJobOutputDownloadError>> {
        jobs_api::get_job_output_download(
            &self.config,
            job_uuid,
            output_path,
            compress,
            format,
            allow_if_running,
        )
        .await
    }

    pub async fn get_job_output_list(
        &self,
        job_uuid: &str,
        output_path: &str,
        limit: Option<i32>,
        skip: Option<i32>,
        allow_if_running: Option<bool>,
    ) -> Result<models::RespGetJobOutputList, Error<jobs_api::GetJobOutputListError>> {
        jobs_api::get_job_output_list(
            &self.config,
            job_uuid,
            output_path,
            limit,
            skip,
            allow_if_running,
        )
        .await
    }

    pub async fn get_job_search_list(
        &self,
        limit: Option<i32>,
        skip: Option<i32>,
        start_after: Option<i32>,
        order_by: Option<&str>,
        compute_total: Option<bool>,
        select: Option<&str>,
        list_type: Option<&str>,
    ) -> Result<models::RespJobSearchAllAttributes, Error<jobs_api::GetJobSearchListError>> {
        jobs_api::get_job_search_list(
            &self.config,
            limit,
            skip,
            start_after,
            order_by,
            compute_total,
            select,
            list_type,
        )
        .await
    }

    pub async fn get_job_search_list_by_post_sql_str(
        &self,
        limit: Option<i32>,
        skip: Option<i32>,
        start_after: Option<i32>,
        order_by: Option<&str>,
        compute_total: Option<bool>,
        select: Option<&str>,
        list_type: Option<&str>,
        body: Option<serde_json::Value>,
    ) -> Result<
        models::RespJobSearchAllAttributes,
        Error<jobs_api::GetJobSearchListByPostSqlStrError>,
    > {
        jobs_api::get_job_search_list_by_post_sql_str(
            &self.config,
            limit,
            skip,
            start_after,
            order_by,
            compute_total,
            select,
            list_type,
            body,
        )
        .await
    }

    pub async fn get_job_status(
        &self,
        job_uuid: &str,
    ) -> Result<models::RespGetJobStatus, Error<jobs_api::GetJobStatusError>> {
        jobs_api::get_job_status(&self.config, job_uuid).await
    }

    pub async fn get_resubmit_request_json(
        &self,
        job_uuid: &str,
    ) -> Result<models::RespGetResubmit, Error<jobs_api::GetResubmitRequestJsonError>> {
        jobs_api::get_resubmit_request_json(&self.config, job_uuid).await
    }

    pub async fn hide_job(
        &self,
        job_uuid: &str,
        body: Option<serde_json::Value>,
    ) -> Result<models::RespHideJob, Error<jobs_api::HideJobError>> {
        jobs_api::hide_job(&self.config, job_uuid, body).await
    }

    pub async fn patch_job_annotations(
        &self,
        job_uuid: &str,
        req_job_annotation: models::ReqJobAnnotation,
    ) -> Result<models::RespJobAnnotations, Error<jobs_api::PatchJobAnnotationsError>> {
        jobs_api::patch_job_annotations(&self.config, job_uuid, req_job_annotation).await
    }

    pub async fn put_job_annotations(
        &self,
        job_uuid: &str,
        req_job_annotation: models::ReqJobAnnotation,
    ) -> Result<models::RespJobAnnotations, Error<jobs_api::PutJobAnnotationsError>> {
        jobs_api::put_job_annotations(&self.config, job_uuid, req_job_annotation).await
    }

    pub async fn resubmit_job(
        &self,
        job_uuid: &str,
        body: Option<serde_json::Value>,
    ) -> Result<models::RespSubmitJob, Error<jobs_api::ResubmitJobError>> {
        jobs_api::resubmit_job(&self.config, job_uuid, body).await
    }

    pub async fn send_event(
        &self,
        job_uuid: &str,
        req_user_event: models::ReqUserEvent,
    ) -> Result<models::RespBasic, Error<jobs_api::SendEventError>> {
        jobs_api::send_event(&self.config, job_uuid, req_user_event).await
    }

    pub async fn submit_job(
        &self,
        req_submit_job: models::ReqSubmitJob,
    ) -> Result<models::RespSubmitJob, Error<jobs_api::SubmitJobError>> {
        jobs_api::submit_job(&self.config, req_submit_job).await
    }

    pub async fn unhide_job(
        &self,
        job_uuid: &str,
        body: Option<serde_json::Value>,
    ) -> Result<models::RespHideJob, Error<jobs_api::UnhideJobError>> {
        jobs_api::unhide_job(&self.config, job_uuid, body).await
    }
}

#[derive(Clone)]
pub struct SharingClient {
    config: Arc<configuration::Configuration>,
}

impl SharingClient {
    pub async fn delete_job_share(
        &self,
        job_uuid: &str,
        user: &str,
    ) -> Result<models::RespUnShareJob, Error<sharing_api::DeleteJobShareError>> {
        sharing_api::delete_job_share(&self.config, job_uuid, user).await
    }

    pub async fn get_job_share(
        &self,
        job_uuid: &str,
        limit: Option<i32>,
        skip: Option<i32>,
    ) -> Result<models::RespGetJobShareList, Error<sharing_api::GetJobShareError>> {
        sharing_api::get_job_share(&self.config, job_uuid, limit, skip).await
    }

    pub async fn share_job(
        &self,
        job_uuid: &str,
        req_share_job: models::ReqShareJob,
    ) -> Result<models::RespShareJob, Error<sharing_api::ShareJobError>> {
        sharing_api::share_job(&self.config, job_uuid, req_share_job).await
    }
}

#[derive(Clone)]
pub struct SubscriptionsClient {
    config: Arc<configuration::Configuration>,
}

impl SubscriptionsClient {
    pub async fn delete_subscriptions(
        &self,
        uuid: &str,
    ) -> Result<models::ResultChangeCount, Error<subscriptions_api::DeleteSubscriptionsError>> {
        subscriptions_api::delete_subscriptions(&self.config, uuid).await
    }

    pub async fn get_subscriptions(
        &self,
        job_uuid: &str,
        limit: Option<i32>,
        skip: Option<i32>,
    ) -> Result<models::RespGetSubscriptions, Error<subscriptions_api::GetSubscriptionsError>> {
        subscriptions_api::get_subscriptions(&self.config, job_uuid, limit, skip).await
    }

    pub async fn subscribe(
        &self,
        job_uuid: &str,
        req_subscribe: models::ReqSubscribe,
    ) -> Result<models::RespResourceUrl, Error<subscriptions_api::SubscribeError>> {
        subscriptions_api::subscribe(&self.config, job_uuid, req_subscribe).await
    }
}
