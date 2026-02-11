use crate::apis::{configuration, Error, ldaps_api, owners_api, sites_api, tenants_api};
use crate::models;
use http::header::{HeaderMap, HeaderValue};
use std::sync::Arc;

#[derive(Clone)]
pub struct TapisTenants {
    config: Arc<configuration::Configuration>,
    pub ldaps: LdapsClient,
    pub owners: OwnersClient,
    pub sites: SitesClient,
    pub tenants: TenantsClient,
}

impl TapisTenants {
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
            ldaps: LdapsClient { config: config.clone() },
            owners: OwnersClient { config: config.clone() },
            sites: SitesClient { config: config.clone() },
            tenants: TenantsClient { config: config.clone() },
        })
    }

    pub fn config(&self) -> &configuration::Configuration {
        &self.config
    }
}

#[derive(Clone)]
pub struct LdapsClient {
    config: Arc<configuration::Configuration>,
}

impl LdapsClient {
    pub async fn create_ldap(&self, new_ldap: models::NewLdap) -> Result<models::CreateLdap201Response, Error<ldaps_api::CreateLdapError>> {
        ldaps_api::create_ldap(&self.config, new_ldap).await
    }

    pub async fn delete_ldap(&self, ldap_id: &str) -> Result<models::DeleteSite200Response, Error<ldaps_api::DeleteLdapError>> {
        ldaps_api::delete_ldap(&self.config, ldap_id).await
    }

    pub async fn get_ldap(&self, ldap_id: &str) -> Result<models::CreateLdap201Response, Error<ldaps_api::GetLdapError>> {
        ldaps_api::get_ldap(&self.config, ldap_id).await
    }

    pub async fn list_ldaps(&self, limit: Option<i32>, offset: Option<i32>) -> Result<models::ListLdaps200Response, Error<ldaps_api::ListLdapsError>> {
        ldaps_api::list_ldaps(&self.config, limit, offset).await
    }

}

#[derive(Clone)]
pub struct OwnersClient {
    config: Arc<configuration::Configuration>,
}

impl OwnersClient {
    pub async fn create_owner(&self, owner: models::Owner) -> Result<models::CreateOwner201Response, Error<owners_api::CreateOwnerError>> {
        owners_api::create_owner(&self.config, owner).await
    }

    pub async fn delete_owner(&self, email: &str) -> Result<models::DeleteSite200Response, Error<owners_api::DeleteOwnerError>> {
        owners_api::delete_owner(&self.config, email).await
    }

    pub async fn get_owner(&self, email: &str) -> Result<models::CreateOwner201Response, Error<owners_api::GetOwnerError>> {
        owners_api::get_owner(&self.config, email).await
    }

    pub async fn list_owners(&self, limit: Option<i32>, offset: Option<i32>) -> Result<models::ListOwners200Response, Error<owners_api::ListOwnersError>> {
        owners_api::list_owners(&self.config, limit, offset).await
    }

}

#[derive(Clone)]
pub struct SitesClient {
    config: Arc<configuration::Configuration>,
}

impl SitesClient {
    pub async fn create_site(&self, new_site: models::NewSite) -> Result<models::CreateSite201Response, Error<sites_api::CreateSiteError>> {
        sites_api::create_site(&self.config, new_site).await
    }

    pub async fn delete_site(&self, site_id: &str) -> Result<models::DeleteSite200Response, Error<sites_api::DeleteSiteError>> {
        sites_api::delete_site(&self.config, site_id).await
    }

    pub async fn get_site(&self, site_id: &str) -> Result<models::CreateSite201Response, Error<sites_api::GetSiteError>> {
        sites_api::get_site(&self.config, site_id).await
    }

    pub async fn list_sites(&self, limit: Option<i32>, offset: Option<i32>) -> Result<models::ListSites200Response, Error<sites_api::ListSitesError>> {
        sites_api::list_sites(&self.config, limit, offset).await
    }

}

#[derive(Clone)]
pub struct TenantsClient {
    config: Arc<configuration::Configuration>,
}

impl TenantsClient {
    pub async fn create_ldap(&self, new_ldap: models::NewLdap) -> Result<models::CreateLdap201Response, Error<tenants_api::CreateLdapError>> {
        tenants_api::create_ldap(&self.config, new_ldap).await
    }

    pub async fn create_owner(&self, owner: models::Owner) -> Result<models::CreateOwner201Response, Error<tenants_api::CreateOwnerError>> {
        tenants_api::create_owner(&self.config, owner).await
    }

    pub async fn create_tenant(&self, new_tenant: models::NewTenant) -> Result<models::CreateTenant201Response, Error<tenants_api::CreateTenantError>> {
        tenants_api::create_tenant(&self.config, new_tenant).await
    }

    pub async fn delete_ldap(&self, ldap_id: &str) -> Result<models::DeleteSite200Response, Error<tenants_api::DeleteLdapError>> {
        tenants_api::delete_ldap(&self.config, ldap_id).await
    }

    pub async fn delete_owner(&self, email: &str) -> Result<models::DeleteSite200Response, Error<tenants_api::DeleteOwnerError>> {
        tenants_api::delete_owner(&self.config, email).await
    }

    pub async fn get_ldap(&self, ldap_id: &str) -> Result<models::CreateLdap201Response, Error<tenants_api::GetLdapError>> {
        tenants_api::get_ldap(&self.config, ldap_id).await
    }

    pub async fn get_owner(&self, email: &str) -> Result<models::CreateOwner201Response, Error<tenants_api::GetOwnerError>> {
        tenants_api::get_owner(&self.config, email).await
    }

    pub async fn get_tenant(&self, tenant_id: &str) -> Result<models::CreateTenant201Response, Error<tenants_api::GetTenantError>> {
        tenants_api::get_tenant(&self.config, tenant_id).await
    }

    pub async fn get_tenant_history(&self, tenant_id: &str) -> Result<models::GetTenantHistory200Response, Error<tenants_api::GetTenantHistoryError>> {
        tenants_api::get_tenant_history(&self.config, tenant_id).await
    }

    pub async fn list_ldaps(&self, limit: Option<i32>, offset: Option<i32>) -> Result<models::ListLdaps200Response, Error<tenants_api::ListLdapsError>> {
        tenants_api::list_ldaps(&self.config, limit, offset).await
    }

    pub async fn list_owners(&self, limit: Option<i32>, offset: Option<i32>) -> Result<models::ListOwners200Response, Error<tenants_api::ListOwnersError>> {
        tenants_api::list_owners(&self.config, limit, offset).await
    }

    pub async fn list_tenants(&self, limit: Option<i32>, offset: Option<i32>) -> Result<models::ListTenants200Response, Error<tenants_api::ListTenantsError>> {
        tenants_api::list_tenants(&self.config, limit, offset).await
    }

    pub async fn update_tenant(&self, tenant_id: &str, update_tenant: models::UpdateTenant) -> Result<models::CreateTenant201Response, Error<tenants_api::UpdateTenantError>> {
        tenants_api::update_tenant(&self.config, tenant_id, update_tenant).await
    }

}

