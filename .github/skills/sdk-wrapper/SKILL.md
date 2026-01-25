---
name: sdk-wrapper
description: Create high-level client wrapper for TAPIS Rust SDKs with ergonomic API and global JWT authentication. Ensures 100% API coverage, type safety, and clean client interface. Use after generating base SDK with sdk-gen skill or when adding wrapper to existing generated SDK.
license: Apache-2.0
---

# SDK Wrapper Creation

## Overview

Transform low-level generated API code into an ergonomic, high-level client interface with automatic JWT authentication.

**What This Skill Does:**
- Creates `TapisClient` wrapper with sub-clients for each resource
- Implements global JWT authentication via `reqwest` default headers
- Wraps all generated API methods for 100% coverage
- Preserves type safety and error handling

**Prerequisites:**
- Base SDK generated (use `sdk-gen` skill first)
- SDK compiles successfully (`cargo build` works)

---

## Wrapper Architecture

### Before (Generated API - Verbose)

```rust
use tapis_pods::apis::configuration::Configuration;
use tapis_pods::apis::pods_api;

let mut config = Configuration::default();
config.base_path = "https://api.tapis.io/v3".to_string();
// Manual auth header management needed

let pods = pods_api::list_pods(&config).await?;
```

### After (Wrapper - Clean)

```rust
use tapis_pods::client::TapisPods;

let client = TapisPods::new("https://api.tapis.io/v3", &jwt_token)?;
let pods = client.pods.list_pods().await?;
```

---

## Naming Conventions

**CRITICAL:** The wrapper struct name must follow TAPIS conventions:

| Component | Package Name | Wrapper Struct Name |
|-----------|-------------|---------------------|
| pods | `tapis_pods` | `TapisPods` |
| files | `tapis_files` | `TapisFiles` |
| systems | `tapis_systems` | `TapisSystems` |
| actors | `tapis_actors` | `TapisActors` |
| globus-proxy | `tapis_globus_proxy` | `TapisGlobusProxy` |

**Format:** `Tapis<Component>` where Component is PascalCase version of the component name.

---

## Step-by-Step Implementation

### 1. Add Required Dependency

/// 
/// IMPORTANT: Replace "TapisClient" with "Tapis<Component>" 
/// where Component is the PascalCase component name.
/// Examples: TapisPods, TapisFiles, TapisSystems, TapisActors
pub struct TapisPods {  // ← Replace with Tapis<YourComponent>
    config: Arc<configuration::Configuration>,
    pub pods: PodsClient,
    pub volumes: VolumesClient,
    pub templates: TemplatesClient,
    // Add all resource sub-clients...
}

impl TapisPods {  // ← Replace with Tapis<YourComponent>
    /// Create a new TapisPods c
```rust
use crate::apis::configuration;
use std::sync::Arc;
use http::{HeaderMap, HeaderValue};Pods;  // ← Use your component name
    /// 
    /// let client = TapisPods::new(  // ← Use your wrapper namece> API
pub struct TapisClient {
    config: Arc<configuration::Configuration>,
    pub pods: PodsClient,
    pub volumes: VolumesClient,
    pub templates: TemplatesClient,
    // Add all resource sub-clients...
}

impl TapisClient {
    /// Create a new TapisClient with JWT authentication
    ///
    /// # Arguments
    /// * `base_url` - Base URL for the API (e.g., "https://api.tapis.io/v3")
    /// * `jwt_token` - JWT authentication token
    ///
    /// # Example
    /// ```no_run
    /// use tapis_pods::client::TapisClient;
    /// 
    /// let client = TapisClient::new(
    ///     "https://api.tapis.io/v3",
    ///     "your_jwt_token"
    /// )?;
    /// ```
    pub fn new(base_url: &str, jwt_token: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // Create default headers with JWT
        let mut headers = HeaderMap::new();
        headers.insert("X-Tapis-Token", HeaderValue::from_str(jwt_token)?);

        // Build reqwest client with default headers (KEY: automatic auth)
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        // Create configuration
        let mut config = configuration::Configuration::default();
        config.base_path = base_url.to_string();
        config.cPods {  // ← Use your wrapper name
            config: config.clone(),
            pods: PodsClient { config: config.clone() },
            volumes: VolumesClient { config: config.clone() },
            templates: TemplatesClient { config: config.clone() },
            // Initialize all sub-clients...
        })
    }
}

// IMPORTANT: For other components, replace struct name:
// - TapisFiles for files component
// - TapisSystems for systems component
// - TapisActors for actors component
// - etc.           templates: TemplatesClient { config: config.clone() },
            // Initialize all sub-clients...
        })
    }
}
```

### 3. Implement Sub-Clients

**For EACH resource in `src/apis/`**, create a sub-client:

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
        crate::apis::pods_api::list_pods(&self.config).await
    }

    /// Get a specific pod
    pub async fn get_pod(&self, pod_id: &str) -> Result<
        crate::models::PodResponse,
        crate::apis::Error<crate::apis::pods_api::GetPodError>
    > {
        crate::apis::pods_api::get_pod(&self.config, pod_id).await
    }

    /// Create a new pod
    pub async fn create_pod(&self, new_pod: crate::models::NewPod) -> Result<
        crate::models::PodResponse,
        crate::apis::Error<crate::apis::pods_api::CreatePodError>
    > {
        crate::apis::pods_api::create_pod(&self.config, new_pod).await
    }

    // ⚠️ CRITICAL: Wrap ALL methods from pods_api.rs - 100% coverage required
}

// Repeat for VolumesClient, TemplatesClient, etc.
```

### 4. Export Client Module

**Edit `src/lib.rs`:**
```rust
pub mod client;  // Add this line
```

---

## Critical Requirements

### ✅ MUST DO

1. **100% API Coverage**
   - Every method in every `*_api.rs` file MUST be wrapped
   - Verify with grep counts (see Verification section)

2. **Type Preservation**
   - Keep exact parameter types from generated API
   - Keep exact return types including error types
   - No transformations or conversions

3. **Consistent Naming**
   - Sub-client names match API module names
   - Method names match generated function names
   - Use lowercase with underscores (Rust convention)

4. **Global Authentication**
   - Use `reqwest::Client::builder().default_headers()`
   - Set `X-Tapis-Token` header in constructor
   - Never modify generated `apis/*_api.rs` files

5. **Documentation**
   - Add doc comments to main client struct
   - Add doc comments to each sub-client
   - Add doc comments to public methods

### ❌ MUST NOT DO

1. **Never Modify Generated Files**
   - Do not edit `src/apis/*_api.rs`
   - Do not edit `src/models/*.rs`
   - These files are regenerated and changes will be lost

2. **Never Change Signatures**
   - Do not change parameter types
   - Do not change return types
   - Do not add/remove parameters

3. **No Business Logic**
   - Wrapper only delegates to generated code
   - No data transformations
   - No validation logic

4. **No Error Hiding**
   - Preserve original error types
   - Don't convert to generic errors
   - Let users handle errors

---

## Finding All Methods to Wrap

### Identify All API Modules

```bash
ls src/apis/*_api.rs
# Example output:
# pods_api.rs
# volumes_api.rs
# templates_api.rs
# snapshots_api.rs
# ...
```

### Count Methods in Each Module

```bash
# For each API module
grep "^pub async fn" src/apis/pods_api.rs | wc -l
grep "^pub async fn" src/apis/volumes_api.rs | wc -l
# Etc.
```

### Get Method Signatures

```bash
# List all methods with signatures
grep -A 5 "^pub async fn" src/apis/pods_api.rs
```

**Example output:**
```rust
pub async fn list_pods(
    configuration: &configuration::Configuration,
) -> Result<models::PodsResponse, Error<ListPodsError>> {

pub async fn create_pod(
    configuration: &configuration::Configuration,
    new_pod: models::NewPod,
) -> Result<models::PodResponse, Error<CreatePodError>> {
```

### Wrap Each Method

For each method found, create wrapper:

```rust
pub async fn method_name(&self, /* params */) -> Result<
    crate::models::ResponseType,
    crate::apis::Error<crate::apis::module_api::ErrorType>
> {
    crate::apis::module_api::method_name(&self.config, /* params */).await
}
```

---

## Example: Complete Pods Client

```rust
#[derive(Clone)]
pub struct PodsClient {
    config: Arc<configuration::Configuration>,
}

impl PodsClient {
    /// List all pods accessible to the authenticated user
    pub async fn list_pods(&self) -> Result<
        crate::models::PodsResponse,
        crate::apis::Error<crate::apis::pods_api::ListPodsError>
    > {
        crate::apis::pods_api::list_pods(&self.config).await
    }

    /// Get details for a specific pod
    pub async fn get_pod(&self, pod_id: &str) -> Result<
        crate::models::PodResponse,
        crate::apis::Error<crate::apis::pods_api::GetPodError>
    > {
        crate::apis::pods_api::get_pod(&self.config, pod_id).await
    }

    /// Create a new pod
    pub async fn create_pod(&self, new_pod: crate::models::NewPod) -> Result<
        crate::models::PodResponse,
        crate::apis::Error<crate::apis::pods_api::CreatePodError>
    > {
        crate::apis::pods_api::create_pod(&self.config, new_pod).await
    }

    /// Update an existing pod
    pub async fn update_pod(
        &self,
        pod_id: &str,
        update_pod: crate::models::UpdatePod,
    ) -> Result<
        crate::models::PodResponse,
        crate::apis::Error<crate::apis::pods_api::UpdatePodError>
    > {
        crate::apis::pods_api::update_pod(&self.config, pod_id, update_pod).await
    }

    /// Delete a pod
    pub async fn delete_pod(&self, pod_id: &str, force: bool) -> Result<
        crate::models::PodDeleteResponse,
        crate::apis::Error<crate::apis::pods_api::DeletePodError>
    > {
        crate::apis::pods_api::delete_pod(&self.config, pod_id, force).await
    }

    // ... continue for ALL methods in pods_api.rs
}
```

---

## Verification

### Check Compilation

```bash
cargo build
```

**Expected:** Builds successfully (warnings OK).

### Verify 100% Coverage

```bash
# Count generated API methods
grep "^pub async fn" src/apis/*_api.rs | wc -l

# Count wrapped methods in client
grep "pub async fn" src/client.rs | wc -l

# Numbers should match!
```

**Example:**
```
Generated methods: 62
Wrapped methods: 62
✓ Coverage: 100%
```

### Test Wrapper

Create `examples/test_wrapper.rs`:

```rust
use tapis_pods::client::TapisClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let jwt = std::env::var("TAPIS_TOKEN")?;
    let client = TapisClient::new("https://api.tapis.io/v3", &jwt)?;
    
    println!("Testing wrapper...");
    let pods = client.pods.list_pods().await?;
    println!("✓ Wrapper works! Found {} pods", pods.result.len());
    
    Ok(())
}
```

**Run test:**
```bash
export TAPIS_TOKEN="your_jwt"
cargo run --example test_wrapper
```

---

## Why Global Headers?

### The Problem

**Old approach (deprecated):** Manually add headers in every API call:

```rust
// ❌ BAD: Requires modifying generated files
if let Some(ref tapis_token) = configuration.tapis_token {
    req_builder = req_builder.header("X-Tapis-Token", tapis_token);
}
```

**Problems:**
- Must modify generated `*_api.rs` files
- Breaks when SDK is regenerated
- Code duplication in every method
- Difficult to maintain

### The Solution

**New approach (current):** Use `reqwest` default headers:

```rust
// ✅ GOOD: Set once in wrapper constructor
let mut headers = HeaderMap::new();
headers.insert("X-Tapis-Token", HeaderValue::from_str(jwt_token)?);

let client = reqwest::Client::builder()
    .default_headers(headers)  // Automatic injection
    .build()?;
```

**Benefits:**
- JWT set once during client creation
- Automatically applied to ALL requests
- No modification of generated files
- SDK regeneration doesn't break authentication
- Clean separation of concerns
- Zero performance overhead

---

## Common Issues

### Issue: Missing Methods

**Symptom:** User tries to call method that doesn't exist

**Solution:**
```bash
# 1. Find the method in generated API
grep "pub async fn method_name" src/apis/*_api.rs

# 2. Get the signature
grep -A 10 "pub async fn method_name" src/apis/resource_api.rs

# 3. Add wrapper following template in section 3
```

### Issue: Type Mismatch

**Symptom:** Compilation error about wrong types

**Solution:** Always use exact types from generated code:

```rust
// ✓ Correct
pub async fn list_pods(&self) -> Result<
    crate::models::PodsResponse,  // Exact model type
    crate::apis::Error<crate::apis::pods_api::ListPodsError>  // Exact error type
>

// ✗ Wrong
pub async fn list_pods(&self) -> Result<PodsResponse, Error>
```

### Issue: Incomplete Coverage

**Symptom:** Method counts don't match

**Solution:**
```bash
# Find missing methods
diff <(grep "^pub async fn" src/apis/*_api.rs | sort) \
     <(grep "pub async fn" src/client.rs | sort)
```

---

## Real-World Issues & Solutions

These issues were discovered during wrapper creation for TAPIS services:

### Issue: Services with Public and Authenticated Endpoints

**Scenario:** Some services (like Authenticator) have both public endpoints (e.g., `/oauth2/tokens` for creating tokens) and authenticated endpoints (e.g., `/oauth2/userinfo` for user data).

**Problem:** Standard wrapper requires JWT token, preventing use of public endpoints.

**Solution:** Make JWT token optional in the constructor:

```rust
impl TapisAuthenticator {
    /// Create a new client with optional authentication
    pub fn new(
        base_url: &str, 
        jwt_token: Option<&str>
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut headers = HeaderMap::new();
        
        // Only add token header if provided
        if let Some(token) = jwt_token {
            headers.insert("X-Tapis-Token", HeaderValue::from_str(token)?);
        }
        
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;
        
        // ... rest of initialization
    }
}
```

### Issue: Missing tokio Dependency for Examples

**Error:**
```
error[E0433]: failed to resolve: use of unlinked crate `tokio`
#[tokio::main]
  ^^^^^ use of unlinked crate `tokio`
```

**Solution:** Add tokio to `Cargo.toml` as a dev-dependency:
```toml
[dev-dependencies]
tokio = { version = "1", features = ["full"] }
```

### Automated Coverage Verification

Quick script to verify 100% coverage:

```bash
# Count generated API methods
GENERATED=$(grep "^pub async fn" src/apis/*_api.rs | wc -l | tr -d ' ')

# Count wrapped methods
WRAPPED=$(grep "pub async fn" src/client.rs | wc -l | tr -d ' ')

echo "Generated: $GENERATED, Wrapped: $WRAPPED"
[ "$GENERATED" -eq "$WRAPPED" ] && echo "✓ 100%" || echo "✗ Incomplete"
```

---

## Completion Checklist

Verify all items before considering wrapper complete:

- [ ] `http` dependency added to `Cargo.toml`
- [ ] `src/client.rs` created
- [ ] Main wrapper struct named `Tapis<Component>` (e.g., `TapisPods`, `TapisAuthenticator`)
- [ ] All sub-client structs created (one per API module)
- [ ] ALL methods wrapped (100% coverage verified)
- [ ] Global authentication via `default_headers()` working
- [ ] Optional authentication considered (if service has public endpoints)
- [ ] `tokio` added to dev-dependencies for examples
- [ ] Module exported in `src/lib.rs`
- [ ] `cargo build` succeeds
- [ ] Test example runs successfully
- [ ] No modifications to generated `apis/` files
- [ ] Naming conventions followed (TapisXXX pattern)

---

## Quick Reference

```bash
# 1. Add dependency
echo 'http = "^1.0"' >> Cargo.toml

# 2. Create wrapper
touch src/client.rs
# Implement following templates above

# 3. Export
echo "pub mod client;" >> src/lib.rs

# 4. Verify coverage
grep "^pub async fn" src/apis/*_api.rs | wc -l
grep "pub async fn" src/client.rs | wc -l

# 5. Test
export TAPIS_TOKEN="jwt"
cargo run --example test_wrapper
```

---

**Related Skills:**
- `sdk-gen` - Generate base SDK (run first)
- `sdk-debug` - Fix compilation issues

**Additional Resources:**
- See `references/authentication.md` for deep dive into JWT authentication
- See `../tapis-pods/src/client.rs` for complete example
