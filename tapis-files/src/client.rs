use crate::apis::{configuration, Error, content_api, file_operations_api, general_api, permissions_api, post_its_api, sharing_api, transfers_api};
use crate::models;
use http::header::{HeaderMap, HeaderValue};
use std::sync::Arc;

#[derive(Clone)]
pub struct TapisFiles {
    config: Arc<configuration::Configuration>,
    pub content: ContentClient,
    pub file_operations: FileOperationsClient,
    pub general: GeneralClient,
    pub permissions: PermissionsClient,
    pub post_its: PostItsClient,
    pub sharing: SharingClient,
    pub transfers: TransfersClient,
}

impl TapisFiles {
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
            content: ContentClient { config: config.clone() },
            file_operations: FileOperationsClient { config: config.clone() },
            general: GeneralClient { config: config.clone() },
            permissions: PermissionsClient { config: config.clone() },
            post_its: PostItsClient { config: config.clone() },
            sharing: SharingClient { config: config.clone() },
            transfers: TransfersClient { config: config.clone() },
        })
    }

    pub fn config(&self) -> &configuration::Configuration {
        &self.config
    }
}

#[derive(Clone)]
pub struct ContentClient {
    config: Arc<configuration::Configuration>,
}

impl ContentClient {
    pub async fn get_contents(&self, system_id: &str, path: &str, range: Option<models::HeaderByteRange>, zip: Option<bool>, more: Option<i64>, impersonation_id: Option<&str>, shared_ctx: Option<&str>) -> Result<(), Error<content_api::GetContentsError>> {
        content_api::get_contents(&self.config, system_id, path, range, zip, more, impersonation_id, shared_ctx).await
    }

}

#[derive(Clone)]
pub struct FileOperationsClient {
    config: Arc<configuration::Configuration>,
}

impl FileOperationsClient {
    pub async fn delete(&self, system_id: &str, path: &str) -> Result<models::FileStringResponse, Error<file_operations_api::DeleteError>> {
        file_operations_api::delete(&self.config, system_id, path).await
    }

    pub async fn get_facl(&self, system_id: &str, path: &str) -> Result<models::NativeLinuxGetFaclResponse, Error<file_operations_api::GetFaclError>> {
        file_operations_api::get_facl(&self.config, system_id, path).await
    }

    pub async fn get_stat_info(&self, system_id: &str, path: &str, follow_links: Option<bool>) -> Result<models::FileStatInfoResponse, Error<file_operations_api::GetStatInfoError>> {
        file_operations_api::get_stat_info(&self.config, system_id, path, follow_links).await
    }

    pub async fn insert(&self, system_id: &str, path: &str, file: std::path::PathBuf) -> Result<models::FileStringResponse, Error<file_operations_api::InsertError>> {
        file_operations_api::insert(&self.config, system_id, path, file).await
    }

    pub async fn list_files(&self, system_id: &str, path: &str, pattern: Option<&str>, limit: Option<i32>, offset: Option<i64>, recurse: Option<bool>, impersonation_id: Option<&str>, shared_ctx: Option<&str>) -> Result<models::FileListingResponse, Error<file_operations_api::ListFilesError>> {
        file_operations_api::list_files(&self.config, system_id, path, pattern, limit, offset, recurse, impersonation_id, shared_ctx).await
    }

    pub async fn mkdir(&self, system_id: &str, shared_ctx: Option<&str>, mkdir_request: Option<models::MkdirRequest>) -> Result<models::FileStringResponse, Error<file_operations_api::MkdirError>> {
        file_operations_api::mkdir(&self.config, system_id, shared_ctx, mkdir_request).await
    }

    pub async fn move_copy(&self, system_id: &str, path: &str, move_copy_request: Option<models::MoveCopyRequest>) -> Result<models::FileStringResponse, Error<file_operations_api::MoveCopyError>> {
        file_operations_api::move_copy(&self.config, system_id, path, move_copy_request).await
    }

    pub async fn run_linux_native_op(&self, system_id: &str, path: &str, recursive: Option<bool>, native_linux_op_request: Option<models::NativeLinuxOpRequest>) -> Result<models::NativeLinuxOpResultResponse, Error<file_operations_api::RunLinuxNativeOpError>> {
        file_operations_api::run_linux_native_op(&self.config, system_id, path, recursive, native_linux_op_request).await
    }

    pub async fn set_facl(&self, system_id: &str, path: &str, native_linux_set_facl_request: models::NativeLinuxSetFaclRequest) -> Result<models::NativeLinuxSetFaclResponse, Error<file_operations_api::SetFaclError>> {
        file_operations_api::set_facl(&self.config, system_id, path, native_linux_set_facl_request).await
    }

}

#[derive(Clone)]
pub struct GeneralClient {
    config: Arc<configuration::Configuration>,
}

impl GeneralClient {
    pub async fn health_check(&self) -> Result<models::RespBasic, Error<general_api::HealthCheckError>> {
        general_api::health_check(&self.config).await
    }

    pub async fn ready_check(&self) -> Result<models::RespBasic, Error<general_api::ReadyCheckError>> {
        general_api::ready_check(&self.config).await
    }

}

#[derive(Clone)]
pub struct PermissionsClient {
    config: Arc<configuration::Configuration>,
}

impl PermissionsClient {
    pub async fn delete_permissions(&self, system_id: &str, path: &str, username: &str) -> Result<models::StringResponse, Error<permissions_api::DeletePermissionsError>> {
        permissions_api::delete_permissions(&self.config, system_id, path, username).await
    }

    pub async fn get_permissions(&self, system_id: &str, path: &str, username: Option<&str>) -> Result<models::FilePermissionResponse, Error<permissions_api::GetPermissionsError>> {
        permissions_api::get_permissions(&self.config, system_id, path, username).await
    }

    pub async fn grant_permissions(&self, system_id: &str, path: &str, create_permission_request: models::CreatePermissionRequest) -> Result<models::FilePermissionResponse, Error<permissions_api::GrantPermissionsError>> {
        permissions_api::grant_permissions(&self.config, system_id, path, create_permission_request).await
    }

}

#[derive(Clone)]
pub struct PostItsClient {
    config: Arc<configuration::Configuration>,
}

impl PostItsClient {
    pub async fn create_post_it(&self, system_id: &str, path: &str, create_post_it_request: models::CreatePostItRequest) -> Result<models::PostItResponse, Error<post_its_api::CreatePostItError>> {
        post_its_api::create_post_it(&self.config, system_id, path, create_post_it_request).await
    }

    pub async fn delete_post_it(&self, postit_id: &str) -> Result<models::RespChangeCount, Error<post_its_api::DeletePostItError>> {
        post_its_api::delete_post_it(&self.config, postit_id).await
    }

    pub async fn get_post_it(&self, postit_id: &str) -> Result<models::PostItResponse, Error<post_its_api::GetPostItError>> {
        post_its_api::get_post_it(&self.config, postit_id).await
    }

    pub async fn list_post_its(&self, list_type: Option<models::ListTypeEnum>, limit: Option<i32>, order_by: Option<&str>, skip: Option<i32>, start_after: Option<&str>, select: Option<&str>) -> Result<models::PostItListResponse, Error<post_its_api::ListPostItsError>> {
        post_its_api::list_post_its(&self.config, list_type, limit, order_by, skip, start_after, select).await
    }

    pub async fn redeem_post_it(&self, postit_id: &str, zip: Option<bool>, download: Option<bool>) -> Result<(), Error<post_its_api::RedeemPostItError>> {
        post_its_api::redeem_post_it(&self.config, postit_id, zip, download).await
    }

    pub async fn update_post_it(&self, postit_id: &str, update_post_it_request: models::UpdatePostItRequest) -> Result<models::PostItResponse, Error<post_its_api::UpdatePostItError>> {
        post_its_api::update_post_it(&self.config, postit_id, update_post_it_request).await
    }

}

#[derive(Clone)]
pub struct SharingClient {
    config: Arc<configuration::Configuration>,
}

impl SharingClient {
    pub async fn get_share_info(&self, system_id: &str, path: &str) -> Result<models::RespShareInfo, Error<sharing_api::GetShareInfoError>> {
        sharing_api::get_share_info(&self.config, system_id, path).await
    }

    pub async fn share_path(&self, system_id: &str, path: &str, req_share_update: models::ReqShareUpdate) -> Result<models::RespBasic, Error<sharing_api::SharePathError>> {
        sharing_api::share_path(&self.config, system_id, path, req_share_update).await
    }

    pub async fn share_path_public(&self, system_id: &str, path: &str) -> Result<models::RespBasic, Error<sharing_api::SharePathPublicError>> {
        sharing_api::share_path_public(&self.config, system_id, path).await
    }

    pub async fn un_share_path(&self, system_id: &str, path: &str, req_share_update: models::ReqShareUpdate) -> Result<models::RespBasic, Error<sharing_api::UnSharePathError>> {
        sharing_api::un_share_path(&self.config, system_id, path, req_share_update).await
    }

    pub async fn un_share_path_all(&self, system_id: &str, path: &str, recurse: Option<bool>) -> Result<models::RespBasic, Error<sharing_api::UnSharePathAllError>> {
        sharing_api::un_share_path_all(&self.config, system_id, path, recurse).await
    }

    pub async fn un_share_path_public(&self, system_id: &str, path: &str) -> Result<models::RespBasic, Error<sharing_api::UnSharePathPublicError>> {
        sharing_api::un_share_path_public(&self.config, system_id, path).await
    }

}

#[derive(Clone)]
pub struct TransfersClient {
    config: Arc<configuration::Configuration>,
}

impl TransfersClient {
    pub async fn cancel_transfer_task(&self, transfer_task_id: &str) -> Result<models::StringResponse, Error<transfers_api::CancelTransferTaskError>> {
        transfers_api::cancel_transfer_task(&self.config, transfer_task_id).await
    }

    pub async fn create_transfer_task(&self, req_transfer: models::ReqTransfer) -> Result<models::TransferTaskResponse, Error<transfers_api::CreateTransferTaskError>> {
        transfers_api::create_transfer_task(&self.config, req_transfer).await
    }

    pub async fn get_recent_transfer_tasks(&self, limit: Option<i32>, offset: Option<i32>) -> Result<models::TransferTaskListResponse, Error<transfers_api::GetRecentTransferTasksError>> {
        transfers_api::get_recent_transfer_tasks(&self.config, limit, offset).await
    }

    pub async fn get_transfer_task(&self, transfer_task_id: &str, include_summary: Option<bool>, impersonation_id: Option<&str>) -> Result<models::TransferTaskResponse, Error<transfers_api::GetTransferTaskError>> {
        transfers_api::get_transfer_task(&self.config, transfer_task_id, include_summary, impersonation_id).await
    }

    pub async fn get_transfer_task_details(&self, transfer_task_id: &str, impersonation_id: Option<&str>) -> Result<models::TransferTaskResponse, Error<transfers_api::GetTransferTaskDetailsError>> {
        transfers_api::get_transfer_task_details(&self.config, transfer_task_id, impersonation_id).await
    }

}

