# TapisAuthenticator SDK - Development Notes

## Overview

This document captures the development process, issues encountered, and solutions applied during the generation and enhancement of the TapisAuthenticator Rust SDK.

Generated: January 24, 2026

---

## Generation Process

### 1. SDK Generation

**Command:**
```bash
cd .github/skills/sdk-gen/scripts
./generate_rust_sdk.sh prod authenticator
```

**What was generated:**
- 6 API modules (admin, clients, health_check, metadata, profiles, tokens)
- 28 data models
- 17 API methods
- Complete documentation

---

## Issues Encountered & Solutions

### Issue #1: Script Path Error

**Error:**
```
❌ Error: OpenAPI_specs.json not found at: /path/to/repo/skills/sdk-gen/references/OpenAPI_specs.json
```

**Root Cause:** Generation script was looking in `skills/sdk-gen/` instead of `.github/skills/sdk-gen/`

**Solution:** Updated script path:
```bash
# Before:
OPENAPI_SPECS_FILE="$REPO_ROOT/skills/sdk-gen/references/OpenAPI_specs.json"

# After:
OPENAPI_SPECS_FILE="$REPO_ROOT/.github/skills/sdk-gen/references/OpenAPI_specs.json"
```

**File Modified:** [.github/skills/sdk-gen/scripts/generate_rust_sdk.sh](../.github/skills/sdk-gen/scripts/generate_rust_sdk.sh#L144)

---

### Issue #2: Empty Enum Compilation Error

**Error:**
```
error: expected identifier, found `}`
  --> src/models/delete_client_200_response.rs:65:5
   |
65 |     }
   |     ^ expected identifier
```

**Root Cause:** OpenAPI generator created an empty `Result` enum for a null result type:

```rust
pub enum Result {
    // Empty - invalid Rust syntax!
}

impl Default for Result {
    fn default() -> Result {
        Self::  // No variant to select
    }
}
```

**Solution:** Replaced empty enum with `serde_json::Value`:

```rust
// Changed field type from:
pub result: Option<Option<Result>>,

// To:
pub result: Option<Option<serde_json::Value>>,

// And removed the empty enum entirely
```

**File Fixed:** [src/models/delete_client_200_response.rs](src/models/delete_client_200_response.rs)

**Lesson:** After generation, always check for empty enums:
```bash
grep -A 2 "pub enum.*{$" src/models/*.rs | grep -A 1 "^}$"
```

---

### Issue #3: Missing tokio for Examples

**Error:**
```
error[E0433]: failed to resolve: use of unlinked crate `tokio`
  --> examples/authenticator_example.rs:21:3
   |
21 | #[tokio::main]
   |   ^^^^^ use of unlinked crate `tokio`
```

**Root Cause:** Generated SDK doesn't include async runtime dependencies. Examples need `#[tokio::main]` but tokio wasn't in dependencies.

**Solution:** Added tokio as dev-dependency:

```toml
[dev-dependencies]
tokio = { version = "1", features = ["full"] }
```

**Rationale:** 
- Keeps main library runtime-agnostic
- Users can choose their preferred async runtime
- Examples and tests have what they need

**File Modified:** [Cargo.toml](Cargo.toml)

---

### Issue #4: Optional Authentication Requirement

**Challenge:** Authenticator service has both:
- Public endpoints (e.g., `/oauth2/tokens` - create JWT without authentication)
- Authenticated endpoints (e.g., `/oauth2/userinfo` - requires JWT)

**Standard Pattern Problem:** Typical wrapper requires JWT token in constructor, blocking public endpoint access.

**Solution:** Made JWT token optional:

```rust
impl TapisAuthenticator {
    pub fn new(
        base_url: &str, 
        jwt_token: Option<&str>  // ← Optional!
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut headers = HeaderMap::new();
        
        // Only add auth header if token provided
        if let Some(token) = jwt_token {
            headers.insert("X-Tapis-Token", HeaderValue::from_str(token)?);
        }
        
        // ... rest
    }
}
```

**Usage:**
```rust
// For token creation (no auth needed)
let public_client = TapisAuthenticator::new(url, None)?;
let token = public_client.tokens.create_token(credentials).await?;

// For authenticated requests
let auth_client = TapisAuthenticator::new(url, Some(&jwt))?;
let userinfo = auth_client.profiles.get_userinfo().await?;
```

**File Modified:** [src/client.rs](src/client.rs)

---

### Issue #5: Complex Nested Model Structures

**Challenge:** Token response has nested structure not immediately obvious from types:

```rust
pub struct TokenResponse {
    pub access_token: Box<TokenResponseAccessToken>,
}

pub struct TokenResponseAccessToken {
    pub access_token: Option<String>,  // ← The actual JWT string is here
    pub expires_in: Option<i32>,
}
```

**Solution:** Examined model files before writing examples:

```bash
cat src/models/token_response.rs
cat src/models/token_response_access_token.rs
```

**Correct Access Pattern:**
```rust
if let Some(result) = response.result {
    // Need to go through nested structure
    if let Some(token_str) = result.access_token.access_token {
        println!("JWT: {}", token_str);
    }
}
```

**Lesson:** Always inspect generated models before writing example code.

---

## Wrapper Implementation

### Architecture

**Main Client:**
```rust
pub struct TapisAuthenticator {
    config: Arc<configuration::Configuration>,
    pub admin: AdminClient,
    pub clients: ClientsClient,
    pub health_check: HealthCheckClient,
    pub metadata: MetadataClient,
    pub profiles: ProfilesClient,
    pub tokens: TokensClient,
}
```

**Sub-Clients:** One for each API module, wrapping all methods

**Authentication:** Global JWT via `reqwest::Client` default headers

### Coverage Verification

**Command:**
```bash
echo "Generated: $(grep '^pub async fn' src/apis/*_api.rs | wc -l | tr -d ' ')"
echo "Wrapped:   $(grep 'pub async fn' src/client.rs | wc -l | tr -d ' ')"
```

**Result:**
```
Generated: 17
Wrapped:   17
✓ Coverage: 100%
```

---

## Example Implementation

### Features Demonstrated

1. **Token Creation (Unauthenticated)**
   - Shows optional JWT usage
   - Handles username/password authentication
   - Displays created JWT token

2. **Health Checks**
   - Basic connectivity test
   - No authentication required

3. **Server Metadata**
   - Version and capability discovery
   - Authenticated request example

4. **User Information**
   - Profile retrieval
   - Demonstrates authenticated API access

5. **OAuth2 Client Listing**
   - Pagination parameters
   - Complex response handling

### Environment Variables

```bash
# For token creation
export TAPIS_USERNAME="your_username"
export TAPIS_PASSWORD="your_password"

# For authenticated examples
export TAPIS_TOKEN="your_jwt_token"
```

---

## Testing & Verification

### Build Verification
```bash
cargo build --all-targets
# ✓ Compiles successfully
```

### Example Compilation
```bash
cargo build --example authenticator_example
# ✓ Compiles successfully
```

### Coverage Check
```bash
Generated API methods: 17
Wrapped methods:       17
✓ Coverage: 100%
```

---

## Lessons Learned & Best Practices

### 1. Always Verify Generated Code

```bash
# Check for empty enums
grep -A 2 "pub enum.*{$" src/models/*.rs | grep -A 1 "^}$"

# Count API methods
grep "^pub async fn" src/apis/*_api.rs | wc -l
```

### 2. Add Dev Dependencies Early

```toml
[dev-dependencies]
tokio = { version = "1", features = ["full"] }
```

### 3. Consider Optional Authentication

For services with public endpoints, make JWT optional:
```rust
pub fn new(base_url: &str, jwt_token: Option<&str>) -> Result<...>
```

### 4. Inspect Models Before Writing Examples

```bash
# Find relevant models
grep -l "ModelName" src/models/*.rs

# Read the structure
cat src/models/model_name.rs
```

### 5. Wrapper Coverage Verification Script

```bash
#!/bin/bash
GENERATED=$(grep "^pub async fn" src/apis/*_api.rs | wc -l | tr -d ' ')
WRAPPED=$(grep "pub async fn" src/client.rs | wc -l | tr -d ' ')
echo "Generated: $GENERATED, Wrapped: $WRAPPED"
[ "$GENERATED" -eq "$WRAPPED" ] && echo "✓ 100%" || echo "✗ Incomplete"
```

---

## Files Modified/Created

### Generated Files (Do Not Modify)
- `src/apis/*.rs` - API implementations
- `src/models/*.rs` - Data models (except fixed empty enum)
- `src/apis/configuration.rs` - Base configuration

### Created Files
- ✅ `src/client.rs` - High-level wrapper
- ✅ `examples/authenticator_example.rs` - Usage examples
- ✅ `DEVELOPMENT_NOTES.md` - This file

### Modified Files
- ✅ `Cargo.toml` - Added http and tokio dependencies
- ✅ `src/lib.rs` - Exported client module
- ✅ `README.md` - Updated with wrapper documentation
- ✅ `src/models/delete_client_200_response.rs` - Fixed empty enum
- ✅ `.github/skills/sdk-gen/SKILL.md` - Added troubleshooting
- ✅ `.github/skills/sdk-wrapper/SKILL.md` - Added real-world issues

---

## Next Steps for Other Services

When generating SDKs for other TAPIS services:

1. **Run generation script** from skills directory
2. **Check for empty enums** immediately after generation
3. **Add http dependency** to Cargo.toml for wrapper
4. **Add tokio dev-dependency** for examples
5. **Create wrapper** following TapisXXX naming convention
6. **Verify 100% coverage** with grep commands
7. **Create examples** after inspecting model structures
8. **Consider authentication** requirements (optional vs required)

---

## References

- **Skills Documentation:** `.github/skills/sdk-gen/SKILL.md`
- **Wrapper Guide:** `.github/skills/sdk-wrapper/SKILL.md`
- **OpenAPI Specs:** `.github/skills/sdk-gen/references/OpenAPI_specs.json`
- **Example SDK:** `tapis-pods/` (similar structure, required auth only)

---

## Contact

For issues or questions about this SDK:
- TAPIS Project: https://tapis-project.org
- Email: cicsupport@tacc.utexas.edu
