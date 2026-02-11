use crate::apis::{configuration, events_api, general_api, subscriptions_api, test_api, Error};
use crate::models;
use http::header::{HeaderMap, HeaderValue};
use std::sync::Arc;

#[derive(Clone)]
pub struct TapisNotifications {
    config: Arc<configuration::Configuration>,
    pub events: EventsClient,
    pub general: GeneralClient,
    pub subscriptions: SubscriptionsClient,
    pub test: TestClient,
}

impl TapisNotifications {
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
            events: EventsClient {
                config: config.clone(),
            },
            general: GeneralClient {
                config: config.clone(),
            },
            subscriptions: SubscriptionsClient {
                config: config.clone(),
            },
            test: TestClient {
                config: config.clone(),
            },
        })
    }

    pub fn config(&self) -> &configuration::Configuration {
        &self.config
    }
}

#[derive(Clone)]
pub struct EventsClient {
    config: Arc<configuration::Configuration>,
}

impl EventsClient {
    pub async fn end_event_series(
        &self,
        event_series: models::EventSeries,
        tenant: Option<&str>,
    ) -> Result<models::RespBasic, Error<events_api::EndEventSeriesError>> {
        events_api::end_event_series(&self.config, event_series, tenant).await
    }

    pub async fn post_event(
        &self,
        event: models::Event,
        tenant: Option<&str>,
    ) -> Result<models::RespBasic, Error<events_api::PostEventError>> {
        events_api::post_event(&self.config, event, tenant).await
    }

    pub async fn publish_event(
        &self,
        event: models::Event,
        tenant: Option<&str>,
    ) -> Result<models::RespBasic, Error<events_api::PublishEventError>> {
        events_api::publish_event(&self.config, event, tenant).await
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

    pub async fn ready_check(
        &self,
    ) -> Result<models::RespBasic, Error<general_api::ReadyCheckError>> {
        general_api::ready_check(&self.config).await
    }
}

#[derive(Clone)]
pub struct SubscriptionsClient {
    config: Arc<configuration::Configuration>,
}

impl SubscriptionsClient {
    pub async fn delete_subscription_by_name(
        &self,
        name: &str,
        owned_by: Option<&str>,
    ) -> Result<models::RespChangeCount, Error<subscriptions_api::DeleteSubscriptionByNameError>>
    {
        subscriptions_api::delete_subscription_by_name(&self.config, name, owned_by).await
    }

    pub async fn delete_subscription_by_uuid(
        &self,
        uuid: &str,
    ) -> Result<models::RespChangeCount, Error<subscriptions_api::DeleteSubscriptionByUuidError>>
    {
        subscriptions_api::delete_subscription_by_uuid(&self.config, uuid).await
    }

    pub async fn delete_subscriptions_by_subject(
        &self,
        subject: &str,
        owned_by: Option<&str>,
        any_owner: Option<bool>,
    ) -> Result<models::RespChangeCount, Error<subscriptions_api::DeleteSubscriptionsBySubjectError>>
    {
        subscriptions_api::delete_subscriptions_by_subject(
            &self.config,
            subject,
            owned_by,
            any_owner,
        )
        .await
    }

    pub async fn disable_subscription(
        &self,
        name: &str,
        owned_by: Option<&str>,
    ) -> Result<models::RespChangeCount, Error<subscriptions_api::DisableSubscriptionError>> {
        subscriptions_api::disable_subscription(&self.config, name, owned_by).await
    }

    pub async fn enable_subscription(
        &self,
        name: &str,
        owned_by: Option<&str>,
    ) -> Result<models::RespChangeCount, Error<subscriptions_api::EnableSubscriptionError>> {
        subscriptions_api::enable_subscription(&self.config, name, owned_by).await
    }

    pub async fn get_subscription_by_name(
        &self,
        name: &str,
        select: Option<&str>,
        owned_by: Option<&str>,
    ) -> Result<models::RespSubscription, Error<subscriptions_api::GetSubscriptionByNameError>>
    {
        subscriptions_api::get_subscription_by_name(&self.config, name, select, owned_by).await
    }

    pub async fn get_subscription_by_uuid(
        &self,
        uuid: &str,
        select: Option<&str>,
    ) -> Result<models::RespSubscription, Error<subscriptions_api::GetSubscriptionByUuidError>>
    {
        subscriptions_api::get_subscription_by_uuid(&self.config, uuid, select).await
    }

    pub async fn get_subscriptions(
        &self,
        search: Option<&str>,
        limit: Option<i32>,
        order_by: Option<&str>,
        skip: Option<i32>,
        start_after: Option<&str>,
        compute_total: Option<bool>,
        select: Option<&str>,
        owned_by: Option<&str>,
        any_owner: Option<bool>,
    ) -> Result<models::RespSubscriptions, Error<subscriptions_api::GetSubscriptionsError>> {
        subscriptions_api::get_subscriptions(
            &self.config,
            search,
            limit,
            order_by,
            skip,
            start_after,
            compute_total,
            select,
            owned_by,
            any_owner,
        )
        .await
    }

    pub async fn is_enabled(
        &self,
        name: &str,
        owned_by: Option<&str>,
    ) -> Result<models::RespBoolean, Error<subscriptions_api::IsEnabledError>> {
        subscriptions_api::is_enabled(&self.config, name, owned_by).await
    }

    pub async fn patch_subscription_by_name(
        &self,
        name: &str,
        req_patch_subscription: models::ReqPatchSubscription,
        owned_by: Option<&str>,
    ) -> Result<models::RespResourceUrl, Error<subscriptions_api::PatchSubscriptionByNameError>>
    {
        subscriptions_api::patch_subscription_by_name(
            &self.config,
            name,
            req_patch_subscription,
            owned_by,
        )
        .await
    }

    pub async fn post_subscription(
        &self,
        req_post_subscription: models::ReqPostSubscription,
    ) -> Result<models::RespResourceUrl, Error<subscriptions_api::PostSubscriptionError>> {
        subscriptions_api::post_subscription(&self.config, req_post_subscription).await
    }

    pub async fn search_subscriptions_query_parameters(
        &self,
        free_form_parameter_name: Option<std::collections::HashMap<String, String>>,
        limit: Option<i32>,
        order_by: Option<&str>,
        skip: Option<i32>,
        start_after: Option<&str>,
        compute_total: Option<bool>,
        select: Option<&str>,
        owned_by: Option<&str>,
    ) -> Result<
        models::RespSubscriptions,
        Error<subscriptions_api::SearchSubscriptionsQueryParametersError>,
    > {
        subscriptions_api::search_subscriptions_query_parameters(
            &self.config,
            free_form_parameter_name,
            limit,
            order_by,
            skip,
            start_after,
            compute_total,
            select,
            owned_by,
        )
        .await
    }

    pub async fn search_subscriptions_request_body(
        &self,
        req_search_subscriptions: models::ReqSearchSubscriptions,
        limit: Option<i32>,
        order_by: Option<&str>,
        skip: Option<i32>,
        start_after: Option<&str>,
        compute_total: Option<bool>,
        select: Option<&str>,
        owned_by: Option<&str>,
    ) -> Result<
        models::RespSubscriptions,
        Error<subscriptions_api::SearchSubscriptionsRequestBodyError>,
    > {
        subscriptions_api::search_subscriptions_request_body(
            &self.config,
            req_search_subscriptions,
            limit,
            order_by,
            skip,
            start_after,
            compute_total,
            select,
            owned_by,
        )
        .await
    }

    pub async fn update_ttl(
        &self,
        name: &str,
        ttl_minutes: i32,
        owned_by: Option<&str>,
    ) -> Result<models::RespChangeCount, Error<subscriptions_api::UpdateTtlError>> {
        subscriptions_api::update_ttl(&self.config, name, ttl_minutes, owned_by).await
    }
}

#[derive(Clone)]
pub struct TestClient {
    config: Arc<configuration::Configuration>,
}

impl TestClient {
    pub async fn begin_test_sequence(
        &self,
        subscription_ttl: Option<i32>,
        number_of_events: Option<i32>,
        end_series: Option<bool>,
        body: Option<serde_json::Value>,
    ) -> Result<models::RespSubscription, Error<test_api::BeginTestSequenceError>> {
        test_api::begin_test_sequence(
            &self.config,
            subscription_ttl,
            number_of_events,
            end_series,
            body,
        )
        .await
    }

    pub async fn delete_test_sequence(
        &self,
        name: &str,
    ) -> Result<models::RespChangeCount, Error<test_api::DeleteTestSequenceError>> {
        test_api::delete_test_sequence(&self.config, name).await
    }

    pub async fn get_test_sequence(
        &self,
        name: &str,
    ) -> Result<models::RespTestSequence, Error<test_api::GetTestSequenceError>> {
        test_api::get_test_sequence(&self.config, name).await
    }

    pub async fn record_test_notification(
        &self,
        name: &str,
        notification: models::Notification,
    ) -> Result<models::RespBasic, Error<test_api::RecordTestNotificationError>> {
        test_api::record_test_notification(&self.config, name, notification).await
    }
}
