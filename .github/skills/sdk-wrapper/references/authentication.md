# Authentication Deep Dive

Comprehensive guide to implementing JWT authentication in TAPIS Rust SDK wrappers.

## TAPIS Authentication Model

TAPIS uses JWT (JSON Web Tokens) for authentication, passed via the `X-Tapis-Token` HTTP header.

### Token Lifecycle

1. **Obtain Token:** User authenticates via TAPIS Authenticator service
2. **Store Token:** Token stored securely by client application
3. **Use Token:** Token included in every API request header
4. **Refresh Token:** Token refreshed before expiration

### Token Format

```
X-Tapis-Token: eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
```

## Implementation Approaches

### ❌ Approach 1: Manual Header Injection (Deprecated)

**Old pattern:** Modify generated API files

```rust
// In every API function (DON'T DO THIS)
if let Some(ref tapis_token) = configuration.tapis_token {
    req_builder = req_builder.header("X-Tapis-Token", tapis_token);
}
```

**Problems:**
- Modifies generated files (breaks on regeneration)
- Code duplication (repeated in every method)
- Maintenance burden
- Easy to forget
- Not thread-safe

### ✅ Approach 2: Global Default Headers (Current)

**New pattern:** Set headers once on HTTP client

```rust
use http::{HeaderMap, HeaderValue};

let mut headers = HeaderMap::new();
headers.insert(
    "X-Tapis-Token",
    HeaderValue::from_str(jwt_token)?
);

let http_client = reqwest::Client::builder()
    .default_headers(headers)
    .build()?;
```

**Benefits:**
- Set once, works everywhere
- No generated file modifications
- Thread-safe (Arc-wrapped)
- Clean separation of concerns
- Zero runtime overhead

## Implementation Details

### Step 1: Import Required Types

```rust
use http::{HeaderMap, HeaderValue};
use std::sync::Arc;
use crate::apis::configuration;
```

### Step 2: Create Header Map

```rust
pub fn new(base_url: &str, jwt_token: &str) -> Result<Self, Box<dyn std::error::Error>> {
    // Create headers
    let mut headers = HeaderMap::new();
    
    // Add JWT token
    headers.insert(
        "X-Tapis-Token",
        HeaderValue::from_str(jwt_token)?
    );
    
    // Add other headers if needed
    headers.insert(
        "Content-Type",
        HeaderValue::from_static("application/json")
    );
    
    // Build HTTP client
    let http_client = reqwest::Client::builder()
        .default_headers(headers)  // KEY: Apply headers globally
        .build()?;
    
    // ... rest of initialization
}
```

### Step 3: Configure API Client

```rust
// Create configuration with authenticated client
let mut config = configuration::Configuration::default();
config.base_path = base_url.to_string();
config.client = http_client;  // Use pre-configured client

let config = Arc::new(config);  // Share config across sub-clients
```

### Step 4: Share Config with Sub-Clients

```rust
Ok(TapisClient {
    config: config.clone(),
    pods: PodsClient { config: config.clone() },
    volumes: VolumesClient { config: config.clone() },
    // All sub-clients share the same authenticated config
})
```

## Error Handling

### Invalid JWT Format

```rust
match HeaderValue::from_str(jwt_token) {
    Ok(header_value) => header_value,
    Err(e) => {
        return Err(format!("Invalid JWT token format: {}", e).into());
    }
}
```

### Empty Token

```rust
pub fn new(base_url: &str, jwt_token: &str) -> Result<Self, Box<dyn std::error::Error>> {
    if jwt_token.is_empty() {
        return Err("JWT token cannot be empty".into());
    }
    // ... rest of implementation
}
```

### Token with Special Characters

JWT tokens are base64-encoded and URL-safe, but validate anyway:

```rust
let jwt_token = jwt_token.trim();  // Remove whitespace

if jwt_token.contains('\n') || jwt_token.contains('\r') {
    return Err("JWT token contains invalid characters".into());
}
```

## Advanced Patterns

### Multiple Authentication Methods

Support both JWT and service tokens:

```rust
pub enum TapisAuth {
    JWT(String),
    ServiceToken(String),
    Basic { username: String, password: String },
}

impl TapisClient {
    pub fn new(base_url: &str, auth: TapisAuth) -> Result<Self, Box<dyn std::error::Error>> {
        let mut headers = HeaderMap::new();
        
        match auth {
            TapisAuth::JWT(token) => {
                headers.insert("X-Tapis-Token", HeaderValue::from_str(&token)?);
            }
            TapisAuth::ServiceToken(token) => {
                headers.insert("X-Tapis-Service-Token", HeaderValue::from_str(&token)?);
            }
            TapisAuth::Basic { username, password } => {
                let credentials = format!("{}:{}", username, password);
                let encoded = base64::encode(credentials);
                headers.insert(
                    "Authorization",
                    HeaderValue::from_str(&format!("Basic {}", encoded))?
                );
            }
        }
        
        // ... rest of implementation
    }
}
```

### Token Refresh

For long-running applications:

```rust
use tokio::sync::RwLock;

pub struct TapisClient {
    config: Arc<RwLock<configuration::Configuration>>,
    jwt_token: Arc<RwLock<String>>,
}

impl TapisClient {
    pub async fn refresh_token(&self, new_token: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut headers = HeaderMap::new();
        headers.insert("X-Tapis-Token", HeaderValue::from_str(new_token)?);
        
        let http_client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;
        
        let mut config = self.config.write().await;
        config.client = http_client;
        
        let mut token = self.jwt_token.write().await;
        *token = new_token.to_string();
        
        Ok(())
    }
}
```

### Environment Variable Integration

```rust
impl TapisClient {
    pub fn from_env(base_url: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let jwt_token = std::env::var("TAPIS_TOKEN")
            .map_err(|_| "TAPIS_TOKEN environment variable not set")?;
        
        Self::new(base_url, &jwt_token)
    }
}
```

## Security Best Practices

### 1. Never Log Tokens

```rust
// ❌ BAD
println!("Token: {}", jwt_token);
log::debug!("Using token: {}", jwt_token);

// ✅ GOOD
log::debug!("Token configured (length: {})", jwt_token.len());
```

### 2. Clear Tokens on Drop

```rust
impl Drop for TapisClient {
    fn drop(&mut self) {
        // Clear sensitive data
        // Note: This is best-effort; Rust doesn't guarantee zeroing
        log::debug!("Dropping TapisClient and clearing credentials");
    }
}
```

### 3. Use Secure Storage

```rust
// For desktop apps, use keyring
use keyring::Entry;

let entry = Entry::new("tapis-sdk", "jwt-token")?;
let jwt_token = entry.get_password()?;

let client = TapisClient::new(base_url, &jwt_token)?;
```

### 4. Validate Token Format

```rust
fn validate_jwt_format(token: &str) -> Result<(), String> {
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 3 {
        return Err("Invalid JWT format: must have 3 parts".to_string());
    }
    
    // Each part should be valid base64
    for part in parts {
        if part.is_empty() {
            return Err("Invalid JWT: empty part".to_string());
        }
    }
    
    Ok(())
}
```

## Testing Authentication

### Mock JWT for Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    fn mock_jwt() -> String {
        // Valid JWT structure, but fake signature
        "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.\
         eyJzdWIiOiJ0ZXN0LXVzZXIiLCJpYXQiOjE1MTYyMzkwMjJ9.\
         SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c".to_string()
    }
    
    #[test]
    fn test_client_creation() {
        let client = TapisClient::new(
            "https://api.tapis.io/v3",
            &mock_jwt()
        );
        assert!(client.is_ok());
    }
}
```

### Test Without Network

```rust
#[tokio::test]
async fn test_headers_applied() {
    let client = TapisClient::new(
        "https://api.tapis.io/v3",
        "test-token"
    ).unwrap();
    
    // Headers should be in the config
    // (Actual verification requires inspecting reqwest internals)
}
```

## Troubleshooting

### Issue: "Invalid header value"

**Cause:** JWT contains invalid characters for HTTP headers

**Solution:**
```rust
let jwt_token = jwt_token.trim().replace('\n', "").replace('\r', "");
HeaderValue::from_str(&jwt_token)?
```

### Issue: Authentication fails silently

**Cause:** Headers not being applied

**Solution:** Verify client configuration:
```rust
// Add debug logging
log::debug!("Creating HTTP client with headers: {:?}", headers.keys());

let http_client = reqwest::Client::builder()
    .default_headers(headers)
    .build()?;

log::debug!("HTTP client configured successfully");
```

### Issue: Token works in curl but not in SDK

**Cause:** Header name mismatch

**Solution:** Verify exact header name:
```bash
# Check what header the API expects
curl -v -H "X-Tapis-Token: $TOKEN" https://api.tapis.io/v3/pods
```

## Resources

- [JWT.io](https://jwt.io/) - JWT debugger
- [reqwest default_headers](https://docs.rs/reqwest/latest/reqwest/struct.ClientBuilder.html#method.default_headers)
- [http HeaderMap](https://docs.rs/http/latest/http/header/struct.HeaderMap.html)
- [TAPIS Authentication](https://tapis.readthedocs.io/en/latest/technical/authentication.html)
