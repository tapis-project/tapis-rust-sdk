use crate::apis::{
    archives_api, channels_api, configuration, healthcheck_api, hello_api, instruments_api,
    measured_properties_api, measurements_api, ontologies_api, projects_api, ready_api,
    revoke_roles_api, roles_api, search_across_all_streams_end_points_api, sites_api,
    templates_api, transfer_instrument_data_api, units_api, variables_api, Error,
};
use crate::models;
use http::header::{HeaderMap, HeaderValue};
use std::sync::Arc;

#[derive(Clone)]
pub struct TapisStreams {
    config: Arc<configuration::Configuration>,
    pub archives: ArchivesClient,
    pub channels: ChannelsClient,
    pub healthcheck: HealthcheckClient,
    pub hello: HelloClient,
    pub instruments: InstrumentsClient,
    pub measured_properties: MeasuredPropertiesClient,
    pub measurements: MeasurementsClient,
    pub ontologies: OntologiesClient,
    pub projects: ProjectsClient,
    pub ready: ReadyClient,
    pub revoke_roles: RevokeRolesClient,
    pub roles: RolesClient,
    pub search_across_all_streams_end_points: SearchAcrossAllStreamsEndPointsClient,
    pub sites: SitesClient,
    pub templates: TemplatesClient,
    pub transfer_instrument_data: TransferInstrumentDataClient,
    pub units: UnitsClient,
    pub variables: VariablesClient,
}

impl TapisStreams {
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
            archives: ArchivesClient {
                config: config.clone(),
            },
            channels: ChannelsClient {
                config: config.clone(),
            },
            healthcheck: HealthcheckClient {
                config: config.clone(),
            },
            hello: HelloClient {
                config: config.clone(),
            },
            instruments: InstrumentsClient {
                config: config.clone(),
            },
            measured_properties: MeasuredPropertiesClient {
                config: config.clone(),
            },
            measurements: MeasurementsClient {
                config: config.clone(),
            },
            ontologies: OntologiesClient {
                config: config.clone(),
            },
            projects: ProjectsClient {
                config: config.clone(),
            },
            ready: ReadyClient {
                config: config.clone(),
            },
            revoke_roles: RevokeRolesClient {
                config: config.clone(),
            },
            roles: RolesClient {
                config: config.clone(),
            },
            search_across_all_streams_end_points: SearchAcrossAllStreamsEndPointsClient {
                config: config.clone(),
            },
            sites: SitesClient {
                config: config.clone(),
            },
            templates: TemplatesClient {
                config: config.clone(),
            },
            transfer_instrument_data: TransferInstrumentDataClient {
                config: config.clone(),
            },
            units: UnitsClient {
                config: config.clone(),
            },
            variables: VariablesClient {
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
    pub async fn archive_project(
        &self,
        project_id: &str,
        new_archive: models::NewArchive,
    ) -> Result<models::ArchiveProject201Response, Error<archives_api::ArchiveProjectError>> {
        archives_api::archive_project(&self.config, project_id, new_archive).await
    }

    pub async fn list_archives(
        &self,
        project_id: &str,
    ) -> Result<models::ListArchives201Response, Error<archives_api::ListArchivesError>> {
        archives_api::list_archives(&self.config, project_id).await
    }
}

#[derive(Clone)]
pub struct ChannelsClient {
    config: Arc<configuration::Configuration>,
}

impl ChannelsClient {
    pub async fn create_channels(
        &self,
        new_channel: models::NewChannel,
    ) -> Result<models::ListChannels200Response, Error<channels_api::CreateChannelsError>> {
        channels_api::create_channels(&self.config, new_channel).await
    }

    pub async fn delete_channel(
        &self,
        channel_id: &str,
    ) -> Result<models::GetChannel200Response, Error<channels_api::DeleteChannelError>> {
        channels_api::delete_channel(&self.config, channel_id).await
    }

    pub async fn get_channel(
        &self,
        channel_id: &str,
    ) -> Result<models::GetChannel200Response, Error<channels_api::GetChannelError>> {
        channels_api::get_channel(&self.config, channel_id).await
    }

    pub async fn list_alerts(
        &self,
        channel_id: &str,
    ) -> Result<models::ListAlerts200Response, Error<channels_api::ListAlertsError>> {
        channels_api::list_alerts(&self.config, channel_id).await
    }

    pub async fn list_channels(
        &self,
        query: Option<&str>,
        limit: Option<i32>,
        skip: Option<i32>,
    ) -> Result<models::ListChannels200Response, Error<channels_api::ListChannelsError>> {
        channels_api::list_channels(&self.config, query, limit, skip).await
    }

    pub async fn update_channel(
        &self,
        channel_id: &str,
        update_channel: models::UpdateChannel,
    ) -> Result<models::GetChannel200Response, Error<channels_api::UpdateChannelError>> {
        channels_api::update_channel(&self.config, channel_id, update_channel).await
    }

    pub async fn update_status(
        &self,
        channel_id: &str,
        update_channel_status: models::UpdateChannelStatus,
    ) -> Result<models::GetChannel200Response, Error<channels_api::UpdateStatusError>> {
        channels_api::update_status(&self.config, channel_id, update_channel_status).await
    }
}

#[derive(Clone)]
pub struct HealthcheckClient {
    config: Arc<configuration::Configuration>,
}

impl HealthcheckClient {
    pub async fn healthcheck(
        &self,
        tenant: &str,
    ) -> Result<models::Hello200Response, Error<healthcheck_api::HealthcheckError>> {
        healthcheck_api::healthcheck(&self.config, tenant).await
    }
}

#[derive(Clone)]
pub struct HelloClient {
    config: Arc<configuration::Configuration>,
}

impl HelloClient {
    pub async fn hello(&self) -> Result<models::Hello200Response, Error<hello_api::HelloError>> {
        hello_api::hello(&self.config).await
    }
}

#[derive(Clone)]
pub struct InstrumentsClient {
    config: Arc<configuration::Configuration>,
}

impl InstrumentsClient {
    pub async fn create_instrument(
        &self,
        project_id: &str,
        site_id: &str,
        new_instrument: Vec<models::NewInstrument>,
    ) -> Result<models::ListInstruments200Response, Error<instruments_api::CreateInstrumentError>>
    {
        instruments_api::create_instrument(&self.config, project_id, site_id, new_instrument).await
    }

    pub async fn delete_instrument(
        &self,
        project_id: &str,
        site_id: &str,
        inst_id: &str,
    ) -> Result<models::GetInstrument200Response, Error<instruments_api::DeleteInstrumentError>>
    {
        instruments_api::delete_instrument(&self.config, project_id, site_id, inst_id).await
    }

    pub async fn get_instrument(
        &self,
        project_id: &str,
        site_id: &str,
        inst_id: &str,
    ) -> Result<models::GetInstrument200Response, Error<instruments_api::GetInstrumentError>> {
        instruments_api::get_instrument(&self.config, project_id, site_id, inst_id).await
    }

    pub async fn list_instruments(
        &self,
        project_id: &str,
        site_id: &str,
        query: Option<&str>,
        limit: Option<i32>,
        skip: Option<i32>,
    ) -> Result<models::ListInstruments200Response, Error<instruments_api::ListInstrumentsError>>
    {
        instruments_api::list_instruments(&self.config, project_id, site_id, query, limit, skip)
            .await
    }

    pub async fn update_instrument(
        &self,
        project_id: &str,
        site_id: &str,
        inst_id: &str,
        update_inst: models::UpdateInst,
    ) -> Result<models::ListInstruments200Response, Error<instruments_api::UpdateInstrumentError>>
    {
        instruments_api::update_instrument(&self.config, project_id, site_id, inst_id, update_inst)
            .await
    }
}

#[derive(Clone)]
pub struct MeasuredPropertiesClient {
    config: Arc<configuration::Configuration>,
}

impl MeasuredPropertiesClient {
    pub async fn list_measured_properties(
        &self,
        query: Option<&str>,
        limit: Option<i32>,
        skip: Option<i32>,
    ) -> Result<
        models::ListMeasuredProperties200Response,
        Error<measured_properties_api::ListMeasuredPropertiesError>,
    > {
        measured_properties_api::list_measured_properties(&self.config, query, limit, skip).await
    }
}

#[derive(Clone)]
pub struct MeasurementsClient {
    config: Arc<configuration::Configuration>,
}

impl MeasurementsClient {
    pub async fn create_measurement(
        &self,
        new_measurement: models::NewMeasurement,
    ) -> Result<models::CreateMeasurement201Response, Error<measurements_api::CreateMeasurementError>>
    {
        measurements_api::create_measurement(&self.config, new_measurement).await
    }

    pub async fn download_measurements(
        &self,
        inst_id: &str,
        query: Option<&str>,
        var_ids: Option<&str>,
        limit: Option<i32>,
        skip: Option<i32>,
        start_date: Option<&str>,
        end_date: Option<&str>,
        with_metadata: Option<bool>,
        format: Option<&str>,
    ) -> Result<
        models::CreateMeasurement201Response,
        Error<measurements_api::DownloadMeasurementsError>,
    > {
        measurements_api::download_measurements(
            &self.config,
            inst_id,
            query,
            var_ids,
            limit,
            skip,
            start_date,
            end_date,
            with_metadata,
            format,
        )
        .await
    }

    pub async fn list_measurements(
        &self,
        project_id: &str,
        site_id: &str,
        inst_id: &str,
        query: Option<&str>,
        var_ids: Option<&str>,
        limit: Option<i32>,
        skip: Option<i32>,
        start_date: Option<&str>,
        end_date: Option<&str>,
        geojson: Option<serde_json::Value>,
        format: Option<&str>,
        with_metadata: Option<bool>,
    ) -> Result<models::CreateMeasurement201Response, Error<measurements_api::ListMeasurementsError>>
    {
        measurements_api::list_measurements(
            &self.config,
            project_id,
            site_id,
            inst_id,
            query,
            var_ids,
            limit,
            skip,
            start_date,
            end_date,
            geojson,
            format,
            with_metadata,
        )
        .await
    }
}

#[derive(Clone)]
pub struct OntologiesClient {
    config: Arc<configuration::Configuration>,
}

impl OntologiesClient {
    pub async fn add_ontology(
        &self,
        new_ontology: models::NewOntology,
    ) -> Result<models::ListOntologies200Response, Error<ontologies_api::AddOntologyError>> {
        ontologies_api::add_ontology(&self.config, new_ontology).await
    }

    pub async fn delete_ontology(
        &self,
        onto_id: &str,
    ) -> Result<models::GetOntology200Response, Error<ontologies_api::DeleteOntologyError>> {
        ontologies_api::delete_ontology(&self.config, onto_id).await
    }

    pub async fn get_ontology(
        &self,
        onto_id: &str,
    ) -> Result<models::GetOntology200Response, Error<ontologies_api::GetOntologyError>> {
        ontologies_api::get_ontology(&self.config, onto_id).await
    }

    pub async fn list_ontologies(
        &self,
        query: Option<&str>,
        limit: Option<i32>,
        skip: Option<i32>,
    ) -> Result<models::ListOntologies200Response, Error<ontologies_api::ListOntologiesError>> {
        ontologies_api::list_ontologies(&self.config, query, limit, skip).await
    }

    pub async fn update_ontology(
        &self,
        onto_id: &str,
        new_ontology: models::NewOntology,
    ) -> Result<models::GetOntology200Response, Error<ontologies_api::UpdateOntologyError>> {
        ontologies_api::update_ontology(&self.config, onto_id, new_ontology).await
    }
}

#[derive(Clone)]
pub struct ProjectsClient {
    config: Arc<configuration::Configuration>,
}

impl ProjectsClient {
    pub async fn create_project(
        &self,
        new_project: models::NewProject,
    ) -> Result<models::CreateProject201Response, Error<projects_api::CreateProjectError>> {
        projects_api::create_project(&self.config, new_project).await
    }

    pub async fn delete_project(
        &self,
        project_id: &str,
    ) -> Result<models::CreateProject201Response, Error<projects_api::DeleteProjectError>> {
        projects_api::delete_project(&self.config, project_id).await
    }

    pub async fn get_project(
        &self,
        project_id: &str,
    ) -> Result<models::CreateProject201Response, Error<projects_api::GetProjectError>> {
        projects_api::get_project(&self.config, project_id).await
    }

    pub async fn list_projects(
        &self,
        query: Option<&str>,
        limit: Option<i32>,
        skip: Option<i32>,
    ) -> Result<models::ListProjects200Response, Error<projects_api::ListProjectsError>> {
        projects_api::list_projects(&self.config, query, limit, skip).await
    }

    pub async fn update_project(
        &self,
        project_id: &str,
        new_project: models::NewProject,
    ) -> Result<models::CreateProject201Response, Error<projects_api::UpdateProjectError>> {
        projects_api::update_project(&self.config, project_id, new_project).await
    }
}

#[derive(Clone)]
pub struct ReadyClient {
    config: Arc<configuration::Configuration>,
}

impl ReadyClient {
    pub async fn ready(&self) -> Result<models::Hello200Response, Error<ready_api::ReadyError>> {
        ready_api::ready(&self.config).await
    }
}

#[derive(Clone)]
pub struct RevokeRolesClient {
    config: Arc<configuration::Configuration>,
}

impl RevokeRolesClient {
    pub async fn revoke_role(
        &self,
        revoke_role: models::RevokeRole,
    ) -> Result<models::ListRoles200Response, Error<revoke_roles_api::RevokeRoleError>> {
        revoke_roles_api::revoke_role(&self.config, revoke_role).await
    }
}

#[derive(Clone)]
pub struct RolesClient {
    config: Arc<configuration::Configuration>,
}

impl RolesClient {
    pub async fn grant_role(
        &self,
        new_role: models::NewRole,
    ) -> Result<models::ListRoles200Response, Error<roles_api::GrantRoleError>> {
        roles_api::grant_role(&self.config, new_role).await
    }

    pub async fn list_roles(
        &self,
        user: &str,
        resource_type: &str,
        resource_id: &str,
    ) -> Result<models::ListRoles200Response, Error<roles_api::ListRolesError>> {
        roles_api::list_roles(&self.config, user, resource_type, resource_id).await
    }
}

#[derive(Clone)]
pub struct SearchAcrossAllStreamsEndPointsClient {
    config: Arc<configuration::Configuration>,
}

impl SearchAcrossAllStreamsEndPointsClient {
    pub async fn search(
        &self,
        resource_type: &str,
        list_type: Option<models::ListTypeEnum>,
        skip: Option<i32>,
        compute_total: Option<bool>,
    ) -> Result<
        models::Search200Response,
        Error<search_across_all_streams_end_points_api::SearchError>,
    > {
        search_across_all_streams_end_points_api::search(
            &self.config,
            resource_type,
            list_type,
            skip,
            compute_total,
        )
        .await
    }
}

#[derive(Clone)]
pub struct SitesClient {
    config: Arc<configuration::Configuration>,
}

impl SitesClient {
    pub async fn create_site(
        &self,
        project_id: &str,
        new_site: Vec<models::NewSite>,
    ) -> Result<models::ListSites200Response, Error<sites_api::CreateSiteError>> {
        sites_api::create_site(&self.config, project_id, new_site).await
    }

    pub async fn delete_site(
        &self,
        project_id: &str,
        site_id: &str,
    ) -> Result<models::GetSite200Response, Error<sites_api::DeleteSiteError>> {
        sites_api::delete_site(&self.config, project_id, site_id).await
    }

    pub async fn get_site(
        &self,
        project_id: &str,
        site_id: &str,
    ) -> Result<models::GetSite200Response, Error<sites_api::GetSiteError>> {
        sites_api::get_site(&self.config, project_id, site_id).await
    }

    pub async fn list_sites(
        &self,
        project_id: &str,
        query: Option<&str>,
        limit: Option<i32>,
        skip: Option<i32>,
    ) -> Result<models::ListSites200Response, Error<sites_api::ListSitesError>> {
        sites_api::list_sites(&self.config, project_id, query, limit, skip).await
    }

    pub async fn update_site(
        &self,
        project_id: &str,
        site_id: &str,
        update_site: models::UpdateSite,
    ) -> Result<models::GetSite200Response, Error<sites_api::UpdateSiteError>> {
        sites_api::update_site(&self.config, project_id, site_id, update_site).await
    }
}

#[derive(Clone)]
pub struct TemplatesClient {
    config: Arc<configuration::Configuration>,
}

impl TemplatesClient {
    pub async fn create_template(
        &self,
        new_template: models::NewTemplate,
    ) -> Result<models::ListTemplates200Response, Error<templates_api::CreateTemplateError>> {
        templates_api::create_template(&self.config, new_template).await
    }

    pub async fn get_template(
        &self,
        template_id: &str,
    ) -> Result<models::ListTemplates200Response, Error<templates_api::GetTemplateError>> {
        templates_api::get_template(&self.config, template_id).await
    }

    pub async fn list_templates(
        &self,
        query: Option<&str>,
        limit: Option<i32>,
        skip: Option<i32>,
    ) -> Result<models::ListTemplates200Response, Error<templates_api::ListTemplatesError>> {
        templates_api::list_templates(&self.config, query, limit, skip).await
    }

    pub async fn update_template(
        &self,
        template_id: &str,
        new_template: models::NewTemplate,
    ) -> Result<models::UpdateTemplate201Response, Error<templates_api::UpdateTemplateError>> {
        templates_api::update_template(&self.config, template_id, new_template).await
    }
}

#[derive(Clone)]
pub struct TransferInstrumentDataClient {
    config: Arc<configuration::Configuration>,
}

impl TransferInstrumentDataClient {
    pub async fn transfer_data(
        &self,
        transfer: models::Transfer,
    ) -> Result<
        models::TransferData201Response,
        Error<transfer_instrument_data_api::TransferDataError>,
    > {
        transfer_instrument_data_api::transfer_data(&self.config, transfer).await
    }
}

#[derive(Clone)]
pub struct UnitsClient {
    config: Arc<configuration::Configuration>,
}

impl UnitsClient {
    pub async fn list_units(
        &self,
        query: Option<&str>,
        limit: Option<i32>,
        skip: Option<i32>,
    ) -> Result<models::ListUnits200Response, Error<units_api::ListUnitsError>> {
        units_api::list_units(&self.config, query, limit, skip).await
    }
}

#[derive(Clone)]
pub struct VariablesClient {
    config: Arc<configuration::Configuration>,
}

impl VariablesClient {
    pub async fn create_variable(
        &self,
        project_id: &str,
        site_id: &str,
        inst_id: &str,
        new_variable: Vec<models::NewVariable>,
    ) -> Result<models::ListVariables200Response, Error<variables_api::CreateVariableError>> {
        variables_api::create_variable(&self.config, project_id, site_id, inst_id, new_variable)
            .await
    }

    pub async fn delete_variable(
        &self,
        project_id: &str,
        site_id: &str,
        inst_id: &str,
        var_id: &str,
    ) -> Result<models::GetVariable200Response, Error<variables_api::DeleteVariableError>> {
        variables_api::delete_variable(&self.config, project_id, site_id, inst_id, var_id).await
    }

    pub async fn get_variable(
        &self,
        project_id: &str,
        site_id: &str,
        inst_id: &str,
        var_id: &str,
    ) -> Result<models::GetVariable200Response, Error<variables_api::GetVariableError>> {
        variables_api::get_variable(&self.config, project_id, site_id, inst_id, var_id).await
    }

    pub async fn list_variables(
        &self,
        project_id: &str,
        site_id: &str,
        inst_id: &str,
        query: Option<&str>,
        limit: Option<i32>,
        skip: Option<i32>,
    ) -> Result<models::ListVariables200Response, Error<variables_api::ListVariablesError>> {
        variables_api::list_variables(
            &self.config,
            project_id,
            site_id,
            inst_id,
            query,
            limit,
            skip,
        )
        .await
    }

    pub async fn update_variable(
        &self,
        project_id: &str,
        site_id: &str,
        inst_id: &str,
        var_id: &str,
        update_variable: models::UpdateVariable,
    ) -> Result<models::GetVariable200Response, Error<variables_api::UpdateVariableError>> {
        variables_api::update_variable(
            &self.config,
            project_id,
            site_id,
            inst_id,
            var_id,
            update_variable,
        )
        .await
    }
}
