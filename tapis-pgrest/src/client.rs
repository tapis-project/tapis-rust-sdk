use crate::apis::{configuration, Error, manage_roles_api, manage_tables_api, manage_views_api, tables_api, views_api};
use crate::models;
use http::header::{HeaderMap, HeaderValue};
use std::sync::Arc;

#[derive(Clone)]
pub struct TapisPgrest {
    config: Arc<configuration::Configuration>,
    pub manage_roles: ManageRolesClient,
    pub manage_tables: ManageTablesClient,
    pub manage_views: ManageViewsClient,
    pub tables: TablesClient,
    pub views: ViewsClient,
}

impl TapisPgrest {
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
            manage_roles: ManageRolesClient { config: config.clone() },
            manage_tables: ManageTablesClient { config: config.clone() },
            manage_views: ManageViewsClient { config: config.clone() },
            tables: TablesClient { config: config.clone() },
            views: ViewsClient { config: config.clone() },
        })
    }

    pub fn config(&self) -> &configuration::Configuration {
        &self.config
    }
}

#[derive(Clone)]
pub struct ManageRolesClient {
    config: Arc<configuration::Configuration>,
}

impl ManageRolesClient {
    pub async fn create_role(&self, new_role: models::NewRole) -> Result<models::CreateRole200Response, Error<manage_roles_api::CreateRoleError>> {
        manage_roles_api::create_role(&self.config, new_role).await
    }

    pub async fn get_role(&self, role_name: &str) -> Result<models::GetRole200Response, Error<manage_roles_api::GetRoleError>> {
        manage_roles_api::get_role(&self.config, role_name).await
    }

    pub async fn list_roles(&self) -> Result<models::ListRoles200Response, Error<manage_roles_api::ListRolesError>> {
        manage_roles_api::list_roles(&self.config).await
    }

    pub async fn manage_role(&self, role_name: &str, manage_role: models::ManageRole) -> Result<models::CreateRole200Response, Error<manage_roles_api::ManageRoleError>> {
        manage_roles_api::manage_role(&self.config, role_name, manage_role).await
    }

}

#[derive(Clone)]
pub struct ManageTablesClient {
    config: Arc<configuration::Configuration>,
}

impl ManageTablesClient {
    pub async fn create_table(&self, new_table: models::NewTable) -> Result<models::CreateTable201Response, Error<manage_tables_api::CreateTableError>> {
        manage_tables_api::create_table(&self.config, new_table).await
    }

    pub async fn delete_table(&self, table_id: &str) -> Result<models::BasicResponse, Error<manage_tables_api::DeleteTableError>> {
        manage_tables_api::delete_table(&self.config, table_id).await
    }

    pub async fn get_table(&self, table_id: &str, details: Option<bool>) -> Result<models::CreateTable201Response, Error<manage_tables_api::GetTableError>> {
        manage_tables_api::get_table(&self.config, table_id, details).await
    }

    pub async fn list_tables(&self) -> Result<models::ListTables200Response, Error<manage_tables_api::ListTablesError>> {
        manage_tables_api::list_tables(&self.config).await
    }

    pub async fn update_table(&self, table_id: &str, update_table: models::UpdateTable) -> Result<models::CreateTable201Response, Error<manage_tables_api::UpdateTableError>> {
        manage_tables_api::update_table(&self.config, table_id, update_table).await
    }

}

#[derive(Clone)]
pub struct ManageViewsClient {
    config: Arc<configuration::Configuration>,
}

impl ManageViewsClient {
    pub async fn create_view(&self, new_view: models::NewView) -> Result<models::CreateView201Response, Error<manage_views_api::CreateViewError>> {
        manage_views_api::create_view(&self.config, new_view).await
    }

    pub async fn delete_view(&self, view_name: &str) -> Result<models::BasicResponse, Error<manage_views_api::DeleteViewError>> {
        manage_views_api::delete_view(&self.config, view_name).await
    }

    pub async fn get_manage_view(&self, view_name: &str, details: Option<bool>) -> Result<models::CreateView201Response, Error<manage_views_api::GetManageViewError>> {
        manage_views_api::get_manage_view(&self.config, view_name, details).await
    }

    pub async fn list_views(&self) -> Result<models::ListViews200Response, Error<manage_views_api::ListViewsError>> {
        manage_views_api::list_views(&self.config).await
    }

    pub async fn refresh_materialized_view(&self, view_name: &str) -> Result<models::BasicResponse, Error<manage_views_api::RefreshMaterializedViewError>> {
        manage_views_api::refresh_materialized_view(&self.config, view_name).await
    }

}

#[derive(Clone)]
pub struct TablesClient {
    config: Arc<configuration::Configuration>,
}

impl TablesClient {
    pub async fn add_table_row(&self, root_url: &str, new_table_row: models::NewTableRow) -> Result<models::AddTableRow201Response, Error<tables_api::AddTableRowError>> {
        tables_api::add_table_row(&self.config, root_url, new_table_row).await
    }

    pub async fn add_table_rows(&self, root_url: &str, new_table_rows: models::NewTableRows) -> Result<models::GetTableRows200Response, Error<tables_api::AddTableRowsError>> {
        tables_api::add_table_rows(&self.config, root_url, new_table_rows).await
    }

    pub async fn delete_table_row(&self, root_url: &str, item: &str) -> Result<models::BasicResponse, Error<tables_api::DeleteTableRowError>> {
        tables_api::delete_table_row(&self.config, root_url, item).await
    }

    pub async fn get_table_row(&self, root_url: &str, item: &str) -> Result<models::AddTableRow201Response, Error<tables_api::GetTableRowError>> {
        tables_api::get_table_row(&self.config, root_url, item).await
    }

    pub async fn get_table_rows(&self, root_url: &str, limit: Option<i32>, offset: Option<i32>) -> Result<models::GetTableRows200Response, Error<tables_api::GetTableRowsError>> {
        tables_api::get_table_rows(&self.config, root_url, limit, offset).await
    }

    pub async fn update_table_row(&self, root_url: &str, item: &str, body: serde_json::Value) -> Result<models::AddTableRow201Response, Error<tables_api::UpdateTableRowError>> {
        tables_api::update_table_row(&self.config, root_url, item, body).await
    }

    pub async fn update_table_rows(&self, root_url: &str, update_multiple_table_rows: models::UpdateMultipleTableRows) -> Result<models::BasicResponse, Error<tables_api::UpdateTableRowsError>> {
        tables_api::update_table_rows(&self.config, root_url, update_multiple_table_rows).await
    }

}

#[derive(Clone)]
pub struct ViewsClient {
    config: Arc<configuration::Configuration>,
}

impl ViewsClient {
    pub async fn get_view(&self, view_name: &str, limit: Option<i32>, offset: Option<i32>) -> Result<models::GetTableRows200Response, Error<views_api::GetViewError>> {
        views_api::get_view(&self.config, view_name, limit, offset).await
    }

}

