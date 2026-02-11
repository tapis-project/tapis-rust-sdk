# Tapis Rust SDK

[![Crates.io](https://img.shields.io/crates/v/tapis-sdk.svg)](https://crates.io/crates/tapis-sdk)
[![Documentation](https://docs.rs/tapis-sdk/badge.svg)](https://docs.rs/tapis-sdk)
[![License](https://img.shields.io/crates/l/tapis-sdk.svg)](https://github.com/tapis-project/tapis-rust-sdk/blob/main/LICENSE)

A comprehensive Rust SDK for the [Tapis Framework](https://tapis-project.org), providing type-safe async clients for Tapis v3 services.

Current release line: `0.6.0`

## About Tapis

Tapis is a distributed API framework for science and engineering research. It provides authentication, authorization, workload execution, data management, and supporting platform services across institutional resources.

## Workspace Layout

The repository is a Cargo workspace:

- `tapis-sdk` is the umbrella crate.
- Each service also has its own crate (recommended when you only need a subset).

### Components

| Module (`tapis-sdk`) | Crate | Purpose | Docs |
|---|---|---|---|
| `actors` | `tapis_actors` | Actor-based functions and executions | [docs.rs/tapis_actors](https://docs.rs/tapis_actors) |
| `apps` | `tapis_apps` | Application definitions and sharing | [docs.rs/tapis_apps](https://docs.rs/tapis_apps) |
| `authenticator` | `tapis_authenticator` | AuthN/AuthZ, clients, and token APIs | [docs.rs/tapis_authenticator](https://docs.rs/tapis_authenticator) |
| `files` | `tapis_files` | File operations, permissions, transfers | [docs.rs/tapis_files](https://docs.rs/tapis_files) |
| `globus_proxy` | `tapis_globus_proxy` | Globus proxy and transfer operations | [docs.rs/tapis_globus_proxy](https://docs.rs/tapis_globus_proxy) |
| `jobs` | `tapis_jobs` | Job submission and lifecycle management | [docs.rs/tapis_jobs](https://docs.rs/tapis_jobs) |
| `meta` | `tapis_meta` | Metadata collections and documents | [docs.rs/tapis_meta](https://docs.rs/tapis_meta) |
| `notifications` | `tapis_notifications` | Event subscriptions and notifications | [docs.rs/tapis_notifications](https://docs.rs/tapis_notifications) |
| `pgrest` | `tapis_pgrest` | Postgres REST-style data access | [docs.rs/tapis_pgrest](https://docs.rs/tapis_pgrest) |
| `pods` | `tapis_pods` | Pods, templates, volumes, snapshots | [docs.rs/tapis_pods](https://docs.rs/tapis_pods) |
| `sk` | `tapis_sk` | Security kernel and vault/secret APIs | [docs.rs/tapis_sk](https://docs.rs/tapis_sk) |
| `streams` | `tapis_streams` | Streams/channels and telemetry resources | [docs.rs/tapis_streams](https://docs.rs/tapis_streams) |
| `systems` | `tapis_systems` | Systems, credentials, scheduler profiles | [docs.rs/tapis_systems](https://docs.rs/tapis_systems) |
| `tenants` | `tapis_tenants` | Tenant, site, owner, LDAP management | [docs.rs/tapis_tenants](https://docs.rs/tapis_tenants) |
| `tokens` | `tapis_tokens` | Token service APIs | [docs.rs/tapis_tokens](https://docs.rs/tapis_tokens) |
| `workflows` | `tapis_workflows` | Workflow and pipeline orchestration | [docs.rs/tapis_workflows](https://docs.rs/tapis_workflows) |
| umbrella | `tapis-sdk` | Re-exports all modules above | [docs.rs/tapis-sdk](https://docs.rs/tapis-sdk) |

## Installation

Use the umbrella crate:

```toml
[dependencies]
tapis-sdk = "0.6.0"
tokio = { version = "1", features = ["full"] }
```

Or use individual crates:

```toml
[dependencies]
tapis_systems = "0.6.0"
tapis_apps = "0.6.0"
tapis_jobs = "0.6.0"
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
tapis-sdk = { version = "0.6.0", default-features = false, features = ["rustls-tls"] }
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
cargo publish -p tapis_actors
cargo publish -p tapis_apps
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
