# Tapis Rust SDK

[![Crates.io](https://img.shields.io/crates/v/tapis-sdk.svg)](https://crates.io/crates/tapis-sdk)
[![Documentation](https://docs.rs/tapis-sdk/badge.svg)](https://docs.rs/tapis-sdk)
[![License](https://img.shields.io/crates/l/tapis-sdk.svg)](https://github.com/tapis-project/tapis-rust-sdk/blob/main/LICENSE)

A comprehensive Rust SDK for the [Tapis Framework](https://tapis-project.org), providing type-safe async clients for Tapis v3 services.

Current release line: `0.2.0`

## About Tapis

Tapis is a distributed API framework for science and engineering research. It provides authentication, authorization, workload execution, data management, and supporting platform services across institutional resources.

## Workspace Layout

The repository is a Cargo workspace:

- `tapis-sdk` is the umbrella crate.
- Each service also has its own crate (recommended when you only need a subset).

### Components

| Module (`tapis-sdk`) | Crate | Purpose | Docs |
|---|---|---|---|
| `actors` | `tapis-actors` | Actor-based functions and executions | [docs.rs/tapis-actors](https://docs.rs/tapis-actors) |
| `apps` | `tapis-apps` | Application definitions and sharing | [docs.rs/tapis-apps](https://docs.rs/tapis-apps) |
| `authenticator` | `tapis-authenticator` | AuthN/AuthZ, clients, and token APIs | [docs.rs/tapis-authenticator](https://docs.rs/tapis-authenticator) |
| `files` | `tapis-files` | File operations, permissions, transfers | [docs.rs/tapis-files](https://docs.rs/tapis-files) |
| `globus_proxy` | `tapis-globus-proxy` | Globus proxy and transfer operations | [docs.rs/tapis-globus-proxy](https://docs.rs/tapis-globus-proxy) |
| `jobs` | `tapis-jobs` | Job submission and lifecycle management | [docs.rs/tapis-jobs](https://docs.rs/tapis-jobs) |
| `meta` | `tapis-meta` | Metadata collections and documents | [docs.rs/tapis-meta](https://docs.rs/tapis-meta) |
| `notifications` | `tapis-notifications` | Event subscriptions and notifications | [docs.rs/tapis-notifications](https://docs.rs/tapis-notifications) |
| `pgrest` | `tapis-pgrest` | Postgres REST-style data access | [docs.rs/tapis-pgrest](https://docs.rs/tapis-pgrest) |
| `pods` | `tapis-pods` | Pods, templates, volumes, snapshots | [docs.rs/tapis-pods](https://docs.rs/tapis-pods) |
| `sk` | `tapis-sk` | Security kernel and vault/secret APIs | [docs.rs/tapis-sk](https://docs.rs/tapis-sk) |
| `streams` | `tapis-streams` | Streams/channels and telemetry resources | [docs.rs/tapis-streams](https://docs.rs/tapis-streams) |
| `systems` | `tapis-systems` | Systems, credentials, scheduler profiles | [docs.rs/tapis-systems](https://docs.rs/tapis-systems) |
| `tenants` | `tapis-tenants` | Tenant, site, owner, LDAP management | [docs.rs/tapis-tenants](https://docs.rs/tapis-tenants) |
| `tokens` | `tapis-tokens` | Token service APIs | [docs.rs/tapis-tokens](https://docs.rs/tapis-tokens) |
| `workflows` | `tapis-workflows` | Workflow and pipeline orchestration | [docs.rs/tapis-workflows](https://docs.rs/tapis-workflows) |
| umbrella | `tapis-sdk` | Re-exports all modules above | [docs.rs/tapis-sdk](https://docs.rs/tapis-sdk) |

## Installation

Use the umbrella crate:

```toml
[dependencies]
tapis-sdk = "0.2.0"
tokio = { version = "1", features = ["full"] }
```

Or use individual crates:

```toml
[dependencies]
tapis-systems = "0.2.0"
tapis-apps = "0.2.0"
tapis-jobs = "0.2.0"
tokio = { version = "1", features = ["full"] }
```

## Authentication and Client Initialization

Wrappers accept optional token injection:

- `TapisAuthenticator::new(base_url, None)` works for endpoints that do not require `X-Tapis-Token`.
- Other services typically use `Some(token)`.

```rust
use tapis_sdk::authenticator::{models, TapisAuthenticator};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_url = std::env::var("TAPIS_BASE_URL")
        .unwrap_or_else(|_| "https://dev.develop.tapis.io/v3".to_string());

    let authenticator = TapisAuthenticator::new(&base_url, None)?;

    let mut req = models::NewToken::new();
    req.username = Some(std::env::var("TAPIS_USERNAME")?);
    req.password = Some(std::env::var("TAPIS_PASSWORD")?);
    req.grant_type = Some("password".to_string());

    let token_resp = authenticator.tokens.create_token(req).await?;
    println!("Token response status: {:?}", token_resp.status);

    Ok(())
}
```

## Systems, Apps, and Jobs Workflow

The snippets below demonstrate a common flow:

1. Create a system
2. Create an app that references that system
3. Submit a job
4. Monitor job status until completion

Notes:

- Payload requirements can vary by tenant policy.
- The examples intentionally show minimal payloads that match the generated Rust types.

### 1) Create a System

```rust
use tapis_sdk::systems::{models, TapisSystems};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_url = std::env::var("TAPIS_BASE_URL")
        .unwrap_or_else(|_| "https://dev.develop.tapis.io/v3".to_string());
    let token = std::env::var("TAPIS_TOKEN")?;

    let systems = TapisSystems::new(&base_url, Some(token.as_str()))?;

    let system_id = "sdk-demo-system".to_string();
    let mut req = models::ReqPostSystem::new(
        system_id,
        models::SystemTypeEnum::Linux,
        "login.example.org".to_string(),
        models::AuthnEnum::Password,
        true,
    );
    req.description = Some("Created by tapis-rust-sdk README example".to_string());
    req.root_dir = Some("/home/${apiUserId}".to_string());
    req.enabled = Some(true);

    let resp = systems.systems.create_system(req, Some(true)).await?;
    println!("Create system URL: {:?}", resp.result.and_then(|r| r.url));

    Ok(())
}
```

### 2) Create an App

```rust
use tapis_sdk::apps::{models, TapisApps};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_url = std::env::var("TAPIS_BASE_URL")
        .unwrap_or_else(|_| "https://dev.develop.tapis.io/v3".to_string());
    let token = std::env::var("TAPIS_TOKEN")?;

    let apps = TapisApps::new(&base_url, Some(token.as_str()))?;

    let mut req = models::ReqPostApp::new(
        "sdk-demo-app".to_string(),
        "1.0.0".to_string(),
        "docker://alpine:3.20".to_string(),
    );
    req.description = Some("Created by tapis-rust-sdk README example".to_string());
    req.runtime = Some(models::RuntimeEnum::Docker);
    req.enabled = Some(true);
    req.version_enabled = Some(true);

    let resp = apps.applications.create_app_version(req).await?;
    println!("Create app URL: {:?}", resp.result.and_then(|r| r.url));

    Ok(())
}
```

### 3) Submit and Monitor a Job

```rust
use std::time::Duration;
use tapis_sdk::jobs::{models, TapisJobs};
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_url = std::env::var("TAPIS_BASE_URL")
        .unwrap_or_else(|_| "https://dev.develop.tapis.io/v3".to_string());
    let token = std::env::var("TAPIS_TOKEN")?;

    let jobs = TapisJobs::new(&base_url, Some(token.as_str()))?;

    let mut submit = models::ReqSubmitJob::new(
        "sdk-demo-job".to_string(),
        "sdk-demo-app".to_string(),
        "1.0.0".to_string(),
    );
    submit.exec_system_id = Some("sdk-demo-system".to_string());
    submit.archive_system_id = Some("sdk-demo-system".to_string());
    submit.archive_mode = Some(models::ArchiveModeEnum::SkipOnFail);

    let submit_resp = jobs.jobs.submit_job(submit).await?;
    let job_uuid = submit_resp
        .result
        .and_then(|j| j.uuid)
        .ok_or("submit_job response did not include a job UUID")?;

    println!("Submitted job UUID: {job_uuid}");

    loop {
        let status_resp = jobs.jobs.get_job_status(&job_uuid).await?;
        let status = status_resp
            .result
            .and_then(|s| s.status)
            .unwrap_or_else(|| "UNKNOWN".to_string());

        println!("Job {job_uuid} status: {status}");

        if matches!(status.as_str(), "FINISHED" | "FAILED" | "CANCELLED") {
            break;
        }

        sleep(Duration::from_secs(5)).await;
    }

    Ok(())
}
```

## Pods Examples

### List Pods and Read Pod Status

```rust
use tapis_sdk::pods::TapisPods;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_url = std::env::var("TAPIS_BASE_URL")
        .unwrap_or_else(|_| "https://dev.develop.tapis.io/v3".to_string());
    let token = std::env::var("TAPIS_TOKEN")?;

    let pods = TapisPods::new(&base_url, Some(token.as_str()))?;
    let resp = pods.pods.list_pods().await?;

    println!("Found {} pods", resp.result.len());
    for pod in resp.result {
        println!("{} => {:?}", pod.pod_id, pod.status);
    }

    Ok(())
}
```

### Create and Fetch a Pod

```rust
use tapis_sdk::pods::{models, TapisPods};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_url = std::env::var("TAPIS_BASE_URL")
        .unwrap_or_else(|_| "https://dev.develop.tapis.io/v3".to_string());
    let token = std::env::var("TAPIS_TOKEN")?;

    let pods = TapisPods::new(&base_url, Some(token.as_str()))?;

    let mut req = models::NewPod::new("sdk-demo-pod".to_string());
    req.image = Some("ubuntu:22.04".to_string());
    req.description = Some("Created by tapis-rust-sdk README example".to_string());

    let create_resp = pods.pods.create_pod(req).await?;
    println!("Created pod: {}", create_resp.result.pod_id);

    let get_resp = pods.pods.get_pod("sdk-demo-pod", None, None).await?;
    println!("Fetched pod status: {:?}", get_resp.result.status);

    Ok(())
}
```

### Full Pods Lifecycle Example (Volume + Pod + Cleanup)

Use the repository example:

- `tapis-pods/examples/tapis_pods_example.rs`

It demonstrates creating a volume, creating a pod using that volume, deleting the pod, then deleting the volume.

## Module Imports (Umbrella Crate)

```rust
use tapis_sdk::authenticator::TapisAuthenticator;
use tapis_sdk::pods::TapisPods;
use tapis_sdk::systems::TapisSystems;
use tapis_sdk::apps::TapisApps;
use tapis_sdk::jobs::TapisJobs;
```

## TLS Features

```toml
[dependencies]
tapis-sdk = { version = "0.2.0", default-features = false, features = ["rustls-tls"] }
```

Available TLS features:

- `native-tls` (default)
- `rustls-tls`

## Examples in This Repository

Each sub-crate contains runnable examples under `examples/`.

Examples to start with:

- `tapis-authenticator/examples/authenticator_example.rs`
- `tapis-systems/examples/systems_basic_example.rs`
- `tapis-apps/examples/apps_basic_example.rs`
- `tapis-jobs/examples/jobs_basic_example.rs`
- `tapis-pods/examples/tapis_pods_example.rs`

Run one example:

```bash
cd tapis-pods
cargo run --example tapis_pods_example
```

## Development

Build all crates:

```bash
cargo build --workspace --all-targets
```

Run tests:

```bash
cargo test --workspace
```

## Publishing

Publish service crates before publishing the parent crate:

```bash
cargo publish -p tapis-actors
cargo publish -p tapis-apps
# ...publish remaining service crates...
cargo publish -p tapis-sdk
```

## License

This project is licensed under the BSD-3-Clause License. See `LICENSE` for details.

## Resources

- [Tapis Project Website](https://tapis-project.org)
- [Tapis Documentation](https://tapis.readthedocs.io/)
- [SDK on docs.rs](https://docs.rs/tapis-sdk)
- [GitHub Repository](https://github.com/tapis-project/tapis-rust-sdk)

## Support

For support, contact `cicsupport@tacc.utexas.edu`.
