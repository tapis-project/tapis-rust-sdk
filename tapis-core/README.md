# tapis-core

[![Crates.io](https://img.shields.io/crates/v/tapis-core.svg)](https://crates.io/crates/tapis-core)
[![Documentation](https://docs.rs/tapis-core/badge.svg)](https://docs.rs/tapis-core)
[![License](https://img.shields.io/crates/l/tapis-core.svg)](https://github.com/tapis-project/tapis-rust-sdk/blob/main/LICENSE)

Shared traits and types for the [Tapis Rust SDK](https://github.com/tapis-project/tapis-rust-sdk).

This crate is intentionally small and has **no** HTTP or network dependencies. It exists solely to break the circular-dependency problem that would arise if service crates directly referenced `tapis-authenticator` for token refresh.

## What's in this crate

### `TokenProvider`

An async trait that defines the contract for obtaining a fresh Tapis JWT:

```rust
#[async_trait::async_trait]
pub trait TokenProvider: Send + Sync {
    async fn get_token(&self) -> Option<String>;
}
```

Return `Some(jwt_string)` to supply a fresh token, or `None` to leave the current token unchanged (e.g., on a transient error).

## Dependency graph

```
tapis-jobs ──dep──> tapis-core <──impl── tapis-authenticator
tapis-systems ────>     ▲
tapis-apps ───────>     │
...                (no service crate depends on tapis-authenticator)
```

`tapis-authenticator` can implement `TokenProvider` and pass it into any service client without creating a cycle.

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
tapis-core = "0.3.1"
async-trait = "0.1"
```

Implement the trait on any type that can produce a token:

```rust
use std::sync::Arc;
use async_trait::async_trait;
use tapis_core::TokenProvider;

struct StaticToken(String);

#[async_trait]
impl TokenProvider for StaticToken {
    async fn get_token(&self) -> Option<String> {
        Some(self.0.clone())
    }
}
```

Then pass it to any service client constructor:

```rust
use tapis_jobs::TapisJobs;

let provider: Arc<dyn TokenProvider> = Arc::new(StaticToken("my-jwt".into()));
let jobs = TapisJobs::with_token_provider(&base_url, None, provider)?;
```

> **Note:** If you are using the umbrella crate, `TokenProvider` is re-exported as `tapis_sdk::core::TokenProvider` — you do not need to add `tapis-core` as a direct dependency.

## Infinite-loop protection

`RefreshMiddleware` (in each service crate) skips the refresh call when the outgoing request URL contains `/oauth2/tokens` or `/v3/tokens`, preventing the token provider from recursively triggering itself.

## License

BSD-3-Clause. See [LICENSE](https://github.com/tapis-project/tapis-rust-sdk/blob/main/LICENSE).
