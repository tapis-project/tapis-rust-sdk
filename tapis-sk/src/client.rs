use crate::apis::{configuration, Error, admin_api, general_api, role_api, share_api, user_api, vault_api};
use crate::models;
use http::header::{HeaderMap, HeaderValue};
use std::sync::Arc;

#[derive(Clone)]
pub struct TapisSk {
    config: Arc<configuration::Configuration>,
    pub admin: AdminClient,
    pub general: GeneralClient,
    pub role: RoleClient,
    pub share: ShareClient,
    pub user: UserClient,
    pub vault: VaultClient,
}

impl TapisSk {
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
            admin: AdminClient { config: config.clone() },
            general: GeneralClient { config: config.clone() },
            role: RoleClient { config: config.clone() },
            share: ShareClient { config: config.clone() },
            user: UserClient { config: config.clone() },
            vault: VaultClient { config: config.clone() },
        })
    }

    pub fn config(&self) -> &configuration::Configuration {
        &self.config
    }
}

#[derive(Clone)]
pub struct AdminClient {
    config: Arc<configuration::Configuration>,
}

impl AdminClient {
    pub async fn reinitialize_site(&self) -> Result<models::RespAuthorized, Error<admin_api::ReinitializeSiteError>> {
        admin_api::reinitialize_site(&self.config).await
    }

}

#[derive(Clone)]
pub struct GeneralClient {
    config: Arc<configuration::Configuration>,
}

impl GeneralClient {
    pub async fn check_health(&self) -> Result<models::RespProbe, Error<general_api::CheckHealthError>> {
        general_api::check_health(&self.config).await
    }

    pub async fn ready(&self) -> Result<models::RespProbe, Error<general_api::ReadyError>> {
        general_api::ready(&self.config).await
    }

    pub async fn readycheck(&self) -> Result<models::RespProbe, Error<general_api::ReadycheckError>> {
        general_api::readycheck(&self.config).await
    }

    pub async fn say_hello(&self) -> Result<models::RespBasic, Error<general_api::SayHelloError>> {
        general_api::say_hello(&self.config).await
    }

}

#[derive(Clone)]
pub struct RoleClient {
    config: Arc<configuration::Configuration>,
}

impl RoleClient {
    pub async fn add_child_role(&self, req_add_child_role: models::ReqAddChildRole) -> Result<models::RespChangeCount, Error<role_api::AddChildRoleError>> {
        role_api::add_child_role(&self.config, req_add_child_role).await
    }

    pub async fn add_role_permission(&self, req_add_role_permission: models::ReqAddRolePermission) -> Result<models::RespChangeCount, Error<role_api::AddRolePermissionError>> {
        role_api::add_role_permission(&self.config, req_add_role_permission).await
    }

    pub async fn create_role(&self, req_create_role: models::ReqCreateRole) -> Result<models::RespResourceUrl, Error<role_api::CreateRoleError>> {
        role_api::create_role(&self.config, req_create_role).await
    }

    pub async fn delete_role_by_name(&self, role_name: &str, tenant: Option<&str>, role_type: Option<models::RoleTypeEnum>) -> Result<models::RespChangeCount, Error<role_api::DeleteRoleByNameError>> {
        role_api::delete_role_by_name(&self.config, role_name, tenant, role_type).await
    }

    pub async fn get_default_user_role(&self, user: &str) -> Result<models::RespName, Error<role_api::GetDefaultUserRoleError>> {
        role_api::get_default_user_role(&self.config, user).await
    }

    pub async fn get_role_by_name(&self, role_name: &str, tenant: Option<&str>, role_type: Option<models::RoleTypeEnum>) -> Result<models::RespRole, Error<role_api::GetRoleByNameError>> {
        role_api::get_role_by_name(&self.config, role_name, tenant, role_type).await
    }

    pub async fn get_role_names(&self, tenant: Option<&str>, role_type: Option<models::RoleTypeEnum>) -> Result<models::RespNameArray, Error<role_api::GetRoleNamesError>> {
        role_api::get_role_names(&self.config, tenant, role_type).await
    }

    pub async fn get_role_permissions(&self, role_name: &str, tenant: Option<&str>, immediate: Option<bool>) -> Result<models::RespNameArray, Error<role_api::GetRolePermissionsError>> {
        role_api::get_role_permissions(&self.config, role_name, tenant, immediate).await
    }

    pub async fn preview_path_prefix(&self, req_preview_path_prefix: models::ReqPreviewPathPrefix) -> Result<models::RespPathPrefixes, Error<role_api::PreviewPathPrefixError>> {
        role_api::preview_path_prefix(&self.config, req_preview_path_prefix).await
    }

    pub async fn remove_child_role(&self, req_remove_child_role: models::ReqRemoveChildRole) -> Result<models::RespChangeCount, Error<role_api::RemoveChildRoleError>> {
        role_api::remove_child_role(&self.config, req_remove_child_role).await
    }

    pub async fn remove_path_permission_from_all_roles(&self, req_remove_permission_from_all_roles: models::ReqRemovePermissionFromAllRoles) -> Result<models::RespChangeCount, Error<role_api::RemovePathPermissionFromAllRolesError>> {
        role_api::remove_path_permission_from_all_roles(&self.config, req_remove_permission_from_all_roles).await
    }

    pub async fn remove_permission_from_all_roles(&self, req_remove_permission_from_all_roles: models::ReqRemovePermissionFromAllRoles) -> Result<models::RespChangeCount, Error<role_api::RemovePermissionFromAllRolesError>> {
        role_api::remove_permission_from_all_roles(&self.config, req_remove_permission_from_all_roles).await
    }

    pub async fn remove_role_permission(&self, req_remove_role_permission: models::ReqRemoveRolePermission) -> Result<models::RespChangeCount, Error<role_api::RemoveRolePermissionError>> {
        role_api::remove_role_permission(&self.config, req_remove_role_permission).await
    }

    pub async fn replace_path_prefix(&self, req_replace_path_prefix: models::ReqReplacePathPrefix) -> Result<models::RespChangeCount, Error<role_api::ReplacePathPrefixError>> {
        role_api::replace_path_prefix(&self.config, req_replace_path_prefix).await
    }

    pub async fn role_permits(&self, role_name: &str, req_role_permits: models::ReqRolePermits, immediate: Option<bool>) -> Result<models::RespAuthorized, Error<role_api::RolePermitsError>> {
        role_api::role_permits(&self.config, role_name, req_role_permits, immediate).await
    }

    pub async fn update_role_description(&self, role_name: &str, req_update_role_description: models::ReqUpdateRoleDescription) -> Result<models::RespBasic, Error<role_api::UpdateRoleDescriptionError>> {
        role_api::update_role_description(&self.config, role_name, req_update_role_description).await
    }

    pub async fn update_role_name(&self, role_name: &str, req_update_role_name: models::ReqUpdateRoleName) -> Result<models::RespBasic, Error<role_api::UpdateRoleNameError>> {
        role_api::update_role_name(&self.config, role_name, req_update_role_name).await
    }

    pub async fn update_role_owner(&self, role_name: &str, req_update_role_owner: models::ReqUpdateRoleOwner) -> Result<models::RespBasic, Error<role_api::UpdateRoleOwnerError>> {
        role_api::update_role_owner(&self.config, role_name, req_update_role_owner).await
    }

}

#[derive(Clone)]
pub struct ShareClient {
    config: Arc<configuration::Configuration>,
}

impl ShareClient {
    pub async fn delete_share(&self, grantor: Option<&str>, grantee: Option<&str>, tenant: Option<&str>, resource_type: Option<&str>, resource_id1: Option<&str>, resource_id2: Option<&str>, privilege: Option<&str>) -> Result<models::RespChangeCount, Error<share_api::DeleteShareError>> {
        share_api::delete_share(&self.config, grantor, grantee, tenant, resource_type, resource_id1, resource_id2, privilege).await
    }

    pub async fn delete_share_by_id(&self, id: i32, tenant: Option<&str>) -> Result<models::RespChangeCount, Error<share_api::DeleteShareByIdError>> {
        share_api::delete_share_by_id(&self.config, id, tenant).await
    }

    pub async fn get_share(&self, id: i32, tenant: Option<&str>) -> Result<models::RespShare, Error<share_api::GetShareError>> {
        share_api::get_share(&self.config, id, tenant).await
    }

    pub async fn get_shares(&self, grantor: Option<&str>, grantee: Option<&str>, tenant: Option<&str>, resource_type: Option<&str>, resource_id1: Option<&str>, resource_id2: Option<&str>, privilege: Option<&str>, created_by: Option<&str>, created_by_tenant: Option<&str>, include_public_grantees: Option<bool>, require_null_id2: Option<bool>, id: Option<i32>) -> Result<models::RespShareList, Error<share_api::GetSharesError>> {
        share_api::get_shares(&self.config, grantor, grantee, tenant, resource_type, resource_id1, resource_id2, privilege, created_by, created_by_tenant, include_public_grantees, require_null_id2, id).await
    }

    pub async fn has_privilege(&self, grantee: Option<&str>, tenant: Option<&str>, resource_type: Option<&str>, resource_id1: Option<&str>, resource_id2: Option<&str>, privilege: Option<&str>, exclude_public: Option<bool>, exclude_public_no_authn: Option<bool>) -> Result<models::RespBoolean, Error<share_api::HasPrivilegeError>> {
        share_api::has_privilege(&self.config, grantee, tenant, resource_type, resource_id1, resource_id2, privilege, exclude_public, exclude_public_no_authn).await
    }

    pub async fn share_resource(&self, req_share_resource: models::ReqShareResource) -> Result<models::RespResourceUrl, Error<share_api::ShareResourceError>> {
        share_api::share_resource(&self.config, req_share_resource).await
    }

}

#[derive(Clone)]
pub struct UserClient {
    config: Arc<configuration::Configuration>,
}

impl UserClient {
    pub async fn get_admins(&self, tenant: &str) -> Result<models::RespNameArray, Error<user_api::GetAdminsError>> {
        user_api::get_admins(&self.config, tenant).await
    }

    pub async fn get_default_user_role1(&self, user: &str) -> Result<models::RespName, Error<user_api::GetDefaultUserRole1Error>> {
        user_api::get_default_user_role1(&self.config, user).await
    }

    pub async fn get_user_names(&self, tenant: Option<&str>) -> Result<models::RespNameArray, Error<user_api::GetUserNamesError>> {
        user_api::get_user_names(&self.config, tenant).await
    }

    pub async fn get_user_perms(&self, user: &str, tenant: Option<&str>, implies: Option<&str>, implied_by: Option<&str>) -> Result<models::RespNameArray, Error<user_api::GetUserPermsError>> {
        user_api::get_user_perms(&self.config, user, tenant, implies, implied_by).await
    }

    pub async fn get_user_roles(&self, user: &str, tenant: Option<&str>) -> Result<models::RespNameArray, Error<user_api::GetUserRolesError>> {
        user_api::get_user_roles(&self.config, user, tenant).await
    }

    pub async fn get_users_with_permission(&self, perm_spec: &str, tenant: Option<&str>) -> Result<models::RespNameArray, Error<user_api::GetUsersWithPermissionError>> {
        user_api::get_users_with_permission(&self.config, perm_spec, tenant).await
    }

    pub async fn get_users_with_role(&self, role_name: &str, tenant: Option<&str>, role_type: Option<models::RoleTypeEnum>) -> Result<models::RespNameArray, Error<user_api::GetUsersWithRoleError>> {
        user_api::get_users_with_role(&self.config, role_name, tenant, role_type).await
    }

    pub async fn grant_role(&self, req_grant_role: models::ReqGrantRole) -> Result<models::RespChangeCount, Error<user_api::GrantRoleError>> {
        user_api::grant_role(&self.config, req_grant_role).await
    }

    pub async fn grant_role_with_permission(&self, req_grant_role_with_permission: models::ReqGrantRoleWithPermission) -> Result<models::RespChangeCount, Error<user_api::GrantRoleWithPermissionError>> {
        user_api::grant_role_with_permission(&self.config, req_grant_role_with_permission).await
    }

    pub async fn grant_user_permission(&self, req_grant_user_permission: models::ReqGrantUserPermission) -> Result<models::RespChangeCount, Error<user_api::GrantUserPermissionError>> {
        user_api::grant_user_permission(&self.config, req_grant_user_permission).await
    }

    pub async fn has_role(&self, req_user_has_role: models::ReqUserHasRole) -> Result<models::RespAuthorized, Error<user_api::HasRoleError>> {
        user_api::has_role(&self.config, req_user_has_role).await
    }

    pub async fn has_role_all(&self, req_user_has_role_multi: models::ReqUserHasRoleMulti) -> Result<models::RespAuthorized, Error<user_api::HasRoleAllError>> {
        user_api::has_role_all(&self.config, req_user_has_role_multi).await
    }

    pub async fn has_role_any(&self, req_user_has_role_multi: models::ReqUserHasRoleMulti) -> Result<models::RespAuthorized, Error<user_api::HasRoleAnyError>> {
        user_api::has_role_any(&self.config, req_user_has_role_multi).await
    }

    pub async fn is_admin(&self, req_user_is_admin: models::ReqUserIsAdmin) -> Result<models::RespAuthorized, Error<user_api::IsAdminError>> {
        user_api::is_admin(&self.config, req_user_is_admin).await
    }

    pub async fn is_permitted(&self, req_user_is_permitted: models::ReqUserIsPermitted) -> Result<models::RespAuthorized, Error<user_api::IsPermittedError>> {
        user_api::is_permitted(&self.config, req_user_is_permitted).await
    }

    pub async fn is_permitted_all(&self, req_user_is_permitted_multi: models::ReqUserIsPermittedMulti) -> Result<models::RespAuthorized, Error<user_api::IsPermittedAllError>> {
        user_api::is_permitted_all(&self.config, req_user_is_permitted_multi).await
    }

    pub async fn is_permitted_any(&self, req_user_is_permitted_multi: models::ReqUserIsPermittedMulti) -> Result<models::RespAuthorized, Error<user_api::IsPermittedAnyError>> {
        user_api::is_permitted_any(&self.config, req_user_is_permitted_multi).await
    }

    pub async fn revoke_role(&self, req_revoke_role: models::ReqRevokeRole) -> Result<models::RespChangeCount, Error<user_api::RevokeRoleError>> {
        user_api::revoke_role(&self.config, req_revoke_role).await
    }

    pub async fn revoke_user_permission(&self, req_revoke_user_permission: models::ReqRevokeUserPermission) -> Result<models::RespChangeCount, Error<user_api::RevokeUserPermissionError>> {
        user_api::revoke_user_permission(&self.config, req_revoke_user_permission).await
    }

}

#[derive(Clone)]
pub struct VaultClient {
    config: Arc<configuration::Configuration>,
}

impl VaultClient {
    pub async fn delete_secret(&self, secret_type: &str, secret_name: &str, req_versions: models::ReqVersions, sysid: Option<&str>, sysuser: Option<&str>, keytype: Option<&str>, dbhost: Option<&str>, dbname: Option<&str>, dbservice: Option<&str>) -> Result<models::RespVersions, Error<vault_api::DeleteSecretError>> {
        vault_api::delete_secret(&self.config, secret_type, secret_name, req_versions, sysid, sysuser, keytype, dbhost, dbname, dbservice).await
    }

    pub async fn destroy_secret(&self, secret_type: &str, secret_name: &str, req_versions: models::ReqVersions, sysid: Option<&str>, sysuser: Option<&str>, keytype: Option<&str>, dbhost: Option<&str>, dbname: Option<&str>, dbservice: Option<&str>) -> Result<models::RespVersions, Error<vault_api::DestroySecretError>> {
        vault_api::destroy_secret(&self.config, secret_type, secret_name, req_versions, sysid, sysuser, keytype, dbhost, dbname, dbservice).await
    }

    pub async fn destroy_secret_meta(&self, secret_type: &str, secret_name: &str, tenant: Option<&str>, user: Option<&str>, sysid: Option<&str>, sysuser: Option<&str>, keytype: Option<&str>, dbhost: Option<&str>, dbname: Option<&str>, dbservice: Option<&str>) -> Result<models::RespBasic, Error<vault_api::DestroySecretMetaError>> {
        vault_api::destroy_secret_meta(&self.config, secret_type, secret_name, tenant, user, sysid, sysuser, keytype, dbhost, dbname, dbservice).await
    }

    pub async fn list_secret_meta(&self, secret_type: &str, tenant: Option<&str>, user: Option<&str>, sysid: Option<&str>, sysuser: Option<&str>, keytype: Option<&str>, dbhost: Option<&str>, dbname: Option<&str>, dbservice: Option<&str>) -> Result<models::RespSecretList, Error<vault_api::ListSecretMetaError>> {
        vault_api::list_secret_meta(&self.config, secret_type, tenant, user, sysid, sysuser, keytype, dbhost, dbname, dbservice).await
    }

    pub async fn read_secret(&self, secret_type: &str, secret_name: &str, tenant: Option<&str>, user: Option<&str>, version: Option<i32>, sysid: Option<&str>, sysuser: Option<&str>, keytype: Option<&str>, dbhost: Option<&str>, dbname: Option<&str>, dbservice: Option<&str>) -> Result<models::RespSecret, Error<vault_api::ReadSecretError>> {
        vault_api::read_secret(&self.config, secret_type, secret_name, tenant, user, version, sysid, sysuser, keytype, dbhost, dbname, dbservice).await
    }

    pub async fn read_secret_meta(&self, secret_type: &str, secret_name: &str, tenant: Option<&str>, user: Option<&str>, sysid: Option<&str>, sysuser: Option<&str>, keytype: Option<&str>, dbhost: Option<&str>, dbname: Option<&str>, dbservice: Option<&str>) -> Result<models::RespSecretVersionMetadata, Error<vault_api::ReadSecretMetaError>> {
        vault_api::read_secret_meta(&self.config, secret_type, secret_name, tenant, user, sysid, sysuser, keytype, dbhost, dbname, dbservice).await
    }

    pub async fn undelete_secret(&self, secret_type: &str, secret_name: &str, req_versions: models::ReqVersions, sysid: Option<&str>, sysuser: Option<&str>, keytype: Option<&str>, dbhost: Option<&str>, dbname: Option<&str>, dbservice: Option<&str>) -> Result<models::RespVersions, Error<vault_api::UndeleteSecretError>> {
        vault_api::undelete_secret(&self.config, secret_type, secret_name, req_versions, sysid, sysuser, keytype, dbhost, dbname, dbservice).await
    }

    pub async fn validate_service_password(&self, secret_name: &str, req_validate_pwd: models::ReqValidatePwd) -> Result<models::RespAuthorized, Error<vault_api::ValidateServicePasswordError>> {
        vault_api::validate_service_password(&self.config, secret_name, req_validate_pwd).await
    }

    pub async fn validate_site_admin_password(&self, secret_name: &str, req_validate_pwd: models::ReqValidatePwd) -> Result<models::RespAuthorized, Error<vault_api::ValidateSiteAdminPasswordError>> {
        vault_api::validate_site_admin_password(&self.config, secret_name, req_validate_pwd).await
    }

    pub async fn write_secret(&self, secret_type: &str, secret_name: &str, req_write_secret: models::ReqWriteSecret, sysid: Option<&str>, sysuser: Option<&str>, keytype: Option<&str>, dbhost: Option<&str>, dbname: Option<&str>, dbservice: Option<&str>) -> Result<models::RespSecretMeta, Error<vault_api::WriteSecretError>> {
        vault_api::write_secret(&self.config, secret_type, secret_name, req_write_secret, sysid, sysuser, keytype, dbhost, dbname, dbservice).await
    }

}

