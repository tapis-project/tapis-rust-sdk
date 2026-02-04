# Tapis Rust SDK

[![Crates.io](https://img.shields.io/crates/v/tapis-sdk.svg)](https://crates.io/crates/tapis-sdk)
[![Documentation](https://docs.rs/tapis-sdk/badge.svg)](https://docs.rs/tapis-sdk)
[![License](https://img.shields.io/crates/l/tapis-sdk.svg)](https://github.com/tapis-project/tapis-rust-sdk/blob/main/LICENSE)

A comprehensive Rust SDK for the [Tapis Framework](https://tapis-project.org), providing type-safe, async clients for Tapis v3 services.

## About Tapis

Tapis is a distributed web-based API framework for science and engineering research and education. It provides authentication, authorization, and a wide range of digital research infrastructure services across multiple institutional and data center resources.

## Overview

The Tapis Rust SDK is structured as a workspace containing multiple service-specific crates. You can either use the umbrella `tapis-sdk` crate for convenience or depend on individual service crates as needed.

### Available Crates

| Crate | Description | Documentation |
|-------|-------------|---------------|
| **tapis-sdk** | Parent crate re-exporting all Tapis services | [![docs.rs](https://docs.rs/tapis-sdk/badge.svg)](https://docs.rs/tapis-sdk) |
| **tapis-authenticator** | Authentication and OAuth2 token management | [![docs.rs](https://docs.rs/tapis-authenticator/badge.svg)](https://docs.rs/tapis-authenticator) |
| **tapis-pods** | Pods-as-a-Service (PaaS) for Docker containers | [![docs.rs](https://docs.rs/tapis-pods/badge.svg)](https://docs.rs/tapis-pods) |

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
tapis-sdk = "0.1"
tokio = { version = "1", features = ["full"] }
```

Or use individual crates:

```toml
[dependencies]
tapis-authenticator = "0.1"
tapis-pods = "0.1"
tokio = { version = "1", features = ["full"] }
```

## Quick Start

### Authentication

Create a JWT token using username and password:

```rust
use tapis_sdk::authenticator::{TapisAuthenticator, models::NewToken};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the authenticator client
    let client = TapisAuthenticator::new(
        "https://tacc.tapis.io",
        None, // No token needed for creating tokens
    )?;

    // Create a new JWT token
    let new_token = NewToken {
        username: Some("your_username".to_string()),
        password: Some("your_password".to_string()),
        grant_type: Some("password".to_string()),
        ..Default::default()
    };

    let response = client.tokens.create_token(new_token).await?;
    
    if let Some(result) = response.result {
        if let Some(access_token) = result.access_token.access_token {
            println!("Access Token: {}", access_token);
        }
    }

    Ok(())
}
```

### List Pods

Retrieve all pods using the Pods service:

```rust
use tapis_sdk::pods::TapisPods;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the Pods client with your JWT token
    let client = TapisPods::new(
        "https://tacc.tapis.io/v3",
        "your_jwt_token_here"
    )?;

    // List all pods
    let response = client.pods.list_pods().await?;
    
    println!("Found {} pods", response.result.len());
    for pod in response.result {
        println!("Pod ID: {}", pod.pod_id);
        if let Some(status) = pod.status {
            println!("  Status: {}", status);
        }
    }

    Ok(())
}
```

### Create a Pod

Deploy a new pod from a Docker image:

```rust
use tapis_sdk::pods::{TapisPods, models::NewPod};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = TapisPods::new(
        "https://tacc.tapis.io/v3",
        "your_jwt_token_here"
    )?;

    // Create a new pod
    let new_pod = NewPod {
        pod_id: Some("my-nginx-pod".to_string()),
        pod_template: Some("template-nginx".to_string()),
        description: Some("My NGINX web server".to_string()),
        ..Default::default()
    };

    let response = client.pods.create_pod(new_pod).await?;
    
    println!("Pod created: {:?}", response.result);

    Ok(())
}
```

### Get Pod Details

```rust
use tapis_sdk::pods::TapisPods;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = TapisPods::new(
        "https://tacc.tapis.io/v3",
        "your_jwt_token_here"
    )?;

    // Get specific pod details
    let pod_id = "my-nginx-pod";
    let response = client.pods.get_pod(pod_id).await?;
    
    if let Some(pod) = response.result {
        println!("Pod: {}", pod.pod_id);
        println!("Status: {:?}", pod.status);
        println!("URL: {:?}", pod.url);
    }

    Ok(())
}
```

## Module Organization

When using the umbrella `tapis-sdk` crate, services are organized under namespaced modules:

```rust
use tapis_sdk::authenticator::TapisAuthenticator;
use tapis_sdk::pods::TapisPods;

// Access models
use tapis_sdk::authenticator::models::NewToken;
use tapis_sdk::pods::models::NewPod;
```

## Features

### TLS Configuration

Both service crates support native TLS and rustls:

```toml
[dependencies]
tapis-sdk = { version = "0.1", default-features = false, features = ["rustls-tls"] }
```

Available features:
- `native-tls` (default): Uses system native TLS
- `rustls-tls`: Uses pure Rust TLS implementation

## Examples

Check out the `examples/` directory in each crate for more comprehensive examples:

- [Authenticator Examples](tapis-authenticator/examples/)
- [Pods Examples](tapis-pods/examples/)

Run examples with:

```bash
# Authenticator example
export TAPIS_USERNAME=your_username
export TAPIS_PASSWORD=your_password
cargo run --example authenticator_example

# Pods example
export TAPIS_TOKEN=your_jwt_token
cargo run --example tapis_token_example
```

## API Documentation

Each service provides comprehensive API documentation:

- **Authenticator**: OAuth2, token management, client management, profiles
- **Pods**: Pod management, templates, volumes, snapshots, images, permissions

Full API documentation is available at [docs.rs/tapis-sdk](https://docs.rs/tapis-sdk).

## Development

### Building from Source

```bash
git clone https://github.com/tapis-project/tapis-rust-sdk.git
cd tapis-rust-sdk
cargo build --release
```

### Running Tests

```bash
cargo test --all
```

### Publishing

The crates are published separately to crates.io:

1. Publish service crates first:
   ```bash
   cd tapis-authenticator && cargo publish
   cd ../tapis-pods && cargo publish
   ```

2. Then publish the parent crate:
   ```bash
   cargo publish
   ```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the BSD-3-Clause License - see the [LICENSE](LICENSE) file for details.

## Resources

- [Tapis Project Website](https://tapis-project.org)
- [Tapis Documentation](https://tapis.readthedocs.io/)
- [API Reference](https://docs.rs/tapis-sdk)
- [GitHub Repository](https://github.com/tapis-project/tapis-rust-sdk)

## Support

For support, please contact: cicsupport@tacc.utexas.edu

## Acknowledgments

This SDK is developed and maintained by the [Texas Advanced Computing Center (TACC)](https://www.tacc.utexas.edu/).
