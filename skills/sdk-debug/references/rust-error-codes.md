# Common Rust Error Codes in SDK Development

Detailed explanations and fixes for Rust compiler errors commonly encountered during SDK development.

## Type and Trait Errors

### E0277: Trait Bound Not Satisfied

**Full Error:**
```
error[E0277]: the trait bound `MyType: Serialize` is not satisfied
```

**Cause:** Type needs to implement a trait (usually `Serialize`/`Deserialize` for API models)

**Fix:**
```rust
// Add derive macro
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MyType {
    pub field: String,
}
```

**Common variants:**
- `Send` not satisfied → Use `Arc` or `Rc` instead of references
- `Sync` not satisfied → Make fields thread-safe
- `Display` not satisfied → Implement `Display` trait or derive

### E0308: Mismatched Types

**Full Error:**
```
error[E0308]: mismatched types
  expected struct `HeaderMap`
  found struct `HashMap<String, String>`
```

**Cause:** Using wrong type (often due to generated code issues)

**Fix:**
```rust
// Change import
use http::HeaderMap;  // Instead of HashMap

// Update field type
pub default_headers: Option<HeaderMap>,  // Instead of HashMap
```

**Common mismatches:**
- `String` vs `&str` → Use `.to_string()` or `.as_str()`
- `i32` vs `u32` → Cast with `as` or change type
- `Vec<T>` vs `&[T]` → Use `.to_vec()` or reference

### E0599: No Method Found

**Full Error:**
```
error[E0599]: no method named `json` found for struct `reqwest::RequestBuilder`
```

**Cause:** Missing feature flag or wrong version

**Fix:**
```toml
# Add feature in Cargo.toml
[dependencies]
reqwest = { version = "^0.12", features = ["json"] }
```

## Lifetime and Borrow Errors

### E0106: Missing Lifetime Specifier

**Full Error:**
```
error[E0106]: missing lifetime specifier
```

**Cause:** Reference needs explicit lifetime parameter

**Fix Option 1 - Use owned types:**
```rust
// Instead of references
pub struct Config {
    pub base_path: String,  // Owned
    pub client: reqwest::Client,  // Owned
}
```

**Fix Option 2 - Add lifetime:**
```rust
pub struct Config<'a> {
    pub base_path: &'a str,
    pub client: &'a reqwest::Client,
}
```

**Recommendation:** Use Option 1 (owned types) for SDK clients

### E0502: Cannot Borrow as Mutable

**Full Error:**
```
error[E0502]: cannot borrow `x` as mutable because it is also borrowed as immutable
```

**Cause:** Trying to mutate while holding immutable reference

**Fix:**
```rust
// ❌ Problem
let config_ref = &configuration;
let mut modified = configuration;  // Error

// ✅ Solution: Clone
let config_ref = &configuration;
let mut modified = configuration.clone();
```

### E0507: Cannot Move Out of Borrowed Content

**Full Error:**
```
error[E0507]: cannot move out of `*self` which is behind a shared reference
```

**Cause:** Trying to move owned value from reference

**Fix:**
```rust
// ❌ Problem
pub fn get_value(&self) -> String {
    self.value  // Tries to move
}

// ✅ Solution: Clone
pub fn get_value(&self) -> String {
    self.value.clone()
}

// ✅ Or return reference
pub fn get_value(&self) -> &str {
    &self.value
}
```

## Module and Import Errors

### E0432: Unresolved Import

**Full Error:**
```
error[E0432]: unresolved import `http::HeaderMap`
```

**Cause:** Missing dependency

**Fix:**
```toml
# Add to Cargo.toml
[dependencies]
http = "^1.0"
```

### E0433: Failed to Resolve

**Full Error:**
```
error[E0433]: failed to resolve: use of undeclared crate or module `models`
```

**Cause:** Wrong import path

**Fix:**
```rust
// Use correct path
use crate::models::MyModel;  // From crate root
use super::models::MyModel;  // From parent module
```

## Async/Await Errors

### E0728: `await` Only Allowed in Async Functions

**Full Error:**
```
error[E0728]: `await` is only allowed inside `async` functions and blocks
```

**Cause:** Using `.await` in non-async function

**Fix:**
```rust
// ❌ Problem
pub fn list_pods(&self) -> Result<...> {
    crate::apis::pods_api::list_pods(&self.config).await
}

// ✅ Solution: Make function async
pub async fn list_pods(&self) -> Result<...> {
    crate::apis::pods_api::list_pods(&self.config).await
}
```

## Dependency Errors

### E0463: Can't Find Crate

**Full Error:**
```
error[E0463]: can't find crate for `serde_json`
```

**Cause:** Dependency not declared

**Fix:**
```toml
[dependencies]
serde_json = "^1.0"
```

### E0425: Cannot Find Value

**Full Error:**
```
error[E0425]: cannot find value `Arc` in this scope
```

**Cause:** Missing import

**Fix:**
```rust
use std::sync::Arc;
```

## SDK-Specific Errors

### Generated Code Type Mismatches

**Error:**
```
error[E0308]: mismatched types in `apis/configuration.rs`
```

**Cause:** OpenAPI generator created wrong types

**Fix:**
```rust
// In configuration.rs (manually fix or use script)
use http::HeaderMap;  // Instead of HashMap

pub struct Configuration {
    pub default_headers: Option<HeaderMap>,  // Updated type
    // ...
}
```

### Missing Feature Flags

**Error:**
```
error[E0599]: no method named `multipart` found
```

**Cause:** Missing reqwest feature

**Fix:**
```toml
[dependencies.reqwest]
version = "^0.12"
features = ["json", "multipart", "stream"]
```

## Quick Reference

| Error Code | Issue | Common Fix |
|------------|-------|-----------|
| E0277 | Missing trait | Add `#[derive(...)]` |
| E0308 | Type mismatch | Check types, fix imports |
| E0432 | Import failed | Add dependency |
| E0502 | Borrow conflict | Clone or restructure |
| E0599 | No method | Add feature flag |
| E0728 | Bad await | Make function async |

## Diagnostic Commands

### Explain Error Code

```bash
rustc --explain E0277
rustc --explain E0308
# etc.
```

### Check Types

```bash
# See what type rustc infers
cargo build 2>&1 | grep "expected"
```

### Check Features

```bash
# See available features
cargo tree -e features | grep reqwest
```

## Resources

- [Rust Error Index](https://doc.rust-lang.org/error-index.html)
- [Rust Book - Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [Common Rust Lifetime Misconceptions](https://github.com/pretzelhammer/rust-blog/blob/master/posts/common-rust-lifetime-misconceptions.md)
