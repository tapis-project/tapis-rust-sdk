use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

use tapis_pods::client::TapisPods;
use tapis_pods::models;

fn unique_suffix() -> String {
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    ts.to_string()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("TapisPods lifecycle example");

    let jwt_token =
        std::env::var("TAPIS_TOKEN").expect("TAPIS_TOKEN environment variable must be set");
    let base_url = std::env::var("TAPIS_BASE_URL")
        .unwrap_or_else(|_| "https://dev.develop.tapis.io/v3".to_string());
    let pod_image = std::env::var("TAPIS_POD_IMAGE").unwrap_or_else(|_| "ubuntu:22.04".to_string());

    let client = TapisPods::new(&base_url, &jwt_token)?;

    let suffix = unique_suffix();
    let volume_id = format!("example-volume-{suffix}");
    let pod_id = format!("example-pod-{suffix}");

    let mut created_pod = false;

    println!("Creating volume: {volume_id}");
    let mut new_volume = models::NewVolume::new(volume_id.clone());
    new_volume.description = Some("Created by tapis_pods_example.rs".to_string());
    new_volume.size_limit = Some(1024);

    match client.volumes.create_volume(new_volume).await {
        Ok(resp) => {
            println!(
                "Volume created: {} (status: {})",
                resp.result.volume_id, resp.status
            );
        }
        Err(err) => {
            eprintln!("Failed to create volume: {err}");
            return Err(err.into());
        }
    }

    println!("Creating pod: {pod_id} with mounted volume: {volume_id}");
    let mut mount = models::VolumeMountsValue::new(models::volume_mounts_value::Type::Tapisvolume);
    mount.source_id = Some(Some(volume_id.clone()));
    mount.read_only = Some(Some(false));

    let mut volume_mounts = HashMap::new();
    volume_mounts.insert("/workspace".to_string(), mount);

    let mut new_pod = models::NewPod::new(pod_id.clone());
    new_pod.image = Some(pod_image.clone());
    new_pod.description = Some("Created by tapis_pods_example.rs".to_string());
    new_pod.status_requested = Some("OFF".to_string());
    new_pod.volume_mounts = Some(volume_mounts);

    match client.pods.create_pod(new_pod).await {
        Ok(resp) => {
            created_pod = true;
            println!(
                "Pod created: {} (status: {})",
                resp.result.pod_id, resp.status
            );
        }
        Err(err) => {
            eprintln!("Failed to create pod: {err}");
        }
    }

    if created_pod {
        println!("Deleting pod: {pod_id}");
        match client.pods.delete_pod(&pod_id).await {
            Ok(resp) => {
                println!("Pod deleted: {} (status: {})", resp.result, resp.status);
            }
            Err(err) => {
                eprintln!("Failed to delete pod {pod_id}: {err}");
            }
        }
    }

    println!("Deleting volume: {volume_id}");
    match client.volumes.delete_volume(&volume_id).await {
        Ok(resp) => {
            println!("Volume deleted: {} (status: {})", resp.result, resp.status);
        }
        Err(err) => {
            eprintln!("Failed to delete volume {volume_id}: {err}");
        }
    }

    println!("Example completed.");
    Ok(())
}
