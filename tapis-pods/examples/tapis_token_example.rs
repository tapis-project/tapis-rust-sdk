// Example: Using the TapisPods high-level client wrapper
//
// The TapisPods wrapper provides a clean API to interact with all Pods services
// with automatic JWT token authentication via X-Tapis-Token header.

use tapis_pods::client::TapisPods;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 TapisPods Client Example");
    println!();

    // Initialize the client with base URL and JWT token from environment
    let jwt_token = std::env::var("TAPIS_TOKEN").expect("TAPIS_TOKEN environment variable not set");

    let client = TapisPods::new("https://dev.develop.tapis.io/v3", &jwt_token)?;

    println!("✓ TapisPods client initialized successfully!");
    println!();

    // Example 1: List all pods
    println!("📋 Listing all pods...");
    println!("{}", "─".repeat(60));

    match client.pods.list_pods().await {
        Ok(response) => {
            println!("✓ Successfully retrieved pods list!");
            println!("Message: {}", response.message);
            println!("Status: {}", response.status);
            if !response.result.is_empty() {
                println!("Number of pods: {}", response.result.len());
                for pod in response.result.iter().take(3) {
                    println!("  - Pod ID: {}", pod.pod_id);
                    if let Some(status) = &pod.status {
                        println!("    Status: {}", status);
                    }
                }
                if response.result.len() > 3 {
                    println!("  ... and {} more", response.result.len() - 3);
                }
            } else {
                println!("No pods found");
            }
        }
        Err(e) => {
            println!("✗ Error listing pods: {}", e);
        }
    }
    println!();

    // Example 2: Show how to use other API modules
    println!("📚 Available API modules:");
    println!("{}", "─".repeat(60));
    println!("✓ client.pods       - Pod management (create, list, delete, etc.)");
    println!("✓ client.templates  - Template management");
    println!("✓ client.volumes    - Volume management");
    println!("✓ client.snapshots  - Snapshot management");
    println!("✓ client.images     - Image management");
    println!("✓ client.permissions - Permission management");
    println!("✓ client.jupyter    - Jupyter pod operations");
    println!("✓ client.misc       - Miscellaneous operations");
    println!();

    // Example 3: Show example usage patterns
    println!("💡 Usage patterns:");
    println!("{}", "─".repeat(60));
    println!("// List templates");
    println!("let templates = client.templates.list_templates().await?;");
    println!();
    println!("// Create a pod");
    println!("let new_pod = NewPod {{ /* fields */ }};");
    println!("let pod = client.pods.create_pod(new_pod).await?;");
    println!();
    println!("// Get pod details");
    println!("let pod = client.pods.get_pod(\"pod-id\").await?;");
    println!();
    println!("// List volumes");
    println!("let volumes = client.volumes.list_volumes().await?;");
    println!();

    println!("✓ Example completed successfully!");
    Ok(())
}
