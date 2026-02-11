---
name: sdk-wrapper
description: Create or repair TAPIS high-level Rust wrappers (Tapis* clients) with full API coverage and stable auth behavior after OpenAPI regeneration.
license: Apache-2.0
---

# SDK Wrapper Creation

## Scope

Use this skill after running `sdk-gen` to ensure each service crate has:
- A high-level wrapper client (`TapisPods`, `TapisAuthenticator`, etc.)
- Sub-clients per API module
- 100% wrapper coverage for generated async API methods
- Proper export from `src/lib.rs`

## Hard Rules

- Do not edit generated API/model files unless required by a known generator bug.
- Wrapper methods must preserve exact parameter and return types from generated APIs.
- Wrapper methods should only delegate; no business logic.
- Keep authentication in wrapper constructor via reqwest default headers.

## Naming Convention

Wrapper struct must be `Tapis<Component>`:
- `tapis_pods` -> `TapisPods`
- `tapis_authenticator` -> `TapisAuthenticator`
- `tapis_globus_proxy` -> `TapisGlobusProxy`

## Required Integration Points

### 1) Service crate exports

In `src/lib.rs`:

```rust
pub mod client;
pub use client::TapisPods; // replace with service wrapper type
```

### 2) Dependency for header types

Wrapper constructors use `HeaderMap` / `HeaderValue`, so crate `Cargo.toml` needs:

```toml
http = "^1.0"
```

### 3) Dev runtime for examples/tests

```toml
[dev-dependencies]
tokio = { version = "^1.0", features = ["full"] }
```

## Constructor Pattern

Authenticated services (e.g., Pods):

```rust
pub fn new(base_url: &str, jwt_token: &str) -> Result<Self, Box<dyn std::error::Error>> {
    let mut headers = HeaderMap::new();
    headers.insert("X-Tapis-Token", HeaderValue::from_str(jwt_token)?);

    let client = reqwest::Client::builder().default_headers(headers).build()?;

    let mut config = configuration::Configuration::default();
    config.base_path = base_url.to_string();
    config.client = client;

    // init sub-clients from Arc<Configuration>
}
```

Services with public + private endpoints (e.g., Authenticator):

```rust
pub fn new(base_url: &str, jwt_token: Option<&str>) -> Result<Self, Box<dyn std::error::Error>>
```

## Coverage Workflow

1. List generated API modules:

```bash
ls src/apis/*_api.rs
```

2. Count generated methods vs wrapped methods:

```bash
rg '^pub async fn ' src/apis/*_api.rs | wc -l
rg '^[[:space:]]*pub async fn ' src/client.rs | wc -l
```

3. Add missing wrapper methods until counts match.

4. Rebuild:

```bash
cargo build --all-targets
```

## Signature Preservation Checklist

For every wrapper method, ensure:
- Same non-configuration parameters as generated method
- Same `Result<SuccessType, crate::apis::Error<module_api::ModuleErrorType>>`
- Delegation call uses `&self.config` then forwards all arguments

Important:
- Keep error types module-qualified (for example `Error<pods_api::ListPodsError>`), not bare `Error<ListPodsError>`.

## Common Regeneration Regressions

### Added endpoints in generated API

Regeneration can add methods that wrappers do not expose (for example new secrets endpoints in Pods). Always rerun method count parity checks after generation.

### Signature drift

Existing wrapper methods can become stale when OpenAPI adds optional query params or changes return type aliases. Update wrappers to match generated signatures exactly.

Also check for generated types that are invalid in wrappers (for example `models::serde_json::Value`); use `serde_json::Value` when that appears.

### Export drift

`src/lib.rs` is overwritten during generation and often loses `pub mod client;`.

## Verification

From repo root:

```bash
cargo build --workspace --all-targets
```

For each wrapped service, verify parity:

```bash
rg '^pub async fn ' tapis-pods/src/apis/*_api.rs | wc -l
rg '^[[:space:]]*pub async fn ' tapis-pods/src/client.rs | wc -l

rg '^pub async fn ' tapis-authenticator/src/apis/*_api.rs | wc -l
rg '^[[:space:]]*pub async fn ' tapis-authenticator/src/client.rs | wc -l
```

## References

- `references/authentication.md`
- `references/architecture-patterns.md`
- `references/testing.md`
- `references/wrapper-template.rs`
