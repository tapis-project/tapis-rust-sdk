use crate::apis::{
    configuration, images_api, jupyter_api, misc_api, permissions_api, pods_api, secrets_api,
    snapshots_api, templates_api, volumes_api, Error,
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
pub struct TapisPods {
    config: Arc<configuration::Configuration>,
    pub images: ImagesClient,
    pub jupyter: JupyterClient,
    pub misc: MiscClient,
    pub permissions: PermissionsClient,
    pub pods: PodsClient,
    pub secrets: SecretsClient,
    pub snapshots: SnapshotsClient,
    pub templates: TemplatesClient,
    pub volumes: VolumesClient,
}

impl TapisPods {
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
            images: ImagesClient {
                config: config.clone(),
            },
            jupyter: JupyterClient {
                config: config.clone(),
            },
            misc: MiscClient {
                config: config.clone(),
            },
            permissions: PermissionsClient {
                config: config.clone(),
            },
            pods: PodsClient {
                config: config.clone(),
            },
            secrets: SecretsClient {
                config: config.clone(),
            },
            snapshots: SnapshotsClient {
                config: config.clone(),
            },
            templates: TemplatesClient {
                config: config.clone(),
            },
            volumes: VolumesClient {
                config: config.clone(),
            },
        })
    }

    pub fn config(&self) -> &configuration::Configuration {
        &self.config
    }
}

#[derive(Clone)]
pub struct ImagesClient {
    config: Arc<configuration::Configuration>,
}

impl ImagesClient {
    pub async fn add_image(
        &self,
        new_image: models::NewImage,
    ) -> Result<models::ImageResponse, Error<images_api::AddImageError>> {
        images_api::add_image(&self.config, new_image).await
    }

    pub async fn add_images(
        &self,
        new_image: Vec<models::NewImage>,
        skip_duplicates: Option<bool>,
    ) -> Result<models::ImagesResponse, Error<images_api::AddImagesError>> {
        images_api::add_images(&self.config, new_image, skip_duplicates).await
    }

    pub async fn delete_image(
        &self,
        image_id: &str,
    ) -> Result<models::ImageDeleteResponse, Error<images_api::DeleteImageError>> {
        images_api::delete_image(&self.config, image_id).await
    }

    pub async fn get_image(
        &self,
        image_id: &str,
    ) -> Result<models::ResponseGetImage, Error<images_api::GetImageError>> {
        images_api::get_image(&self.config, image_id).await
    }

    pub async fn get_images(
        &self,
    ) -> Result<models::ImagesResponse, Error<images_api::GetImagesError>> {
        images_api::get_images(&self.config).await
    }
}

#[derive(Clone)]
pub struct JupyterClient {
    config: Arc<configuration::Configuration>,
}

impl JupyterClient {
    pub async fn ensure_jupyter_pod(
        &self,
    ) -> Result<serde_json::Value, Error<jupyter_api::EnsureJupyterPodError>> {
        jupyter_api::ensure_jupyter_pod(&self.config).await
    }

    pub async fn upload_to_jupyter(
        &self,
        pod_id: &str,
        file: std::path::PathBuf,
        path: &str,
    ) -> Result<serde_json::Value, Error<jupyter_api::UploadToJupyterError>> {
        jupyter_api::upload_to_jupyter(&self.config, pod_id, file, path).await
    }
}

#[derive(Clone)]
pub struct MiscClient {
    config: Arc<configuration::Configuration>,
}

impl MiscClient {
    pub async fn error_handler(
        &self,
        status: &str,
    ) -> Result<serde_json::Value, Error<misc_api::ErrorHandlerError>> {
        misc_api::error_handler(&self.config, status).await
    }

    pub async fn healthcheck(
        &self,
    ) -> Result<serde_json::Value, Error<misc_api::HealthcheckError>> {
        misc_api::healthcheck(&self.config).await
    }

    pub async fn traefik_config(
        &self,
    ) -> Result<serde_json::Value, Error<misc_api::TraefikConfigError>> {
        misc_api::traefik_config(&self.config).await
    }
}

#[derive(Clone)]
pub struct PermissionsClient {
    config: Arc<configuration::Configuration>,
}

impl PermissionsClient {
    pub async fn delete_pod_permission(
        &self,
        pod_id: &str,
        user: &str,
    ) -> Result<models::PodPermissionsResponse, Error<permissions_api::DeletePodPermissionError>>
    {
        permissions_api::delete_pod_permission(&self.config, pod_id, user).await
    }

    pub async fn delete_snapshot_permission(
        &self,
        snapshot_id: &str,
        user: &str,
    ) -> Result<
        models::SnapshotPermissionsResponse,
        Error<permissions_api::DeleteSnapshotPermissionError>,
    > {
        permissions_api::delete_snapshot_permission(&self.config, snapshot_id, user).await
    }

    pub async fn delete_template_permission(
        &self,
        template_id: &str,
        user: &str,
    ) -> Result<
        models::TemplatePermissionsResponse,
        Error<permissions_api::DeleteTemplatePermissionError>,
    > {
        permissions_api::delete_template_permission(&self.config, template_id, user).await
    }

    pub async fn delete_volume_permission(
        &self,
        volume_id: &str,
        user: &str,
    ) -> Result<
        models::VolumePermissionsResponse,
        Error<permissions_api::DeleteVolumePermissionError>,
    > {
        permissions_api::delete_volume_permission(&self.config, volume_id, user).await
    }

    pub async fn get_pod_permissions(
        &self,
        pod_id: &str,
    ) -> Result<models::PodPermissionsResponse, Error<permissions_api::GetPodPermissionsError>>
    {
        permissions_api::get_pod_permissions(&self.config, pod_id).await
    }

    pub async fn get_snapshot_permissions(
        &self,
        snapshot_id: &str,
    ) -> Result<
        models::SnapshotPermissionsResponse,
        Error<permissions_api::GetSnapshotPermissionsError>,
    > {
        permissions_api::get_snapshot_permissions(&self.config, snapshot_id).await
    }

    pub async fn get_template_permissions(
        &self,
        template_id: &str,
    ) -> Result<
        models::TemplatePermissionsResponse,
        Error<permissions_api::GetTemplatePermissionsError>,
    > {
        permissions_api::get_template_permissions(&self.config, template_id).await
    }

    pub async fn get_volume_permissions(
        &self,
        volume_id: &str,
    ) -> Result<models::VolumePermissionsResponse, Error<permissions_api::GetVolumePermissionsError>>
    {
        permissions_api::get_volume_permissions(&self.config, volume_id).await
    }

    pub async fn set_pod_permission(
        &self,
        pod_id: &str,
        set_permission: models::SetPermission,
    ) -> Result<models::PodPermissionsResponse, Error<permissions_api::SetPodPermissionError>> {
        permissions_api::set_pod_permission(&self.config, pod_id, set_permission).await
    }

    pub async fn set_snapshot_permission(
        &self,
        snapshot_id: &str,
        set_permission: models::SetPermission,
    ) -> Result<
        models::SnapshotPermissionsResponse,
        Error<permissions_api::SetSnapshotPermissionError>,
    > {
        permissions_api::set_snapshot_permission(&self.config, snapshot_id, set_permission).await
    }

    pub async fn set_template_permission(
        &self,
        template_id: &str,
        set_permission: models::SetPermission,
    ) -> Result<
        models::TemplatePermissionsResponse,
        Error<permissions_api::SetTemplatePermissionError>,
    > {
        permissions_api::set_template_permission(&self.config, template_id, set_permission).await
    }

    pub async fn set_volume_permission(
        &self,
        volume_id: &str,
        set_permission: models::SetPermission,
    ) -> Result<models::VolumePermissionsResponse, Error<permissions_api::SetVolumePermissionError>>
    {
        permissions_api::set_volume_permission(&self.config, volume_id, set_permission).await
    }
}

#[derive(Clone)]
pub struct PodsClient {
    config: Arc<configuration::Configuration>,
}

impl PodsClient {
    pub async fn create_pod(
        &self,
        new_pod: models::NewPod,
    ) -> Result<models::PodResponse, Error<pods_api::CreatePodError>> {
        pods_api::create_pod(&self.config, new_pod).await
    }

    pub async fn delete_pod(
        &self,
        pod_id: &str,
    ) -> Result<models::PodDeleteResponse, Error<pods_api::DeletePodError>> {
        pods_api::delete_pod(&self.config, pod_id).await
    }

    pub async fn download_from_pod(
        &self,
        pod_id: &str,
        url_path: &str,
        path: Option<&str>,
    ) -> Result<serde_json::Value, Error<pods_api::DownloadFromPodError>> {
        pods_api::download_from_pod(&self.config, pod_id, url_path, path).await
    }

    pub async fn exec_pod_commands(
        &self,
        pod_id: &str,
        execute_pod_commands: models::ExecutePodCommands,
    ) -> Result<serde_json::Value, Error<pods_api::ExecPodCommandsError>> {
        pods_api::exec_pod_commands(&self.config, pod_id, execute_pod_commands).await
    }

    pub async fn get_derived_pod(
        &self,
        pod_id: &str,
        include_configs: Option<bool>,
        resolve_secrets: Option<bool>,
    ) -> Result<models::PodResponse, Error<pods_api::GetDerivedPodError>> {
        pods_api::get_derived_pod(&self.config, pod_id, include_configs, resolve_secrets).await
    }

    pub async fn get_pod(
        &self,
        pod_id: &str,
        include_configs: Option<bool>,
        check_unresolved: Option<bool>,
    ) -> Result<models::PodResponse, Error<pods_api::GetPodError>> {
        pods_api::get_pod(&self.config, pod_id, include_configs, check_unresolved).await
    }

    pub async fn get_pod_credentials(
        &self,
        pod_id: &str,
    ) -> Result<models::PodCredentialsResponse, Error<pods_api::GetPodCredentialsError>> {
        pods_api::get_pod_credentials(&self.config, pod_id).await
    }

    pub async fn get_pod_logs(
        &self,
        pod_id: &str,
    ) -> Result<models::PodLogsResponse, Error<pods_api::GetPodLogsError>> {
        pods_api::get_pod_logs(&self.config, pod_id).await
    }

    pub async fn list_files_in_pod(
        &self,
        pod_id: &str,
        url_path: &str,
        path: Option<&str>,
    ) -> Result<serde_json::Value, Error<pods_api::ListFilesInPodError>> {
        pods_api::list_files_in_pod(&self.config, pod_id, url_path, path).await
    }

    pub async fn list_pods(&self) -> Result<models::PodsResponse, Error<pods_api::ListPodsError>> {
        pods_api::list_pods(&self.config).await
    }

    pub async fn pod_auth(
        &self,
        pod_id_net: &str,
    ) -> Result<models::PodResponse, Error<pods_api::PodAuthError>> {
        pods_api::pod_auth(&self.config, pod_id_net).await
    }

    pub async fn pod_auth_callback(
        &self,
        pod_id_net: &str,
    ) -> Result<models::PodResponse, Error<pods_api::PodAuthCallbackError>> {
        pods_api::pod_auth_callback(&self.config, pod_id_net).await
    }

    pub async fn restart_pod(
        &self,
        pod_id: &str,
        grab_latest_template_tag: Option<bool>,
    ) -> Result<models::PodResponse, Error<pods_api::RestartPodError>> {
        pods_api::restart_pod(&self.config, pod_id, grab_latest_template_tag).await
    }

    pub async fn save_pod_as_template_tag(
        &self,
        pod_id_net: &str,
        new_template_tag_from_pod: models::NewTemplateTagFromPod,
    ) -> Result<models::TemplateTagResponse, Error<pods_api::SavePodAsTemplateTagError>> {
        pods_api::save_pod_as_template_tag(&self.config, pod_id_net, new_template_tag_from_pod)
            .await
    }

    pub async fn start_pod(
        &self,
        pod_id: &str,
    ) -> Result<models::PodResponse, Error<pods_api::StartPodError>> {
        pods_api::start_pod(&self.config, pod_id).await
    }

    pub async fn stop_pod(
        &self,
        pod_id: &str,
    ) -> Result<models::PodResponse, Error<pods_api::StopPodError>> {
        pods_api::stop_pod(&self.config, pod_id).await
    }

    pub async fn update_pod(
        &self,
        pod_id: &str,
        update_pod: models::UpdatePod,
    ) -> Result<models::PodResponse, Error<pods_api::UpdatePodError>> {
        pods_api::update_pod(&self.config, pod_id, update_pod).await
    }

    pub async fn upload_to_pod(
        &self,
        pod_id: &str,
        file: std::path::PathBuf,
        dest_path: &str,
    ) -> Result<serde_json::Value, Error<pods_api::UploadToPodError>> {
        pods_api::upload_to_pod(&self.config, pod_id, file, dest_path).await
    }
}

#[derive(Clone)]
pub struct SecretsClient {
    config: Arc<configuration::Configuration>,
}

impl SecretsClient {
    pub async fn create_secret(
        &self,
        new_secret: models::NewSecret,
    ) -> Result<models::SecretResponse, Error<secrets_api::CreateSecretError>> {
        secrets_api::create_secret(&self.config, new_secret).await
    }

    pub async fn delete_secret(
        &self,
        secret_id: &str,
    ) -> Result<models::SecretDeleteResponse, Error<secrets_api::DeleteSecretError>> {
        secrets_api::delete_secret(&self.config, secret_id).await
    }

    pub async fn get_secret(
        &self,
        secret_id: &str,
    ) -> Result<models::SecretResponse, Error<secrets_api::GetSecretError>> {
        secrets_api::get_secret(&self.config, secret_id).await
    }

    pub async fn get_secret_value(
        &self,
        secret_id: &str,
    ) -> Result<models::SecretValueResponse, Error<secrets_api::GetSecretValueError>> {
        secrets_api::get_secret_value(&self.config, secret_id).await
    }

    pub async fn list_secrets(
        &self,
    ) -> Result<models::SecretsResponse, Error<secrets_api::ListSecretsError>> {
        secrets_api::list_secrets(&self.config).await
    }

    pub async fn update_secret(
        &self,
        secret_id: &str,
        update_secret: models::UpdateSecret,
    ) -> Result<models::SecretResponse, Error<secrets_api::UpdateSecretError>> {
        secrets_api::update_secret(&self.config, secret_id, update_secret).await
    }
}

#[derive(Clone)]
pub struct SnapshotsClient {
    config: Arc<configuration::Configuration>,
}

impl SnapshotsClient {
    pub async fn create_snapshot(
        &self,
        new_snapshot: models::NewSnapshot,
    ) -> Result<models::SnapshotResponse, Error<snapshots_api::CreateSnapshotError>> {
        snapshots_api::create_snapshot(&self.config, new_snapshot).await
    }

    pub async fn delete_snapshot(
        &self,
        snapshot_id: &str,
    ) -> Result<models::DeleteSnapshotResponse, Error<snapshots_api::DeleteSnapshotError>> {
        snapshots_api::delete_snapshot(&self.config, snapshot_id).await
    }

    pub async fn download_snapshot_file(
        &self,
        snapshot_id: &str,
        path: &str,
    ) -> Result<serde_json::Value, Error<snapshots_api::DownloadSnapshotFileError>> {
        snapshots_api::download_snapshot_file(&self.config, snapshot_id, path).await
    }

    pub async fn get_snapshot(
        &self,
        snapshot_id: &str,
    ) -> Result<models::SnapshotResponse, Error<snapshots_api::GetSnapshotError>> {
        snapshots_api::get_snapshot(&self.config, snapshot_id).await
    }

    pub async fn get_snapshot_contents(
        &self,
        snapshot_id: &str,
        path: &str,
        zip: Option<bool>,
    ) -> Result<serde_json::Value, Error<snapshots_api::GetSnapshotContentsError>> {
        snapshots_api::get_snapshot_contents(&self.config, snapshot_id, path, zip).await
    }

    pub async fn list_snapshot_files(
        &self,
        snapshot_id: &str,
    ) -> Result<models::FilesListResponse, Error<snapshots_api::ListSnapshotFilesError>> {
        snapshots_api::list_snapshot_files(&self.config, snapshot_id).await
    }

    pub async fn list_snapshots(
        &self,
    ) -> Result<models::SnapshotsResponse, Error<snapshots_api::ListSnapshotsError>> {
        snapshots_api::list_snapshots(&self.config).await
    }

    pub async fn update_snapshot(
        &self,
        snapshot_id: &str,
        update_snapshot: models::UpdateSnapshot,
    ) -> Result<models::SnapshotResponse, Error<snapshots_api::UpdateSnapshotError>> {
        snapshots_api::update_snapshot(&self.config, snapshot_id, update_snapshot).await
    }
}

#[derive(Clone)]
pub struct TemplatesClient {
    config: Arc<configuration::Configuration>,
}

impl TemplatesClient {
    pub async fn add_template(
        &self,
        new_template: models::NewTemplate,
    ) -> Result<models::TemplateResponse, Error<templates_api::AddTemplateError>> {
        templates_api::add_template(&self.config, new_template).await
    }

    pub async fn add_template_tag(
        &self,
        template_id: &str,
        new_template_tag: models::NewTemplateTag,
    ) -> Result<models::TemplateTagResponse, Error<templates_api::AddTemplateTagError>> {
        templates_api::add_template_tag(&self.config, template_id, new_template_tag).await
    }

    pub async fn delete_template(
        &self,
        template_id: &str,
    ) -> Result<models::TemplateDeleteResponse, Error<templates_api::DeleteTemplateError>> {
        templates_api::delete_template(&self.config, template_id).await
    }

    pub async fn delete_template_tag(
        &self,
        template_id: &str,
        tag_id: &str,
        force: Option<bool>,
    ) -> Result<models::TemplateTagResponse, Error<templates_api::DeleteTemplateTagError>> {
        templates_api::delete_template_tag(&self.config, template_id, tag_id, force).await
    }

    pub async fn get_template(
        &self,
        template_id: &str,
        include_dependencies: Option<bool>,
    ) -> Result<models::TemplateResponse, Error<templates_api::GetTemplateError>> {
        templates_api::get_template(&self.config, template_id, include_dependencies).await
    }

    pub async fn get_template_tag(
        &self,
        template_id: &str,
        tag_id: &str,
        include_configs: Option<bool>,
        include_dependencies: Option<bool>,
    ) -> Result<models::TemplateTagsResponse, Error<templates_api::GetTemplateTagError>> {
        templates_api::get_template_tag(
            &self.config,
            template_id,
            tag_id,
            include_configs,
            include_dependencies,
        )
        .await
    }

    pub async fn list_template_tags(
        &self,
        template_id: &str,
        full: Option<bool>,
        include_configs: Option<bool>,
        include_dependencies: Option<bool>,
    ) -> Result<models::TemplateTagsResponse, Error<templates_api::ListTemplateTagsError>> {
        templates_api::list_template_tags(
            &self.config,
            template_id,
            full,
            include_configs,
            include_dependencies,
        )
        .await
    }

    pub async fn list_templates(
        &self,
        include_dependencies: Option<bool>,
    ) -> Result<models::TemplatesResponse, Error<templates_api::ListTemplatesError>> {
        templates_api::list_templates(&self.config, include_dependencies).await
    }

    pub async fn list_templates_and_tags(
        &self,
        full: Option<bool>,
        include_dependencies: Option<bool>,
    ) -> Result<
        std::collections::HashMap<String, serde_json::Value>,
        Error<templates_api::ListTemplatesAndTagsError>,
    > {
        templates_api::list_templates_and_tags(&self.config, full, include_dependencies).await
    }

    pub async fn update_template(
        &self,
        template_id: &str,
        update_template: models::UpdateTemplate,
    ) -> Result<models::TemplateResponse, Error<templates_api::UpdateTemplateError>> {
        templates_api::update_template(&self.config, template_id, update_template).await
    }
}

#[derive(Clone)]
pub struct VolumesClient {
    config: Arc<configuration::Configuration>,
}

impl VolumesClient {
    pub async fn create_volume(
        &self,
        new_volume: models::NewVolume,
    ) -> Result<models::VolumeResponse, Error<volumes_api::CreateVolumeError>> {
        volumes_api::create_volume(&self.config, new_volume).await
    }

    pub async fn delete_volume(
        &self,
        volume_id: &str,
    ) -> Result<models::DeleteVolumeResponse, Error<volumes_api::DeleteVolumeError>> {
        volumes_api::delete_volume(&self.config, volume_id).await
    }

    pub async fn download_volume_file(
        &self,
        volume_id: &str,
        path: &str,
    ) -> Result<serde_json::Value, Error<volumes_api::DownloadVolumeFileError>> {
        volumes_api::download_volume_file(&self.config, volume_id, path).await
    }

    pub async fn get_volume(
        &self,
        volume_id: &str,
    ) -> Result<models::VolumeResponse, Error<volumes_api::GetVolumeError>> {
        volumes_api::get_volume(&self.config, volume_id).await
    }

    pub async fn get_volume_contents(
        &self,
        volume_id: &str,
        path: &str,
        zip: Option<bool>,
    ) -> Result<serde_json::Value, Error<volumes_api::GetVolumeContentsError>> {
        volumes_api::get_volume_contents(&self.config, volume_id, path, zip).await
    }

    pub async fn list_volume_files(
        &self,
        volume_id: &str,
    ) -> Result<models::FilesListResponse, Error<volumes_api::ListVolumeFilesError>> {
        volumes_api::list_volume_files(&self.config, volume_id).await
    }

    pub async fn list_volumes(
        &self,
    ) -> Result<models::VolumesResponse, Error<volumes_api::ListVolumesError>> {
        volumes_api::list_volumes(&self.config).await
    }

    pub async fn update_volume(
        &self,
        volume_id: &str,
        update_volume: models::UpdateVolume,
    ) -> Result<models::VolumeResponse, Error<volumes_api::UpdateVolumeError>> {
        volumes_api::update_volume(&self.config, volume_id, update_volume).await
    }

    pub async fn upload_to_volume(
        &self,
        volume_id: &str,
        path: &str,
        file: std::path::PathBuf,
    ) -> Result<models::FilesUploadResponse, Error<volumes_api::UploadToVolumeError>> {
        volumes_api::upload_to_volume(&self.config, volume_id, path, file).await
    }
}
