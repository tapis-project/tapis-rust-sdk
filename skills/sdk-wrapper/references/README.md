# SDK Wrapper Reference Materials

Additional documentation for creating high-level client wrappers.

## Contents

- [Architecture Patterns](./architecture-patterns.md) - Design patterns for wrapper clients
- [Authentication Guide](./authentication.md) - Deep dive into JWT authentication
- [Testing Strategies](./testing.md) - How to test the wrapper effectively

## Quick Reference

### Wrapper Template

See [wrapper-template.rs](./wrapper-template.rs) for a complete, copy-paste ready template.

### Common Patterns

**Sub-client with Arc:**
```rust
#[derive(Clone)]
pub struct ResourceClient {
    config: Arc<configuration::Configuration>,
}
```

**Method wrapper:**
```rust
pub async fn method_name(&self, params) -> Result<Model, Error> {
    crate::apis::resource_api::method_name(&self.config, params).await
}
```

**Global authentication:**
```rust
let mut headers = HeaderMap::new();
headers.insert("X-Tapis-Token", HeaderValue::from_str(jwt)?);
let client = reqwest::Client::builder()
    .default_headers(headers)
    .build()?;
```

## Related Documentation

- Main skill: [../SKILL.md](../SKILL.md)
- Example implementation: `../../tapis-pods/src/client.rs`
