---
name: sdk-gen
description: Generate base Rust SDK from OpenAPI specifications (YAML or JSON) for TAPIS services. Handles code generation, validation, and initial setup. Use when starting a new TAPIS Rust SDK from an OpenAPI spec file or when regenerating SDK after API changes.
license: Apache-2.0
---

# SDK Generation from OpenAPI

## Overview

Generate the base Rust SDK code from OpenAPI specifications. This produces the foundational API modules, models, and configuration that will be wrapped with a high-level client interface.

**What This Skill Does:**
- Generates Rust code from OpenAPI specs (YAML or JSON)
- Creates API modules for each resource endpoint
- Generates type-safe models for requests/responses
- Sets up proper dependencies and configuration

**What This Skill Does NOT Do:**
- Create high-level wrapper (use `sdk-wrapper` skill)
- Fix compilation issues (use `sdk-debug` skill)
- Add examples or documentation

---

## Prerequisites

### Install OpenAPI Generator

Choose one method:

```bash
# Via npm (recommended)
npm install -g @openapitools/openapi-generator-cli

# Via Homebrew (macOS)
brew install openapi-generator

# Verify installation
openapi-generator version
```

---

## Step-by-Step Process

### 1. Locate OpenAPI Specification

**Find the spec file:**
- Check TAPIS documentation at https://tapis.readthedocs.io/
- Look for `openapi.yaml` or `openapi.json` files
- Download from API endpoint if available
- Verify format (YAML or JSON)

**Validate the spec (optional but recommended):**
```bash
openapi-generator validate -i path/to/spec.yaml
```

### 2. Run Generation Script

**Navigate to tools directory:**
```bash
cd sdk-tools
```

**Generate SDK:**
```bash
./generate_rust_sdk.sh <spec_file> <output_dir> [package_name]
```

**Examples:**
```bash
# From YAML spec
./generate_rust_sdk.sh ../specs/pods-openapi.yaml ../tapis-pods

# From JSON spec  
./generate_rust_sdk.sh ../specs/files.json ../tapis-files tapis_files

# With custom package name
./generate_rust_sdk.sh spec.yaml ../tapis-systems tapis_systems
```

### 3. Verify Generation Success

**Navigate to generated directory:**
```bash
cd <output_dir>
```

**Check structure:**
```bash
ls -la
# Should see:
# - Cargo.toml
# - src/apis/
# - src/models/
# - docs/
```

**Test compilation:**
```bash
cargo build
```

**Expected:** Compilation succeeds with possible warnings (warnings are OK at this stage).

---

## Generated Structure

```
tapis-<service>/
├── Cargo.toml           # Package manifest
├── README.md            # Generated README
├── src/
│   ├── lib.rs          # Library root
│   ├── apis/
│   │   ├── mod.rs
│   │   ├── configuration.rs    # Configuration struct
│   │   ├── pods_api.rs        # API for pods resource
│   │   ├── volumes_api.rs     # API for volumes resource
│   │   └── ...                # Other resource APIs
│   └── models/
│       ├── mod.rs
│       ├── new_pod.rs         # Request models
│       ├── pod_response.rs    # Response models
│       └── ...                # Other models
└── docs/
    ├── PodsApi.md            # API documentation
    ├── NewPod.md             # Model documentation
    └── ...
```

---

## What Gets Generated

### API Modules (`src/apis/*_api.rs`)

Each resource gets its own API module with async functions:

```rust
// Example: pods_api.rs
pub async fn list_pods(
    configuration: &configuration::Configuration,
) -> Result<models::PodsResponse, Error<ListPodsError>> {
    // Implementation...
}

pub async fn create_pod(
    configuration: &configuration::Configuration,
    new_pod: models::NewPod,
) -> Result<models::PodResponse, Error<CreatePodError>> {
    // Implementation...
}
```

### Models (`src/models/*.rs`)

Type-safe structs for all API data:

```rust
// Example: new_pod.rs
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NewPod {
    pub pod_id: String,
    pub image: Option<String>,
    pub description: Option<String>,
    // ... more fields
}
```

### Configuration (`src/apis/configuration.rs`)

Base configuration struct:

```rust
pub struct Configuration {
    pub base_path: String,
    pub user_agent: Option<String>,
    pub client: reqwest::Client,
    pub basic_auth: Option<BasicAuth>,
    pub oauth_access_token: Option<String>,
    pub bearer_access_token: Option<String>,
    pub api_key: Option<ApiKey>,
}
```

---

## Common Issues & Solutions

### Issue: OpenAPI Generator Not Found

**Error:** `openapi-generator: command not found`

**Solution:**
```bash
# Install via npm
npm install -g @openapitools/openapi-generator-cli

# Or via Homebrew
brew install openapi-generator
```

### Issue: Generation Fails with Validation Error

**Error:** OpenAPI spec validation fails

**Solution:**
```bash
# Skip validation (use with caution)
openapi-generator generate \
    -i spec.yaml \
    -g rust \
    -o output_dir \
    --skip-validate-spec
```

### Issue: Compilation Errors After Generation

**Error:** `cargo build` fails with type errors

**Solutions:**
1. Update Rust toolchain: `rustup update stable`
2. Check for dependency conflicts: `cargo tree`
3. Clean and rebuild: `cargo clean && cargo build`
4. Use `sdk-debug` skill to fix specific issues

### Issue: Missing Endpoints

**Problem:** Some API endpoints not generated

**Solutions:**
1. Verify OpenAPI spec includes all endpoints
2. Check spec version compatibility
3. Try regenerating with updated spec
4. Manually add missing endpoints if necessary

---

## Real-World Issues & Solutions

These issues were discovered during actual SDK generation for TAPIS services:

### Issue: Empty Enum Causes Compilation Failure

**Error:**
```
error: expected identifier, found `}`
  --> src/models/delete_client_200_response.rs:65:5
   |
65 |     }
   |     ^ expected identifier
```

**Root Cause:** OpenAPI generator creates an empty enum when the spec has a null/empty result type:
```rust
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Result {
    // Empty - causes compilation error!
}

impl Default for Result {
    fn default() -> Result {
        Self::  // No variant to select!
    }
}
```

**Solution:** Replace the empty enum with `serde_json::Value`:
```rust
// Before - in the struct field:
pub result: Option<Option<Result>>,

// After:
pub result: Option<Option<serde_json::Value>>,

// Then remove the empty enum entirely
```

**Prevention:** Check for empty enums after generation:
```bash
grep -A 2 "pub enum.*{$" src/models/*.rs | grep -A 1 "^}$"
```

### Issue: Missing tokio Dependency for Examples

**Error:**
```
error[E0433]: failed to resolve: use of unlinked crate `tokio`
  --> examples/example.rs:3:3
   |
3 | #[tokio::main]
  |   ^^^^^ use of unlinked crate `tokio`
```

**Root Cause:** Generated SDK doesn't include `tokio` in dependencies, but examples need it for `#[tokio::main]`

**Solution:** Add tokio to `dev-dependencies` in `Cargo.toml`:
```toml
[dev-dependencies]
tokio = { version = "1", features = ["full"] }
```

**Why:** This keeps tokio out of the main library (users choose their async runtime) but makes it available for examples and tests.

### Issue: Incorrect OpenAPI Spec Path in Script

**Error:**
```
❌ Error: OpenAPI_specs.json not found at: /path/to/repo/skills/sdk-gen/references/OpenAPI_specs.json
```

**Root Cause:** Generation script looking in wrong directory (missing `.github/` prefix)

**Solution:** Update the path in the generation script:
```bash
# Before:
OPENAPI_SPECS_FILE="$REPO_ROOT/skills/sdk-gen/references/OpenAPI_specs.json"

# After:
OPENAPI_SPECS_FILE="$REPO_ROOT/.github/skills/sdk-gen/references/OpenAPI_specs.json"
```

---

## SDK Testing & Verification Scripts

### Verify Compilation

```bash
cd tapis-<service>
cargo build --all-targets
```

**Expected:** Build succeeds (warnings are acceptable)

### Check for Empty Enums (Potential Issues)

```bash
# Find empty enums that will cause compilation errors
find src/models -name "*.rs" -exec grep -l "pub enum.*{\s*}" {} \;
```

### Verify Model Coverage

```bash
# Count generated models
ls -1 src/models/*.rs | wc -l

# List all models
ls -1 src/models/*.rs | xargs -n1 basename | sed 's/.rs$//'
```

### Verify API Coverage

```bash
# Count API modules
ls -1 src/apis/*_api.rs | wc -l

# List all API modules
ls -1 src/apis/*_api.rs | xargs -n1 basename | sed 's/_api.rs$//'

# Count total API methods
grep -c "^pub async fn" src/apis/*_api.rs
```

### Issue: Missing Endpoints

**Problem:** Some API endpoints not generated

**Solutions:**
1. Verify OpenAPI spec includes all endpoints
2. Check spec version compatibility
3. Try regenerating with updated spec
4. Manually add missing endpoints if necessary

---

## Verification Checklist

After generation, verify:

- [ ] `cargo build` succeeds (warnings OK)
- [ ] `src/apis/` contains expected API modules
- [ ] `src/models/` contains request/response models
- [ ] `Cargo.toml` has correct package name
- [ ] `docs/` folder contains generated documentation
- [ ] No critical compilation errors

---

## Next Steps

After successful generation:

1. **Add Wrapper** - Use `sdk-wrapper` skill to create high-level client
2. **Fix Issues** - Use `sdk-debug` skill if compilation problems
3. **Add Examples** - Create usage examples
4. **Documentation** - Write README and guides

---

## Generated File Reference

**Do NOT Modify These Files:**
- `src/apis/*_api.rs` - Generated API implementations
- `src/models/*.rs` - Generated model definitions
- `src/apis/mod.rs` - Generated module exports

**These are regenerated and changes will be lost.**

**Safe to Modify:**
- `Cargo.toml` - Add dependencies as needed
- `README.md` - Replace with custom documentation
- `examples/` - Add after creation
- `tests/` - Add after creation

**Will Be Added Later:**
- `src/client.rs` - High-level wrapper (use `sdk-wrapper` skill)
- `examples/*.rs` - Usage examples
- Custom documentation in `docs/`

---

## Quick Reference

```bash
# Generate SDK
cd sdk-tools
./generate_rust_sdk.sh <spec> <output> [name]

# Verify
cd <output>
cargo build

# Next: Add wrapper
# Use sdk-wrapper skill
```

---

**Related Skills:**
- `sdk-wrapper` - Create high-level client wrapper
- `sdk-debug` - Fix compilation issues and warnings

**API Exploration:**
- Review TAPIS service documentation at `https://tapis.readthedocs.io/`
- Identify available endpoints and their purposes
- Understand authentication requirements (JWT via X-Tapis-Token header)
- Note any special features (file uploads, streaming, pagination)

**OpenAPI Specification:**
- Locate or download the service's OpenAPI spec (YAML or JSON)
- Validate the spec format and completeness
- Check for API versioning and base URLs

### 1.2 Set Up OpenAPI Generator

**Prerequisites:**
```bash
# Install openapi-generator (choose one)
npm install -g @openapitools/openapi-generator-cli
# OR
brew install openapi-generator
```

**Verify Installation:**
```bash
openapi-generator version
```

### 1.3 Generate Base SDK

**Using the Generation Script:**
```bash
cd sdk-tools
./generate_rust_sdk.sh <spec_file> <output_dir> [package_name]
```

**Example:**
```bash
./generate_rust_sdk.sh ../specs/pods-openapi.yaml ../tapis-pods tapis_pods
```

**What Gets Generated:**
- `src/apis/` - API endpoint modules (pods_api.rs, volumes_api.rs, etc.)
- `src/models/` - Request/response models
- `src/apis/configuration.rs` - Base configuration struct
- `Cargo.toml` - Package manifest with dependencies
- `docs/` - Generated API documentation

### 1.4 Validate Generation

```bash
cd <output_dir>
cargo build  # Should compile without errors
cargo check  # Verify no warnings
```

**Common Issues:**
- Missing dependencies: Update Cargo.toml if needed
- Type conflicts: Check OpenAPI spec for issues
- Compilation errors: May need to skip validation with `--skip-validate-spec`

---

## Phase 2: High-Level Wrapper Implementation

### 2.1 Understand Wrapper Architecture

**Goal:** Transform low-level generated APIs into ergonomic client interface:

```rust
// Before (generated API - verbose)
let config = Configuration::default();
config.base_path = "https://api.tapis.io/v3".to_string();
let pods = apis::pods_api::list_pods(&config).await?;

// After (wrapper - clean)
let client = TapisPods::new("https://api.tapis.io/v3", &jwt)?;
let pods = client.pods.list_pods().await?;
```

**Key Principles:**
- **Sub-client Organization**: Group APIs by resource (pods, volumes, templates)
- **Global Authentication**: JWT set once, applied automatically to all requests
- **Type Preservation**: Keep exact parameter and return types from generated code
- **100% Coverage**: Wrap every single API method

### 2.2 Add Required Dependencies

```toml
# Add to Cargo.toml
[dependencies]
http = "^1.0"  # For HeaderMap and HeaderValue
```

### 2.3 Create Wrapper Structure

**Create `src/client.rs`:**

```rust
use crate::apis::configuration;
use std::sync::Arc;
use http::{HeaderMap, HeaderValue};

/// High-level client for Tapis Pods API
/// Note: Each SDK has its own wrapper struct:
/// - tapis-pods → TapisPods
/// - tapis-files → TapisFiles
/// - tapis-jobs → TapisJobs
/// - etc.
pub struct TapisPods {
    config: Arc<configuration::Configuration>,
    pub pods: PodsClient,
    pub volumes: VolumesClient,
    // Add all resource clients...
}

impl TapisPods {
    /// Create a new TapisPods client with JWT authentication
    ///
    /// # Arguments
    /// * `base_url` - Base URL (e.g., "https://api.tapis.io/v3")
    /// * `jwt_token` - JWT authentication token
    pub fn new(base_url: &str, jwt_token: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // Create default headers with JWT
        let mut headers = HeaderMap::new();
        headers.insert("X-Tapis-Token", HeaderValue::from_str(jwt_token)?);

        // Build reqwest client with default headers
        let client = reqwest::Client::builder()
            .default_headers(headers)  // KEY: Global header injection
            .build()?;

        // Create configuration
        let mut config = configuration::Configuration::default();
        config.base_path = base_url.to_string();
        config.client = client;

        let config = Arc::new(config);

        Ok(TapisPods {
            config: config.clone(),
            pods: PodsClient { config: config.clone() },
            volumes: VolumesClient { config: config.clone() },
            // Initialize all sub-clients...
        })
    }
}
```

### 2.4 Implement Sub-Clients

**For Each Resource Group:**

```rust
/// Client for Pods API operations
#[derive(Clone)]
pub struct PodsClient {
    config: Arc<configuration::Configuration>,
}

impl PodsClient {
    /// List all pods
    pub async fn list_pods(&self) -> Result<
        crate::models::PodsResponse,
        crate::apis::Error<crate::apis::pods_api::ListPodsError>
    > {
        // Delegate to generated API
        crate::apis::pods_api::list_pods(&self.config).await
    }

    /// Get a specific pod
    pub async fn get_pod(&self, pod_id: &str) -> Result<
        crate::models::PodResponse,
        crate::apis::Error<crate::apis::pods_api::GetPodError>
    > {
        crate::apis::pods_api::get_pod(&self.config, pod_id).await
    }

    // Wrap ALL other methods - 100% coverage required
}
```

**Critical Requirements:**
- ✅ Wrap EVERY method from generated API
- ✅ Preserve exact parameter types and order
- ✅ Preserve exact return types including error types
- ✅ Add doc comments for user-facing APIs
- ❌ Never modify generated API files
- ❌ Never change function signatures

### 2.5 Verify Complete Coverage

```bash
# Count generated API methods
grep "^pub async fn" src/apis/*_api.rs | wc -l

# Count wrapped methods
grep "pub async fn" src/client.rs | wc -l

# Numbers should match for 100% coverage
```

### 2.6 Export Client Module

**Add to `src/lib.rs`:**
```rust
pub mod client;
```

---

## Phase 3: Testing and Validation

### 3.1 Create Basic Usage Example

**Create `examples/basic_usage.rs`:**

```rust
// Example for tapis-pods SDK:
use tapis_pods::client::TapisPods;

// For other SDKs, use their specific wrapper:
// use tapis_files::client::TapisFiles;
// use tapis_jobs::client::TapisJobs;
// use tapis_systems::client::TapisSystems;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get JWT from environment
    let jwt_token = std::env::var("TAPIS_TOKEN")
        .expect("TAPIS_TOKEN environment variable not set");

    // Initialize client (using TapisPods as example)
    let client = TapisPods::new(
        "https://api.tapis.io/v3",
        &jwt_token
    )?;

    println!("✓ Client initialized successfully!");

    // Test basic operation
    let result = client.pods.list_pods().await?;
    println!("Found {} pods", result.result.len());

    Ok(())
}
```

### 3.2 Build and Test

```bash
# Build the SDK
cargo build

# Set authentication token
export TAPIS_TOKEN="your_jwt_token_here"

# Run example
cargo run --example basic_usage
```

**Expected Output:**
```
✓ Client initialized successfully!
Found N pods
```

### 3.3 Create Integration Tests

**Create `tests/integration_test.rs`:**

```rust
#[tokio::test]
async fn test_authentication() {
    let jwt = std::env::var("TAPIS_TOKEN").expect("TAPIS_TOKEN required");
    // Each SDK uses its own wrapper struct:
    let client = tapis_pods::client::TapisPods::new(
        "https://dev.develop.tapis.io/v3",
        &jwt
    );
    assert!(client.is_ok());
}

#[tokio::test]
async fn test_list_pods() {
    let jwt = std::env::var("TAPIS_TOKEN").expect("TAPIS_TOKEN required");
    let client = tapis_pods::client::TapisPods::new(
        "https://dev.develop.tapis.io/v3",
        &jwt
    ).unwrap();
    
    let result = client.pods.list_pods().await;
    assert!(result.is_ok());
}
```

### 3.4 Quality Checklist

- [ ] `cargo build` succeeds without errors
- [ ] `cargo clippy` shows no warnings
- [ ] `cargo fmt` applied consistently
- [ ] All examples run successfully
- [ ] Authentication works with valid JWT
- [ ] 100% API coverage verified
- [ ] Documentation generated with `cargo doc`
- [ ] No modifications to generated `apis/` files

---

## Phase 4: Documentation and Examples

### 4.1 Create README.md

**Essential Sections:**
- Overview of the TAPIS service
- Installation instructions
- Quick start example
- Authentication setup
- Available API modules
- Link to full documentation

**Example:**
```markdown
# Tapis Pods Rust SDK

Official Rust SDK for the Tapis Pods Service.

## Installation

```toml
[dependencies]
tapis_pods = "1.0.0"
tokio = { version = "1", features = ["full"] }
```

## Quick Start

```rust
use tapis_pods::client::TapisClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = TapisClient::new(
        "https://api.tapis.io/v3",
        &std::env::var("TAPIS_TOKEN")?
    )?;
    
    let pods = client.pods.list_pods().await?;
    println!("Pods: {:?}", pods);
    
    Ok(())
}
```

## Authentication

Set your JWT token:
```bash
export TAPIS_TOKEN="your_jwt_token"
```

## API Modules

- `client.pods` - Pod management
- `client.volumes` - Volume management
- `client.templates` - Template management
- ...
```

### 4.2 Create Advanced Examples

**Create comprehensive workflow examples:**

**`examples/complete_workflow.rs`:**
```rust
// Demonstrates:
// 1. Creating a volume
// 2. Uploading files
// 3. Creating a pod with the volume
// 4. Configuring networking
// 5. Monitoring pod status
```

See `docs/COMPLETE_WORKFLOW_EXAMPLE.md` for full example.

### 4.3 Generate Documentation

```bash
# Generate Rust docs
cargo doc --no-deps --open

# Creates documentation in target/doc/
```

### 4.4 Create Usage Guides

**Key Documents:**
- `CLIENT_WRAPPER.md` - Wrapper architecture and design
- `COMPLETE_WORKFLOW_EXAMPLE.md` - End-to-end examples
- `TROUBLESHOOTING.md` - Common issues and solutions

---

## 📋 Best Practices

### Wrapper Standards

#### ✅ Required

- **100% API Coverage**: Every generated method wrapped
- **Type Preservation**: Exact types from generated code
- **Error Transparency**: Preserve original error types
- **Consistent Naming**: Match generated function names
- **Sub-client Organization**: Group by resource type
- **Global Authentication**: Use `reqwest` default_headers()
- **Documentation**: Doc comments on all public methods

#### ❌ Prohibited

- **Modifying Generated Files**: Never edit `apis/*_api.rs`
- **Type Transformations**: Don't change signatures
- **Business Logic**: Wrapper delegates only
- **Ignoring Errors**: Always preserve full error info

### Authentication Architecture

**Why Global Headers?**

```rust
// ✅ GOOD: Set once in wrapper
let mut headers = HeaderMap::new();
headers.insert("X-Tapis-Token", HeaderValue::from_str(jwt)?);
let client = reqwest::Client::builder()
    .default_headers(headers)
    .build()?;
```

**Benefits:**
- JWT set once during initialization
- Automatically applied to all requests
- No modification of generated files
- SDK regeneration doesn't break auth
- Zero performance overhead
- Clean separation of concerns

**Deprecated Approach (Don't Use):**
```rust
// ❌ BAD: Manual injection in each API method
if let Some(ref tapis_token) = configuration.tapis_token {
    req_builder = req_builder.header("X-Tapis-Token", tapis_token);
}
```

Problems:
- Requires modifying generated files
- Breaks on SDK regeneration
- Code duplication in every method
- Difficult to maintain

### Code Organization

```
tapis-<service>/
├── Cargo.toml
├── README.md
├── src/
│   ├── lib.rs              # Exports all modules
│   ├── client.rs           # High-level wrapper (ADD THIS)
│   ├── apis/               # Generated (DO NOT MODIFY)
│   │   ├── mod.rs
│   │   ├── configuration.rs
│   │   └── *_api.rs
│   └── models/             # Generated (DO NOT MODIFY)
│       └── *.rs
├── examples/               # Usage examples (ADD THESE)
│   ├── basic_usage.rs
│   └── complete_workflow.rs
├── tests/                  # Integration tests
│   └── integration_test.rs
└── docs/                   # Documentation
    ├── COMPLETE_WORKFLOW_EXAMPLE.md
    └── *.md (generated)
```

### Git Branch Management

**IMPORTANT:** SDK generation should follow the environment-to-branch mapping:

| Environment | Git Branch | Usage |
|-------------|------------|-------|
| `prod` | `main` | Production-ready SDKs from production API specs |
| `staging` | `staging` | Testing SDKs from staging API specs |
| `dev` | `dev` | Development SDKs from dev API specs |

**Workflow:**

```bash
# Generate SDK for production environment
git checkout main
./generate_rust_sdk.sh --from-json prod pods

# Generate SDK for staging environment
git checkout staging
./generate_rust_sdk.sh --from-json staging pods

# Generate SDK for development environment
git checkout dev
./generate_rust_sdk.sh --from-json dev pods
```

**Branch Rules:**
- ✅ Always generate SDKs on the correct branch for the target environment
- ✅ Use `main` branch for production-ready SDKs only
- ✅ Test changes on `dev` or `staging` before merging to `main`
- ❌ Never generate dev/staging SDKs on the `main` branch
- ❌ Never mix environment specs across branches

**Example Workflow:**
```bash
# Working on a new feature with dev environment
git checkout dev
git pull origin dev

# Generate SDK from dev environment
cd skills/sdk-gen/scripts
./generate_rust_sdk.sh --from-json dev pods

# Test and validate
cd ../../../tapis-pods
cargo build
cargo test

# Commit changes
git add .
git commit -m "feat: update pods SDK from dev environment"
git push origin dev

# After validation, merge through staging to main
git checkout staging
git merge dev
# Test on staging...

git checkout main
git merge staging
# Final production deployment
```

---

## 🔧 Troubleshooting

### Issue: SDK Generation Fails

**Symptoms:** openapi-generator exits with error

**Solutions:**
```bash
# Validate OpenAPI spec
openapi-generator validate -i spec.yaml

# Skip validation if needed
./generate_rust_sdk.sh spec.yaml output --skip-validate-spec

# Update openapi-generator
npm update -g @openapitools/openapi-generator-cli
```

### Issue: Authentication Not Working (401 Errors)

**Symptoms:** API calls return 401 Unauthorized

**Debug Steps:**
```rust
// 1. Verify JWT format
let jwt = std::env::var("TAPIS_TOKEN")?;
assert!(jwt.starts_with("eyJ"), "Invalid JWT format");

// 2. Check JWT expiration
// JWTs are base64-encoded JSON - decode and check exp field

// 3. Verify base URL includes /v3
let client = TapisClient::new(
    "https://api.tapis.io/v3",  // Must include version
    &jwt
)?;

// 4. Test with curl
// curl -H "X-Tapis-Token: $TAPIS_TOKEN" https://api.tapis.io/v3/pods
```

### Issue: Missing Methods in Wrapper

**Symptoms:** Compilation error - method not found

**Solutions:**
```bash
# 1. Find the method in generated API
grep "pub async fn method_name" src/apis/*_api.rs

# 2. Get the exact signature
grep -A 10 "pub async fn method_name" src/apis/resource_api.rs

# 3. Add to wrapper with exact types
# See Phase 2.4 for template
```

### Issue: Type Mismatch Errors

**Symptoms:** Expected `Type1` but found `Type2`

**Solutions:**
```rust
// Always use exact types from generated code
use crate::models::PodsResponse;  // Not Response or MyResponse

// Check generated function signature
grep -A 5 "pub async fn method" src/apis/resource_api.rs

// Match return type exactly
pub async fn list_pods(&self) -> Result<
    crate::models::PodsResponse,  // Exact model type
    crate::apis::Error<crate::apis::pods_api::ListPodsError>  // Exact error type
> {
    crate::apis::pods_api::list_pods(&self.config).await
}
```

---

## 📚 Reference Files

### Core Documentation (Load First)

- **[WRAPPER_GUIDE.md](../WRAPPER_GUIDE.md)** - Comprehensive wrapper creation guide with:
  - Complete architecture overview
  - Step-by-step implementation
  - Authentication patterns
  - Testing strategies
  - Troubleshooting

- **[README.md](../README.md)** - SDK tools overview and usage

### Example Implementation (Reference During Phase 2/3)

- **`../tapis-pods/`** - Complete reference implementation with:
  - Full wrapper implementation
  - All sub-clients (pods, volumes, templates, etc.)
  - Working examples
  - Integration tests
  - Complete documentation

### Key Example Files

- **`../tapis-pods/src/client.rs`** - Complete wrapper implementation
- **`../tapis-pods/examples/tapis_token_example.rs`** - Basic usage
- **`../tapis-pods/docs/COMPLETE_WORKFLOW_EXAMPLE.md`** - Advanced workflows

---

## ✅ Completion Checklist

Use this checklist when creating a new TAPIS Rust SDK:

### Phase 1: Generation
- [ ] OpenAPI spec obtained and validated
- [ ] SDK generated with `generate_rust_sdk.sh`
- [ ] `cargo build` succeeds without errors
- [ ] Generated code reviewed for completeness

### Phase 2: Wrapper
- [ ] `http` dependency added to `Cargo.toml`
- [ ] `src/client.rs` created
- [ ] Main client struct implemented (e.g., `TapisClient`)
- [ ] All resource sub-clients implemented (e.g., `PodsClient`)
- [ ] ALL API methods wrapped (100% coverage verified)
- [ ] Global authentication via `default_headers()` working
- [ ] Client module exported in `src/lib.rs`
- [ ] No modifications to generated `apis/` files

### Phase 3: Testing
- [ ] At least one example program created
- [ ] Example runs successfully with valid JWT
- [ ] Integration tests created
- [ ] All tests pass
- [ ] Code formatted with `cargo fmt`
- [ ] No clippy warnings

### Phase 4: Documentation
- [ ] README.md with installation and quick start
- [ ] Advanced usage examples
- [ ] API documentation generated (`cargo doc`)
- [ ] Troubleshooting guide
- [ ] Complete workflow example

---

## 🚀 Quick Reference

### Generate SDK
```bash
cd sdk-tools
./generate_rust_sdk.sh <spec_file> <output_dir> [package_name]
```

### Add Wrapper
```bash
cd <output_dir>
echo 'http = "^1.0"' >> Cargo.toml
# Create src/client.rs following Phase 2
echo "pub mod client;" >> src/lib.rs
```

### Test
```bash
export TAPIS_TOKEN="your_jwt"
cargo run --example basic_usage
cargo test
```

### Verify Coverage
```bash
# Count API methods
grep "^pub async fn" src/apis/*_api.rs | wc -l
# Count wrapped methods  
grep "pub async fn" src/client.rs | wc -l
# Should match!
```

---

## 💡 Key Insights

### What Makes a Good TAPIS Rust SDK?

1. **Complete Coverage**: Every API endpoint accessible through wrapper
2. **Type Safety**: Rust's type system catches errors at compile time
3. **Ergonomic API**: Natural, intuitive method calls
4. **Transparent Auth**: JWT handled automatically, invisibly
5. **Clear Errors**: Preserved error types help debugging
6. **Good Docs**: Examples and guides for common workflows

### Why This Approach Works

- **Global Headers**: Elegant, maintainable authentication
- **No Generated Code Edits**: SDK regeneration always works
- **Type Preservation**: Leverage Rust's safety guarantees
- **Sub-client Pattern**: Logical organization, easy discovery
- **100% Coverage**: Users can access entire API surface

### Common Pitfalls to Avoid

- ❌ Modifying generated files (breaks on regeneration)
- ❌ Incomplete API coverage (users can't access all features)
- ❌ Changing types (breaks type safety)
- ❌ Adding business logic to wrapper (keep it simple)
- ❌ Skipping documentation (users need examples)

---

**Last Updated:** January 24, 2026  
**Maintained By:** TAPIS SDK Team
