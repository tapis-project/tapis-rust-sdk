# Complete Workflow Example: Volume, File Upload, and Pod with Advanced Networking

This document demonstrates a complete workflow for:
1. Creating a persistent volume
2. Uploading files to the volume
3. Creating a pod with the volume mounted
4. Configuring advanced networking with Tapis authentication and header forwarding

## Table of Contents
- [Prerequisites](#prerequisites)
- [Complete Example Code](#complete-example-code)
- [Step-by-Step Explanation](#step-by-step-explanation)
  - [1. Initialize the Client](#1-initialize-the-client)
  - [2. Create a Volume](#2-create-a-volume)
  - [3. Upload Files to Volume](#3-upload-files-to-volume)
  - [4. Create Pod with Volume and Networking](#4-create-pod-with-volume-and-networking)
  - [5. Access the Pod](#5-access-the-pod)
- [Understanding tapis_auth_response_headers](#understanding-tapis_auth_response_headers)
- [Network Configuration Options](#network-configuration-options)
- [Cleanup](#cleanup)

## Prerequisites

```toml
[dependencies]
tapis_pods = "1.0.0"
tokio = { version = "1", features = ["full"] }
serde_json = "1.0"
```

## Complete Example Code

```rust
use tapis_pods::client::TapisPods;
use tapis_pods::models::{
    NewVolume, NewPod, ModelsPodsVolumeMount, 
    ModelsPodsNetworking, ModelsPodsResources
};
use std::collections::HashMap;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the TapisPods client
    let base_url = "https://tacc.tapis.io/v3/pods";
    let jwt_token = std::env::var("TAPIS_JWT_TOKEN")
        .expect("TAPIS_JWT_TOKEN environment variable not set");
    
    let client = TapisPods::new(base_url, &jwt_token)?;
    
    println!("🚀 Starting Complete Tapis Pods Workflow");
    println!("═══════════════════════════════════════════\n");
    
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // Step 1: Create a Persistent Volume
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    println!("📦 Step 1: Creating a persistent volume...");
    
    let new_volume = NewVolume {
        volume_id: "my-app-data".to_string(),
        description: Some("Storage for application data and configuration files".to_string()),
        size_limit: Some(1000), // 1GB in MB
        ..Default::default()
    };
    
    let volume_response = client.volumes.create_volume(new_volume).await?;
    let volume_id = &volume_response.result.volume_id;
    
    println!("✓ Volume created successfully!");
    println!("  Volume ID: {}", volume_id);
    println!("  Status: {:?}", volume_response.result.status);
    println!();
    
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // Step 2: Upload Files to the Volume
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    println!("📤 Step 2: Uploading files to volume...");
    
    // Create a sample configuration file
    let config_content = r#"{
    "app_name": "My Tapis Application",
    "version": "1.0.0",
    "settings": {
        "debug": false,
        "port": 8000
    }
}"#;
    std::fs::write("/tmp/config.json", config_content)?;
    
    // Upload the configuration file
    let upload_response = client.volumes.upload_to_volume(
        volume_id,
        "config",  // Remote path in volume
        PathBuf::from("/tmp/config.json")
    ).await?;
    
    println!("✓ File uploaded successfully!");
    println!("  File: {:?}", upload_response.result.file);
    println!("  Message: {}", upload_response.message);
    println!();
    
    // You can upload multiple files
    println!("📤 Uploading additional files...");
    
    let script_content = r#"#!/bin/bash
echo "Starting application..."
python3 /app/main.py
"#;
    std::fs::write("/tmp/startup.sh", script_content)?;
    
    client.volumes.upload_to_volume(
        volume_id,
        "scripts",
        PathBuf::from("/tmp/startup.sh")
    ).await?;
    
    println!("✓ Additional files uploaded!");
    println!();
    
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // Step 3: Create Pod with Volume Mount and Advanced Networking
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    println!("🚢 Step 3: Creating pod with volume and advanced networking...");
    
    // Configure volume mounts
    let mut volume_mounts = HashMap::new();
    volume_mounts.insert(
        volume_id.clone(),
        ModelsPodsVolumeMount {
            r#type: Some("tapisvolume".to_string()),
            mount_path: Some("/data".to_string()),  // Mount at /data in the pod
            sub_path: None,
        }
    );
    
    // Configure advanced networking with Tapis authentication
    let mut networking = HashMap::new();
    
    // Create authentication response headers
    // These headers will be forwarded to your pod when a user accesses it
    let mut auth_headers = HashMap::new();
    auth_headers.insert("X-Tapis-User".to_string(), "username".to_string());
    auth_headers.insert("X-Tapis-Tenant".to_string(), "tenant_id".to_string());
    auth_headers.insert("X-Tapis-Token".to_string(), "access_token".to_string());
    auth_headers.insert("X-User-Email".to_string(), "email".to_string());
    auth_headers.insert("X-User-Full-Name".to_string(), "full_name".to_string());
    
    networking.insert(
        "web".to_string(),  // Networking endpoint name
        ModelsPodsNetworking {
            protocol: Some("http".to_string()),
            port: Some(8000),
            
            // Enable Tapis authentication
            tapis_auth: Some(true),
            
            // ⭐ The key feature: Forward user information as headers to your pod
            // Your application running in the pod will receive these headers
            // containing authenticated user information
            tapis_auth_response_headers: Some(auth_headers),
            
            // Restrict access to specific users (supports regex patterns)
            tapis_auth_allowed_users: Some(vec![
                "specific_user".to_string(),
                "admin_.*".to_string(),  // Regex: any user starting with "admin_"
            ]),
            
            // Where to redirect after authentication
            tapis_auth_return_path: Some("/dashboard".to_string()),
            
            // IP allow list (optional - leave empty to allow all IPs)
            ip_allow_list: Some(vec![]),  // Empty = allow all
            
            // CORS configuration for web applications
            cors_allow_origins: Some(vec![
                "https://tacc.tapis.io".to_string(),
                "https://tacc.develop.tapis.io".to_string(),
            ]),
            cors_allow_methods: Some(vec![
                "GET".to_string(),
                "POST".to_string(),
                "PUT".to_string(),
                "DELETE".to_string(),
            ]),
            cors_allow_headers: Some(vec![
                "Content-Type".to_string(),
                "X-Tapis-Token".to_string(),
                "Authorization".to_string(),
            ]),
            cors_allow_credentials: Some(true),
            cors_max_age: Some(3600),
            
            // UI configuration
            tapis_ui_uri: Some("/app".to_string()),
            tapis_ui_uri_redirect: Some(true),
            tapis_ui_uri_description: Some("Application Dashboard".to_string()),
            
            // URL will be populated by the service
            url: None,
        }
    );
    
    // Add a secondary endpoint for direct API access (no auth)
    networking.insert(
        "api".to_string(),
        ModelsPodsNetworking {
            protocol: Some("http".to_string()),
            port: Some(8001),
            tapis_auth: Some(false),  // No authentication for this endpoint
            ip_allow_list: Some(vec![
                "10.0.0.0/8".to_string(),  // Only allow internal network
            ]),
            ..Default::default()
        }
    );
    
    // Configure pod resources
    let resources = ModelsPodsResources {
        cpu_limit: Some(2000),      // 2 CPU cores (in millicores)
        mem_limit: Some(4096),      // 4GB RAM (in MB)
        cpu_request: Some(500),     // Minimum 0.5 CPU
        mem_request: Some(1024),    // Minimum 1GB RAM
        gpus: Some(0),
        ..Default::default()
    };
    
    // Environment variables for the pod
    let mut env_vars = serde_json::Map::new();
    env_vars.insert("APP_ENV".to_string(), serde_json::json!("production"));
    env_vars.insert("DATA_PATH".to_string(), serde_json::json!("/data/config"));
    env_vars.insert("LOG_LEVEL".to_string(), serde_json::json!("info"));
    
    // Create the pod
    let new_pod = NewPod {
        pod_id: "my-secure-app".to_string(),
        image: Some("python:3.11-slim".to_string()),
        description: Some("Application with persistent storage and secure access".to_string()),
        
        // Startup command
        command: Some(vec![
            "/bin/bash".to_string(),
            "-c".to_string(),
        ]),
        arguments: Some(vec![
            "chmod +x /data/scripts/startup.sh && /data/scripts/startup.sh".to_string(),
        ]),
        
        environment_variables: Some(serde_json::Value::Object(env_vars)),
        volume_mounts: Some(volume_mounts),
        networking: Some(networking),
        resources: Some(resources),
        
        status_requested: Some("ON".to_string()),
        time_to_stop_default: Some(86400),  // 24 hours
        time_to_stop_instance: None,        // Use default
        
        template: None,
        compute_queue: Some("default".to_string()),
    };
    
    let pod_response = client.pods.create_pod(new_pod).await?;
    let pod_id = &pod_response.result.pod_id;
    
    println!("✓ Pod created successfully!");
    println!("  Pod ID: {}", pod_id);
    println!("  Status: {:?}", pod_response.result.status);
    println!();
    
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // Step 4: Wait for Pod to be Running and Get Access URLs
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    println!("⏳ Step 4: Waiting for pod to be running...");
    
    // Poll for pod status
    for attempt in 1..=10 {
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        
        let pod_details = client.pods.get_pod(pod_id).await?;
        let status = &pod_details.result.status;
        
        println!("  Attempt {}/10 - Status: {:?}", attempt, status);
        
        if status == "RUNNING" {
            println!("\n✓ Pod is now running!");
            
            // Display access URLs
            if let Some(networking) = &pod_details.result.networking {
                println!("\n🌐 Access URLs:");
                println!("───────────────────────────────────────────────────────");
                for (name, net_config) in networking.iter() {
                    if let Some(url) = &net_config.url {
                        println!("  {} endpoint: {}", name, url);
                        
                        if net_config.tapis_auth.unwrap_or(false) {
                            println!("    🔒 Protected with Tapis authentication");
                            println!("    📝 Forwards user info as headers to your app");
                        }
                    }
                }
                println!("───────────────────────────────────────────────────────");
            }
            break;
        }
    }
    
    println!();
    
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // Step 5: Demonstrate Volume Contents and Pod Logs
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    println!("📋 Step 5: Checking volume contents and pod logs...");
    
    // List files in the volume
    let volume_files = client.volumes.get_volume_contents(volume_id, "/").await?;
    println!("\n📁 Volume Contents:");
    for file in volume_files.result.iter() {
        println!("  - {}", file.name);
    }
    
    // Get pod logs
    println!("\n📜 Pod Logs (last 50 lines):");
    let logs_response = client.pods.get_pod_logs(pod_id).await?;
    println!("{}", logs_response.result.logs);
    
    println!("\n══════════════════════════════════════════");
    println!("✓ Complete workflow finished successfully!");
    println!("══════════════════════════════════════════");
    
    Ok(())
}
```

## Step-by-Step Explanation

### 1. Initialize the Client

```rust
let base_url = "https://tacc.tapis.io/v3/pods";
let jwt_token = std::env::var("TAPIS_JWT_TOKEN")?;
let client = TapisPods::new(base_url, &jwt_token)?;
```

The client automatically includes your JWT token in all API requests via the `X-Tapis-Token` header.

### 2. Create a Volume

Volumes provide persistent storage that survives pod restarts:

```rust
let new_volume = NewVolume {
    volume_id: "my-app-data".to_string(),
    description: Some("Storage for application data".to_string()),
    size_limit: Some(1000), // 1GB
    ..Default::default()
};

let volume = client.volumes.create_volume(new_volume).await?;
```

### 3. Upload Files to Volume

Upload configuration files, scripts, or any data your pod needs:

```rust
// Upload a file to a specific path in the volume
client.volumes.upload_to_volume(
    "my-app-data",           // volume_id
    "config",                // remote path in volume
    PathBuf::from("/tmp/config.json")  // local file
).await?;
```

The file will be accessible at `/data/config/config.json` inside the pod (based on the mount path).

### 4. Create Pod with Volume and Networking

#### Volume Mounts

```rust
let mut volume_mounts = HashMap::new();
volume_mounts.insert(
    "my-app-data".to_string(),
    ModelsPodsVolumeMount {
        r#type: Some("tapisvolume".to_string()),
        mount_path: Some("/data".to_string()),
        sub_path: None,
    }
);
```

#### Networking Configuration

```rust
let mut networking = HashMap::new();
networking.insert("web".to_string(), ModelsPodsNetworking {
    protocol: Some("http".to_string()),
    port: Some(8000),
    tapis_auth: Some(true),
    tapis_auth_response_headers: Some(auth_headers),
    // ... other settings
});
```

### 5. Access the Pod

Once running, you can access your pod through the generated URLs. If Tapis auth is enabled, users must authenticate first, then their information is forwarded to your pod as headers.

## Understanding tapis_auth_response_headers

This is a powerful feature that enables **secure, authenticated access** to your pods while **forwarding user identity** to your application.

### How It Works

1. **User Authentication**: When `tapis_auth: true`, Tapis handles user authentication
2. **Header Forwarding**: After successful auth, Tapis extracts user information and forwards it as HTTP headers to your pod
3. **Application Access**: Your application receives the headers and knows who the authenticated user is

### Available Headers

```rust
let mut auth_headers = HashMap::new();

// User identity
auth_headers.insert("X-Tapis-User".to_string(), "username".to_string());
auth_headers.insert("X-Tapis-Tenant".to_string(), "tenant_id".to_string());

// Authentication token
auth_headers.insert("X-Tapis-Token".to_string(), "access_token".to_string());

// User profile information
auth_headers.insert("X-User-Email".to_string(), "email".to_string());
auth_headers.insert("X-User-Full-Name".to_string(), "full_name".to_string());
```

### Reading Headers in Your Application

**Python (Flask/FastAPI)**:
```python
from flask import Flask, request

app = Flask(__name__)

@app.route('/')
def home():
    username = request.headers.get('X-Tapis-User')
    email = request.headers.get('X-User-Email')
    
    return f"Welcome, {username} ({email})!"
```

**Node.js (Express)**:
```javascript
app.get('/', (req, res) => {
    const username = req.headers['x-tapis-user'];
    const email = req.headers['x-user-email'];
    
    res.send(`Welcome, ${username} (${email})!`);
});
```

**Rust (Axum)**:
```rust
use axum::{
    routing::get,
    Router,
    http::HeaderMap,
};

async fn handler(headers: HeaderMap) -> String {
    let username = headers.get("x-tapis-user")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown");
    
    format!("Welcome, {}!", username)
}
```

### Security Benefits

1. **No Token Management**: Your app doesn't need to validate JWT tokens
2. **User Context**: Automatic access to authenticated user information
3. **Access Control**: Use `tapis_auth_allowed_users` to restrict access
4. **Audit Trail**: Know exactly which user accessed what

## Network Configuration Options

### Protocol Types

- `http`: Standard HTTP traffic (most common)
- `tcp`: Raw TCP connections (databases, custom protocols)
- `postgres`: PostgreSQL database connections
- `local_only`: Only accessible within the Kubernetes cluster

### Authentication Options

| Field | Type | Purpose |
|-------|------|---------|
| `tapis_auth` | bool | Enable Tapis authentication |
| `tapis_auth_response_headers` | HashMap | Headers to forward to pod |
| `tapis_auth_allowed_users` | Vec<String> | User whitelist (supports regex) |
| `tapis_auth_return_path` | String | Post-auth redirect path |

### Access Control

```rust
// IP-based access control
ip_allow_list: Some(vec![
    "192.168.1.0/24".to_string(),  // Local network
    "10.0.0.5/32".to_string(),     // Specific IP
])

// User-based access control (regex supported)
tapis_auth_allowed_users: Some(vec![
    "alice".to_string(),           // Specific user
    "admin_.*".to_string(),        // All admin users
    ".*@tacc.utexas.edu".to_string(), // Domain pattern
])
```

### CORS Configuration

For web applications that need cross-origin requests:

```rust
cors_allow_origins: Some(vec![
    "https://myapp.com".to_string(),
    "https://dev.myapp.com".to_string(),
]),
cors_allow_methods: Some(vec![
    "GET".to_string(),
    "POST".to_string(),
    "PUT".to_string(),
    "DELETE".to_string(),
]),
cors_allow_headers: Some(vec![
    "Content-Type".to_string(),
    "Authorization".to_string(),
    "X-Custom-Header".to_string(),
]),
cors_allow_credentials: Some(true),
```

## Cleanup

When you're done, clean up resources:

```rust
// Stop and delete the pod
client.pods.delete_pod("my-secure-app", false).await?;

// Delete the volume
client.volumes.delete_volume("my-app-data", false).await?;
```

## Common Use Cases

### 1. Multi-User Web Application

```rust
networking.insert("web".to_string(), ModelsPodsNetworking {
    protocol: Some("http".to_string()),
    port: Some(3000),
    tapis_auth: Some(true),
    tapis_auth_response_headers: Some({
        let mut headers = HashMap::new();
        headers.insert("X-Tapis-User".to_string(), "username".to_string());
        headers.insert("X-Tapis-Token".to_string(), "access_token".to_string());
        headers.insert("X-User-Email".to_string(), "email".to_string());
        headers
    }),
    tapis_auth_allowed_users: Some(vec!["*".to_string()]),  // All users
    cors_allow_origins: Some(vec!["*".to_string()]),
    ..Default::default()
});
```

### 2. Admin-Only Database Interface

```rust
networking.insert("admin".to_string(), ModelsPodsNetworking {
    protocol: Some("http".to_string()),
    port: Some(8080),
    tapis_auth: Some(true),
    tapis_auth_allowed_users: Some(vec![
        "admin_.*".to_string(),  // Only admin users
    ]),
    tapis_auth_response_headers: Some({
        let mut headers = HashMap::new();
        headers.insert("X-Admin-User".to_string(), "username".to_string());
        headers
    }),
    ip_allow_list: Some(vec![
        "10.0.0.0/8".to_string(),  // Internal network only
    ]),
    ..Default::default()
});
```

### 3. Public API with Internal Management

```rust
// Public endpoint
networking.insert("api".to_string(), ModelsPodsNetworking {
    protocol: Some("http".to_string()),
    port: Some(8000),
    tapis_auth: Some(false),  // No auth required
    ..Default::default()
});

// Management endpoint
networking.insert("mgmt".to_string(), ModelsPodsNetworking {
    protocol: Some("http".to_string()),
    port: Some(9000),
    tapis_auth: Some(true),
    tapis_auth_allowed_users: Some(vec!["service_admin".to_string()]),
    ip_allow_list: Some(vec!["127.0.0.1/32".to_string()]),
    ..Default::default()
});
```

## Troubleshooting

### Volume Not Mounting
- Ensure volume is in `AVAILABLE` status before creating pod
- Check volume permissions
- Verify mount path doesn't conflict with existing directories

### Authentication Issues
- Confirm JWT token is valid and not expired
- Check `tapis_auth_allowed_users` patterns
- Verify user belongs to correct tenant

### Headers Not Received
- Ensure `tapis_auth: true` is set
- Check header names match exactly (case-sensitive)
- Confirm your application is reading headers correctly

### Connection Refused
- Wait for pod status to be `RUNNING`
- Check port matches between pod application and networking config
- Verify firewall rules and IP allow list

## Additional Resources

- [Tapis Pods Documentation](https://tapis.readthedocs.io/en/latest/technical/pods.html)
- [Client Wrapper Guide](../CLIENT_WRAPPER.md)
- [API Reference](../README.md)
- [Network Configuration](ModelsPodsNetworking.md)
- [Volume Management](VolumesApi.md)

---

**Note**: This example demonstrates the Rust SDK. The same workflow applies to other language SDKs with similar API structures.
