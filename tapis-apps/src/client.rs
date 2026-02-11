use crate::apis::{
    applications_api, configuration, general_api, permissions_api, sharing_api, Error,
};
use crate::models;
use http::header::{HeaderMap, HeaderValue};
use std::sync::Arc;

#[derive(Clone)]
pub struct TapisApps {
    config: Arc<configuration::Configuration>,
    pub applications: ApplicationsClient,
    pub general: GeneralClient,
    pub permissions: PermissionsClient,
    pub sharing: SharingClient,
}

impl TapisApps {
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
            applications: ApplicationsClient {
                config: config.clone(),
            },
            general: GeneralClient {
                config: config.clone(),
            },
            permissions: PermissionsClient {
                config: config.clone(),
            },
            sharing: SharingClient {
                config: config.clone(),
            },
        })
    }

    pub fn config(&self) -> &configuration::Configuration {
        &self.config
    }
}

#[derive(Clone)]
pub struct ApplicationsClient {
    config: Arc<configuration::Configuration>,
}

impl ApplicationsClient {
    pub async fn change_app_owner(
        &self,
        app_id: &str,
        user_name: &str,
    ) -> Result<models::RespChangeCount, Error<applications_api::ChangeAppOwnerError>> {
        applications_api::change_app_owner(&self.config, app_id, user_name).await
    }

    pub async fn create_app_version(
        &self,
        req_post_app: models::ReqPostApp,
    ) -> Result<models::RespResourceUrl, Error<applications_api::CreateAppVersionError>> {
        applications_api::create_app_version(&self.config, req_post_app).await
    }

    pub async fn delete_app(
        &self,
        app_id: &str,
    ) -> Result<models::RespChangeCount, Error<applications_api::DeleteAppError>> {
        applications_api::delete_app(&self.config, app_id).await
    }

    pub async fn disable_app(
        &self,
        app_id: &str,
    ) -> Result<models::RespChangeCount, Error<applications_api::DisableAppError>> {
        applications_api::disable_app(&self.config, app_id).await
    }

    pub async fn disable_app_version(
        &self,
        app_id: &str,
        app_version: &str,
    ) -> Result<models::RespChangeCount, Error<applications_api::DisableAppVersionError>> {
        applications_api::disable_app_version(&self.config, app_id, app_version).await
    }

    pub async fn enable_app(
        &self,
        app_id: &str,
    ) -> Result<models::RespChangeCount, Error<applications_api::EnableAppError>> {
        applications_api::enable_app(&self.config, app_id).await
    }

    pub async fn enable_app_version(
        &self,
        app_id: &str,
        app_version: &str,
    ) -> Result<models::RespChangeCount, Error<applications_api::EnableAppVersionError>> {
        applications_api::enable_app_version(&self.config, app_id, app_version).await
    }

    pub async fn get_app(
        &self,
        app_id: &str,
        app_version: &str,
        require_exec_perm: Option<bool>,
        impersonation_id: Option<&str>,
        select: Option<&str>,
        resource_tenant: Option<&str>,
    ) -> Result<models::RespApp, Error<applications_api::GetAppError>> {
        applications_api::get_app(
            &self.config,
            app_id,
            app_version,
            require_exec_perm,
            impersonation_id,
            select,
            resource_tenant,
        )
        .await
    }

    pub async fn get_app_latest_version(
        &self,
        app_id: &str,
        require_exec_perm: Option<bool>,
        select: Option<&str>,
        resource_tenant: Option<&str>,
        impersonation_id: Option<&str>,
    ) -> Result<models::RespApp, Error<applications_api::GetAppLatestVersionError>> {
        applications_api::get_app_latest_version(
            &self.config,
            app_id,
            require_exec_perm,
            select,
            resource_tenant,
            impersonation_id,
        )
        .await
    }

    pub async fn get_apps(
        &self,
        search: Option<&str>,
        list_type: Option<models::ListTypeEnum>,
        limit: Option<i32>,
        order_by: Option<&str>,
        skip: Option<i32>,
        start_after: Option<&str>,
        compute_total: Option<bool>,
        select: Option<&str>,
        show_deleted: Option<bool>,
        impersonation_id: Option<&str>,
    ) -> Result<models::RespApps, Error<applications_api::GetAppsError>> {
        applications_api::get_apps(
            &self.config,
            search,
            list_type,
            limit,
            order_by,
            skip,
            start_after,
            compute_total,
            select,
            show_deleted,
            impersonation_id,
        )
        .await
    }

    pub async fn get_history(
        &self,
        app_id: &str,
    ) -> Result<models::RespAppHistory, Error<applications_api::GetHistoryError>> {
        applications_api::get_history(&self.config, app_id).await
    }

    pub async fn is_enabled(
        &self,
        app_id: &str,
        version: Option<&str>,
    ) -> Result<models::RespBoolean, Error<applications_api::IsEnabledError>> {
        applications_api::is_enabled(&self.config, app_id, version).await
    }

    pub async fn lock_app(
        &self,
        app_id: &str,
        app_version: &str,
    ) -> Result<models::RespChangeCount, Error<applications_api::LockAppError>> {
        applications_api::lock_app(&self.config, app_id, app_version).await
    }

    pub async fn patch_app(
        &self,
        app_id: &str,
        app_version: &str,
        req_patch_app: models::ReqPatchApp,
    ) -> Result<models::RespResourceUrl, Error<applications_api::PatchAppError>> {
        applications_api::patch_app(&self.config, app_id, app_version, req_patch_app).await
    }

    pub async fn put_app(
        &self,
        app_id: &str,
        app_version: &str,
        req_put_app: models::ReqPutApp,
    ) -> Result<models::RespResourceUrl, Error<applications_api::PutAppError>> {
        applications_api::put_app(&self.config, app_id, app_version, req_put_app).await
    }

    pub async fn search_apps_query_parameters(
        &self,
        list_type: Option<models::ListTypeEnum>,
        limit: Option<i32>,
        order_by: Option<&str>,
        skip: Option<i32>,
        start_after: Option<&str>,
        compute_total: Option<bool>,
        select: Option<&str>,
    ) -> Result<models::RespApps, Error<applications_api::SearchAppsQueryParametersError>> {
        applications_api::search_apps_query_parameters(
            &self.config,
            list_type,
            limit,
            order_by,
            skip,
            start_after,
            compute_total,
            select,
        )
        .await
    }

    pub async fn search_apps_request_body(
        &self,
        req_search_apps: models::ReqSearchApps,
        list_type: Option<models::ListTypeEnum>,
        limit: Option<i32>,
        order_by: Option<&str>,
        skip: Option<i32>,
        start_after: Option<&str>,
        compute_total: Option<bool>,
        select: Option<&str>,
    ) -> Result<models::RespApps, Error<applications_api::SearchAppsRequestBodyError>> {
        applications_api::search_apps_request_body(
            &self.config,
            req_search_apps,
            list_type,
            limit,
            order_by,
            skip,
            start_after,
            compute_total,
            select,
        )
        .await
    }

    pub async fn undelete_app(
        &self,
        app_id: &str,
    ) -> Result<models::RespChangeCount, Error<applications_api::UndeleteAppError>> {
        applications_api::undelete_app(&self.config, app_id).await
    }

    pub async fn unlock_app(
        &self,
        app_id: &str,
        app_version: &str,
    ) -> Result<models::RespChangeCount, Error<applications_api::UnlockAppError>> {
        applications_api::unlock_app(&self.config, app_id, app_version).await
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
pub struct PermissionsClient {
    config: Arc<configuration::Configuration>,
}

impl PermissionsClient {
    pub async fn get_user_perms(
        &self,
        app_id: &str,
        user_name: &str,
    ) -> Result<models::RespNameArray, Error<permissions_api::GetUserPermsError>> {
        permissions_api::get_user_perms(&self.config, app_id, user_name).await
    }

    pub async fn grant_user_perms(
        &self,
        app_id: &str,
        user_name: &str,
        req_perms: models::ReqPerms,
    ) -> Result<models::RespBasic, Error<permissions_api::GrantUserPermsError>> {
        permissions_api::grant_user_perms(&self.config, app_id, user_name, req_perms).await
    }

    pub async fn revoke_user_perm(
        &self,
        app_id: &str,
        user_name: &str,
        permission: &str,
    ) -> Result<models::RespBasic, Error<permissions_api::RevokeUserPermError>> {
        permissions_api::revoke_user_perm(&self.config, app_id, user_name, permission).await
    }

    pub async fn revoke_user_perms(
        &self,
        app_id: &str,
        user_name: &str,
        req_perms: models::ReqPerms,
    ) -> Result<models::RespBasic, Error<permissions_api::RevokeUserPermsError>> {
        permissions_api::revoke_user_perms(&self.config, app_id, user_name, req_perms).await
    }
}

#[derive(Clone)]
pub struct SharingClient {
    config: Arc<configuration::Configuration>,
}

impl SharingClient {
    pub async fn get_share_info(
        &self,
        app_id: &str,
    ) -> Result<models::RespShareInfo, Error<sharing_api::GetShareInfoError>> {
        sharing_api::get_share_info(&self.config, app_id).await
    }

    pub async fn share_app(
        &self,
        app_id: &str,
        req_share_update: models::ReqShareUpdate,
    ) -> Result<models::RespBasic, Error<sharing_api::ShareAppError>> {
        sharing_api::share_app(&self.config, app_id, req_share_update).await
    }

    pub async fn share_app_public(
        &self,
        app_id: &str,
    ) -> Result<models::RespBasic, Error<sharing_api::ShareAppPublicError>> {
        sharing_api::share_app_public(&self.config, app_id).await
    }

    pub async fn un_share_app(
        &self,
        app_id: &str,
        req_share_update: models::ReqShareUpdate,
    ) -> Result<models::RespBasic, Error<sharing_api::UnShareAppError>> {
        sharing_api::un_share_app(&self.config, app_id, req_share_update).await
    }

    pub async fn un_share_app_public(
        &self,
        app_id: &str,
    ) -> Result<models::RespBasic, Error<sharing_api::UnShareAppPublicError>> {
        sharing_api::un_share_app_public(&self.config, app_id).await
    }
}
