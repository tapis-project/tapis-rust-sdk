/// TapisPods client - A high-level wrapper around the Tapis Pods API
///
/// # Example
///
/// ```rust,no_run
/// use tapis_pods::client::TapisPods;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = TapisPods::new(
///         "https://api.example.com/v3/pods",
///         "eyJhbGc...",
///     )?;
///
///     // List all pods
///     let pods = client.pods.list_pods().await?;
///     println!("Pods: {:#?}", pods);
///
///     Ok(())
/// }
/// ```
use crate::apis::{
    configuration, images_api, jupyter_api, misc_api, permissions_api, pods_api, snapshots_api,
    templates_api, volumes_api,
};
use http::header::{HeaderMap, HeaderValue};
use std::sync::Arc;

/// Main client for accessing the Tapis Pods API
#[derive(Clone)]
pub struct TapisPods {
    config: Arc<configuration::Configuration>,
    pub pods: PodsClient,
    pub templates: TemplatesClient,
    pub volumes: VolumesClient,
    pub snapshots: SnapshotsClient,
    pub images: ImagesClient,
    pub permissions: PermissionsClient,
    pub jupyter: JupyterClient,
    pub misc: MiscClient,
}

impl TapisPods {
    /// Create a new TapisPods client
    ///
    /// # Arguments
    ///
    /// * `base_url` - The base URL of the Tapis Pods API
    /// * `jwt_token` - The JWT token for authentication
    ///
    /// # Returns
    ///
    /// A Result containing the TapisPods client or an error
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use tapis_pods::client::TapisPods;
    ///
    /// let client = TapisPods::new(
    ///     "https://api.example.com/v3/pods",
    ///     "your_jwt_token"
    /// )?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn new(base_url: &str, jwt_token: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // Create default headers with X-Tapis-Token
        let mut headers = HeaderMap::new();
        headers.insert("X-Tapis-Token", HeaderValue::from_str(jwt_token)?);

        // Create reqwest client with default headers
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        // Create configuration
        let mut config = configuration::Configuration::default();
        config.base_path = base_url.to_string();
        config.client = client;

        let config = Arc::new(config);

        Ok(TapisPods {
            config: config.clone(),
            pods: PodsClient {
                config: config.clone(),
            },
            templates: TemplatesClient {
                config: config.clone(),
            },
            volumes: VolumesClient {
                config: config.clone(),
            },
            snapshots: SnapshotsClient {
                config: config.clone(),
            },
            images: ImagesClient {
                config: config.clone(),
            },
            permissions: PermissionsClient {
                config: config.clone(),
            },
            jupyter: JupyterClient {
                config: config.clone(),
            },
            misc: MiscClient {
                config: config.clone(),
            },
        })
    }

    /// Get the underlying configuration
    pub fn config(&self) -> &configuration::Configuration {
        &self.config
    }
}

/// Pods API client
#[derive(Clone)]
pub struct PodsClient {
    config: Arc<configuration::Configuration>,
}

impl PodsClient {
    pub async fn list_pods(
        &self,
    ) -> Result<crate::models::PodsResponse, crate::apis::Error<pods_api::ListPodsError>> {
        pods_api::list_pods(&self.config).await
    }

    pub async fn create_pod(
        &self,
        new_pod: crate::models::NewPod,
    ) -> Result<crate::models::PodResponse, crate::apis::Error<pods_api::CreatePodError>> {
        pods_api::create_pod(&self.config, new_pod).await
    }

    pub async fn get_pod(
        &self,
        pod_id: &str,
    ) -> Result<crate::models::PodResponse, crate::apis::Error<pods_api::GetPodError>> {
        pods_api::get_pod(&self.config, pod_id).await
    }

    pub async fn delete_pod(
        &self,
        pod_id: &str,
    ) -> Result<crate::models::PodDeleteResponse, crate::apis::Error<pods_api::DeletePodError>>
    {
        pods_api::delete_pod(&self.config, pod_id).await
    }

    pub async fn update_pod(
        &self,
        pod_id: &str,
        update_pod: crate::models::UpdatePod,
    ) -> Result<crate::models::PodResponse, crate::apis::Error<pods_api::UpdatePodError>> {
        pods_api::update_pod(&self.config, pod_id, update_pod).await
    }

    pub async fn start_pod(
        &self,
        pod_id: &str,
    ) -> Result<crate::models::PodResponse, crate::apis::Error<pods_api::StartPodError>> {
        pods_api::start_pod(&self.config, pod_id).await
    }

    pub async fn stop_pod(
        &self,
        pod_id: &str,
    ) -> Result<crate::models::PodResponse, crate::apis::Error<pods_api::StopPodError>> {
        pods_api::stop_pod(&self.config, pod_id).await
    }

    pub async fn restart_pod(
        &self,
        pod_id: &str,
        grab_latest_template_tag: Option<bool>,
    ) -> Result<crate::models::PodResponse, crate::apis::Error<pods_api::RestartPodError>> {
        pods_api::restart_pod(&self.config, pod_id, grab_latest_template_tag).await
    }

    pub async fn get_pod_logs(
        &self,
        pod_id: &str,
    ) -> Result<crate::models::PodLogsResponse, crate::apis::Error<pods_api::GetPodLogsError>> {
        pods_api::get_pod_logs(&self.config, pod_id).await
    }

    pub async fn get_pod_credentials(
        &self,
        pod_id: &str,
    ) -> Result<
        crate::models::PodCredentialsResponse,
        crate::apis::Error<pods_api::GetPodCredentialsError>,
    > {
        pods_api::get_pod_credentials(&self.config, pod_id).await
    }

    pub async fn exec_pod_commands(
        &self,
        pod_id: &str,
        execute_pod_commands: crate::models::ExecutePodCommands,
    ) -> Result<serde_json::Value, crate::apis::Error<pods_api::ExecPodCommandsError>> {
        pods_api::exec_pod_commands(&self.config, pod_id, execute_pod_commands).await
    }

    pub async fn get_derived_pod(
        &self,
        pod_id: &str,
    ) -> Result<crate::models::PodResponse, crate::apis::Error<pods_api::GetDerivedPodError>> {
        pods_api::get_derived_pod(&self.config, pod_id).await
    }

    pub async fn pod_auth(
        &self,
        pod_id_net: &str,
    ) -> Result<crate::models::PodResponse, crate::apis::Error<pods_api::PodAuthError>> {
        pods_api::pod_auth(&self.config, pod_id_net).await
    }

    pub async fn pod_auth_callback(
        &self,
        pod_id_net: &str,
    ) -> Result<crate::models::PodResponse, crate::apis::Error<pods_api::PodAuthCallbackError>>
    {
        pods_api::pod_auth_callback(&self.config, pod_id_net).await
    }

    pub async fn save_pod_as_template_tag(
        &self,
        pod_id_net: &str,
        new_template_tag_from_pod: crate::models::NewTemplateTagFromPod,
    ) -> Result<
        crate::models::TemplateTagResponse,
        crate::apis::Error<pods_api::SavePodAsTemplateTagError>,
    > {
        pods_api::save_pod_as_template_tag(&self.config, pod_id_net, new_template_tag_from_pod)
            .await
    }

    pub async fn upload_to_pod(
        &self,
        pod_id: &str,
        file: std::path::PathBuf,
        dest_path: &str,
    ) -> Result<serde_json::Value, crate::apis::Error<pods_api::UploadToPodError>> {
        pods_api::upload_to_pod(&self.config, pod_id, file, dest_path).await
    }
}

/// Templates API client
#[derive(Clone)]
pub struct TemplatesClient {
    config: Arc<configuration::Configuration>,
}

impl TemplatesClient {
    pub async fn list_templates(
        &self,
    ) -> Result<
        crate::models::TemplatesResponse,
        crate::apis::Error<templates_api::ListTemplatesError>,
    > {
        templates_api::list_templates(&self.config).await
    }

    pub async fn add_template(
        &self,
        new_template: crate::models::NewTemplate,
    ) -> Result<crate::models::TemplateResponse, crate::apis::Error<templates_api::AddTemplateError>>
    {
        templates_api::add_template(&self.config, new_template).await
    }

    pub async fn get_template(
        &self,
        template_id: &str,
    ) -> Result<crate::models::TemplateResponse, crate::apis::Error<templates_api::GetTemplateError>>
    {
        templates_api::get_template(&self.config, template_id).await
    }

    pub async fn delete_template(
        &self,
        template_id: &str,
    ) -> Result<
        crate::models::TemplateDeleteResponse,
        crate::apis::Error<templates_api::DeleteTemplateError>,
    > {
        templates_api::delete_template(&self.config, template_id).await
    }

    pub async fn update_template(
        &self,
        template_id: &str,
        update_template: crate::models::UpdateTemplate,
    ) -> Result<
        crate::models::TemplateResponse,
        crate::apis::Error<templates_api::UpdateTemplateError>,
    > {
        templates_api::update_template(&self.config, template_id, update_template).await
    }

    pub async fn list_template_tags(
        &self,
        template_id: &str,
        full: Option<bool>,
    ) -> Result<
        crate::models::TemplateTagsResponse,
        crate::apis::Error<templates_api::ListTemplateTagsError>,
    > {
        templates_api::list_template_tags(&self.config, template_id, full).await
    }

    pub async fn add_template_tag(
        &self,
        template_id: &str,
        new_template_tag: crate::models::NewTemplateTag,
    ) -> Result<
        crate::models::TemplateTagResponse,
        crate::apis::Error<templates_api::AddTemplateTagError>,
    > {
        templates_api::add_template_tag(&self.config, template_id, new_template_tag).await
    }

    pub async fn get_template_tag(
        &self,
        template_id: &str,
        tag_id: &str,
    ) -> Result<
        crate::models::TemplateTagsResponse,
        crate::apis::Error<templates_api::GetTemplateTagError>,
    > {
        templates_api::get_template_tag(&self.config, template_id, tag_id).await
    }

    pub async fn list_templates_and_tags(
        &self,
        full: Option<bool>,
    ) -> Result<serde_json::Value, crate::apis::Error<templates_api::ListTemplatesAndTagsError>>
    {
        templates_api::list_templates_and_tags(&self.config, full).await
    }
}

/// Volumes API client
#[derive(Clone)]
pub struct VolumesClient {
    config: Arc<configuration::Configuration>,
}

impl VolumesClient {
    pub async fn list_volumes(
        &self,
    ) -> Result<crate::models::VolumesResponse, crate::apis::Error<volumes_api::ListVolumesError>>
    {
        volumes_api::list_volumes(&self.config).await
    }

    pub async fn create_volume(
        &self,
        new_volume: crate::models::NewVolume,
    ) -> Result<crate::models::VolumeResponse, crate::apis::Error<volumes_api::CreateVolumeError>>
    {
        volumes_api::create_volume(&self.config, new_volume).await
    }

    pub async fn get_volume(
        &self,
        volume_id: &str,
    ) -> Result<crate::models::VolumeResponse, crate::apis::Error<volumes_api::GetVolumeError>>
    {
        volumes_api::get_volume(&self.config, volume_id).await
    }

    pub async fn delete_volume(
        &self,
        volume_id: &str,
    ) -> Result<
        crate::models::DeleteVolumeResponse,
        crate::apis::Error<volumes_api::DeleteVolumeError>,
    > {
        volumes_api::delete_volume(&self.config, volume_id).await
    }

    pub async fn update_volume(
        &self,
        volume_id: &str,
        update_volume: crate::models::UpdateVolume,
    ) -> Result<crate::models::VolumeResponse, crate::apis::Error<volumes_api::UpdateVolumeError>>
    {
        volumes_api::update_volume(&self.config, volume_id, update_volume).await
    }

    pub async fn list_volume_files(
        &self,
        volume_id: &str,
    ) -> Result<
        crate::models::FilesListResponse,
        crate::apis::Error<volumes_api::ListVolumeFilesError>,
    > {
        volumes_api::list_volume_files(&self.config, volume_id).await
    }

    pub async fn get_volume_contents(
        &self,
        volume_id: &str,
        path: &str,
        zip: Option<bool>,
    ) -> Result<serde_json::Value, crate::apis::Error<volumes_api::GetVolumeContentsError>> {
        volumes_api::get_volume_contents(&self.config, volume_id, path, zip).await
    }

    pub async fn upload_to_volume(
        &self,
        volume_id: &str,
        path: &str,
        file: std::path::PathBuf,
    ) -> Result<
        crate::models::FilesUploadResponse,
        crate::apis::Error<volumes_api::UploadToVolumeError>,
    > {
        volumes_api::upload_to_volume(&self.config, volume_id, path, file).await
    }
}

/// Snapshots API client
#[derive(Clone)]
pub struct SnapshotsClient {
    config: Arc<configuration::Configuration>,
}

impl SnapshotsClient {
    pub async fn list_snapshots(
        &self,
    ) -> Result<
        crate::models::SnapshotsResponse,
        crate::apis::Error<snapshots_api::ListSnapshotsError>,
    > {
        snapshots_api::list_snapshots(&self.config).await
    }

    pub async fn create_snapshot(
        &self,
        new_snapshot: crate::models::NewSnapshot,
    ) -> Result<
        crate::models::SnapshotResponse,
        crate::apis::Error<snapshots_api::CreateSnapshotError>,
    > {
        snapshots_api::create_snapshot(&self.config, new_snapshot).await
    }

    pub async fn get_snapshot(
        &self,
        snapshot_id: &str,
    ) -> Result<crate::models::SnapshotResponse, crate::apis::Error<snapshots_api::GetSnapshotError>>
    {
        snapshots_api::get_snapshot(&self.config, snapshot_id).await
    }

    pub async fn delete_snapshot(
        &self,
        snapshot_id: &str,
    ) -> Result<
        crate::models::DeleteSnapshotResponse,
        crate::apis::Error<snapshots_api::DeleteSnapshotError>,
    > {
        snapshots_api::delete_snapshot(&self.config, snapshot_id).await
    }

    pub async fn update_snapshot(
        &self,
        snapshot_id: &str,
        update_snapshot: crate::models::UpdateSnapshot,
    ) -> Result<
        crate::models::SnapshotResponse,
        crate::apis::Error<snapshots_api::UpdateSnapshotError>,
    > {
        snapshots_api::update_snapshot(&self.config, snapshot_id, update_snapshot).await
    }

    pub async fn list_snapshot_files(
        &self,
        snapshot_id: &str,
    ) -> Result<
        crate::models::FilesListResponse,
        crate::apis::Error<snapshots_api::ListSnapshotFilesError>,
    > {
        snapshots_api::list_snapshot_files(&self.config, snapshot_id).await
    }

    pub async fn get_snapshot_contents(
        &self,
        snapshot_id: &str,
        path: &str,
        zip: Option<bool>,
    ) -> Result<serde_json::Value, crate::apis::Error<snapshots_api::GetSnapshotContentsError>>
    {
        snapshots_api::get_snapshot_contents(&self.config, snapshot_id, path, zip).await
    }
}

/// Images API client
#[derive(Clone)]
pub struct ImagesClient {
    config: Arc<configuration::Configuration>,
}

impl ImagesClient {
    pub async fn get_images(
        &self,
    ) -> Result<crate::models::ImagesResponse, crate::apis::Error<images_api::GetImagesError>> {
        images_api::get_images(&self.config).await
    }

    pub async fn add_image(
        &self,
        new_image: crate::models::NewImage,
    ) -> Result<crate::models::ImageResponse, crate::apis::Error<images_api::AddImageError>> {
        images_api::add_image(&self.config, new_image).await
    }

    pub async fn add_images(
        &self,
        new_image: Vec<crate::models::NewImage>,
        skip_duplicates: Option<bool>,
    ) -> Result<crate::models::ImagesResponse, crate::apis::Error<images_api::AddImagesError>> {
        images_api::add_images(&self.config, new_image, skip_duplicates).await
    }

    pub async fn get_image(
        &self,
        image_id: &str,
    ) -> Result<crate::models::ImageResponse, crate::apis::Error<images_api::GetImageError>> {
        images_api::get_image(&self.config, image_id).await
    }

    pub async fn delete_image(
        &self,
        image_id: &str,
    ) -> Result<crate::models::ImageDeleteResponse, crate::apis::Error<images_api::DeleteImageError>>
    {
        images_api::delete_image(&self.config, image_id).await
    }
}

/// Permissions API client
#[derive(Clone)]
pub struct PermissionsClient {
    config: Arc<configuration::Configuration>,
}

impl PermissionsClient {
    pub async fn get_pod_permissions(
        &self,
        pod_id: &str,
    ) -> Result<
        crate::models::PodPermissionsResponse,
        crate::apis::Error<permissions_api::GetPodPermissionsError>,
    > {
        permissions_api::get_pod_permissions(&self.config, pod_id).await
    }

    pub async fn set_pod_permission(
        &self,
        pod_id: &str,
        set_permission: crate::models::SetPermission,
    ) -> Result<
        crate::models::PodPermissionsResponse,
        crate::apis::Error<permissions_api::SetPodPermissionError>,
    > {
        permissions_api::set_pod_permission(&self.config, pod_id, set_permission).await
    }

    pub async fn delete_pod_permission(
        &self,
        pod_id: &str,
        user: &str,
    ) -> Result<
        crate::models::PodPermissionsResponse,
        crate::apis::Error<permissions_api::DeletePodPermissionError>,
    > {
        permissions_api::delete_pod_permission(&self.config, pod_id, user).await
    }

    pub async fn get_template_permissions(
        &self,
        template_id: &str,
    ) -> Result<
        crate::models::TemplatePermissionsResponse,
        crate::apis::Error<permissions_api::GetTemplatePermissionsError>,
    > {
        permissions_api::get_template_permissions(&self.config, template_id).await
    }

    pub async fn set_template_permission(
        &self,
        template_id: &str,
        set_permission: crate::models::SetPermission,
    ) -> Result<
        crate::models::TemplatePermissionsResponse,
        crate::apis::Error<permissions_api::SetTemplatePermissionError>,
    > {
        permissions_api::set_template_permission(&self.config, template_id, set_permission).await
    }

    pub async fn delete_template_permission(
        &self,
        template_id: &str,
        user: &str,
    ) -> Result<
        crate::models::TemplatePermissionsResponse,
        crate::apis::Error<permissions_api::DeleteTemplatePermissionError>,
    > {
        permissions_api::delete_template_permission(&self.config, template_id, user).await
    }

    pub async fn get_volume_permissions(
        &self,
        volume_id: &str,
    ) -> Result<
        crate::models::VolumePermissionsResponse,
        crate::apis::Error<permissions_api::GetVolumePermissionsError>,
    > {
        permissions_api::get_volume_permissions(&self.config, volume_id).await
    }

    pub async fn set_volume_permission(
        &self,
        volume_id: &str,
        set_permission: crate::models::SetPermission,
    ) -> Result<
        crate::models::VolumePermissionsResponse,
        crate::apis::Error<permissions_api::SetVolumePermissionError>,
    > {
        permissions_api::set_volume_permission(&self.config, volume_id, set_permission).await
    }

    pub async fn delete_volume_permission(
        &self,
        volume_id: &str,
        user: &str,
    ) -> Result<
        crate::models::VolumePermissionsResponse,
        crate::apis::Error<permissions_api::DeleteVolumePermissionError>,
    > {
        permissions_api::delete_volume_permission(&self.config, volume_id, user).await
    }

    pub async fn get_snapshot_permissions(
        &self,
        snapshot_id: &str,
    ) -> Result<
        crate::models::SnapshotPermissionsResponse,
        crate::apis::Error<permissions_api::GetSnapshotPermissionsError>,
    > {
        permissions_api::get_snapshot_permissions(&self.config, snapshot_id).await
    }

    pub async fn set_snapshot_permission(
        &self,
        snapshot_id: &str,
        set_permission: crate::models::SetPermission,
    ) -> Result<
        crate::models::SnapshotPermissionsResponse,
        crate::apis::Error<permissions_api::SetSnapshotPermissionError>,
    > {
        permissions_api::set_snapshot_permission(&self.config, snapshot_id, set_permission).await
    }

    pub async fn delete_snapshot_permission(
        &self,
        snapshot_id: &str,
        user: &str,
    ) -> Result<
        crate::models::SnapshotPermissionsResponse,
        crate::apis::Error<permissions_api::DeleteSnapshotPermissionError>,
    > {
        permissions_api::delete_snapshot_permission(&self.config, snapshot_id, user).await
    }
}

/// Jupyter API client
#[derive(Clone)]
pub struct JupyterClient {
    config: Arc<configuration::Configuration>,
}

impl JupyterClient {
    pub async fn ensure_jupyter_pod(
        &self,
    ) -> Result<serde_json::Value, crate::apis::Error<jupyter_api::EnsureJupyterPodError>> {
        jupyter_api::ensure_jupyter_pod(&self.config).await
    }

    pub async fn upload_to_jupyter(
        &self,
        pod_id: &str,
        file: std::path::PathBuf,
        path: &str,
    ) -> Result<serde_json::Value, crate::apis::Error<jupyter_api::UploadToJupyterError>> {
        jupyter_api::upload_to_jupyter(&self.config, pod_id, file, path).await
    }
}

/// Misc API client
#[derive(Clone)]
pub struct MiscClient {
    config: Arc<configuration::Configuration>,
}

impl MiscClient {
    pub async fn error_handler(
        &self,
        status: &str,
    ) -> Result<serde_json::Value, crate::apis::Error<misc_api::ErrorHandlerError>> {
        misc_api::error_handler(&self.config, status).await
    }

    pub async fn healthcheck(
        &self,
    ) -> Result<serde_json::Value, crate::apis::Error<misc_api::HealthcheckError>> {
        misc_api::healthcheck(&self.config).await
    }

    pub async fn traefik_config(
        &self,
    ) -> Result<serde_json::Value, crate::apis::Error<misc_api::TraefikConfigError>> {
        misc_api::traefik_config(&self.config).await
    }
}
