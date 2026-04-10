# Changelog

All notable changes to this project will be documented in this file.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [0.3.0] — 2026-04-10

### Added

- **`tapis-core` crate** — new published crate providing shared traits and types across the SDK.
  - Introduces the `TokenProvider` trait (`async fn get_token(&self) -> Option<String>`) for implementing custom token refresh strategies.
  - Re-exported from the umbrella crate as `tapis_sdk::core::TokenProvider`.

- **`RefreshMiddleware`** — transparent token refresh on every service client.
  - When a JWT has fewer than 5 seconds until expiry, the middleware calls the provider and substitutes a fresh token automatically.
  - New constructor available on all 16 service clients: `TapisClient::with_token_provider(base_url, jwt_token, Arc<dyn TokenProvider>)`.

- **`HeaderInjectionMiddleware`** — per-call header injection via `with_headers(headers, future)`.
  - Scopes additional HTTP headers to a single call or async block without rebuilding the client.
  - Supports per-call auth patterns (e.g., impersonation, multi-tenant proxying).
  - The `X-Tapis-Token` set at client construction is still sent automatically.

- **`TrackingIdMiddleware`** — automatic forwarding and validation of `X-Tapis-Tracking-Id` headers.
  - Enforces Tapis tracking ID format (ASCII, ≤126 chars, exactly one `.`, valid charset).

- **`PUBLISH.md`** — step-by-step publish workflow document, including a compilation-testing step (`--skip-bump --run-clippy`) before version bumping.

### Changed

- All 16 service crates regenerated from live Tapis v3 OpenAPI specs.
- Middleware stack order on every service client: `LoggingMiddleware → HeaderInjectionMiddleware → TrackingIdMiddleware → RefreshMiddleware`.
- `tapis-core` added as a workspace member and re-exported from the umbrella crate as `tapis_sdk::core`.

### Fixed

- Resolved all Clippy warnings across the workspace:
  - `clippy::empty_docs` — empty doc comment lines stripped from generated model files.
  - `clippy::field_reassign_with_default` — configuration construction changed to struct init syntax.
  - `clippy::clippy_splitn` — `token.split('.').nth(1)` used in place of `splitn`.
- Fixed ordering bug in the code-fix pipeline where `fix_empty_docs` would strip doc comment sentinels before `fix_empty_enum_defaults` could use them, causing `cargo fmt` parse failures.

### Migration from `0.2.0`

- `TapisClient::new(base_url, jwt_token)` — unchanged, no migration needed.
- To opt in to token refresh: replace `::new(...)` with `::with_token_provider(base_url, jwt_token, Arc::new(MyProvider))`.
- Add `tapis-core = "0.3.0"` to your `Cargo.toml` only if you implement `TokenProvider` directly; otherwise it is pulled in transitively through any service crate.

---

## [0.2.0] — Initial release

- Generated Rust SDK for all 16 Tapis v3 services.
- Umbrella crate (`tapis-sdk`) re-exporting all service modules.
- `LoggingMiddleware` on every service client.
- Configurable TLS: `native-tls` (default) and `rustls-tls` feature flags.
