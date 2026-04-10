# Developer Notes

## Prerequisites

### 1) Python Environment (venv)

From repo root:

```bash
python3 -m venv .venv
source .venv/bin/activate
python -m pip install --upgrade pip
```

Note: `regenerate_all_sdks.py` uses Python standard library only (no extra pip deps required).

### 2) Rust Toolchain

Install Rust (if needed) and required components:

```bash
rustup default stable
rustup component add rustfmt clippy
```

### 3) OpenAPI Generation Dependencies

Install required CLI tools:

```bash
# macOS (example)
brew install jq curl openapi-generator
```

Or with npm:

```bash
npm install -g @openapitools/openapi-generator-cli
```

The generation script also relies on `jq` and `curl`.

## Regenerate All SDKs

Run from repo root:

```bash
python3 .github/skills/sdk-gen/scripts/regenerate_all_sdks.py --env prod
```

Version behavior:
- Generated sub-SDK crates use the current parent SDK version from root `Cargo.toml` via `packageVersion`.
- Optional override for generation:
  `export TAPIS_SDK_PACKAGE_VERSION=<target_version>`

Useful variants:

```bash
# Preview actions only
python3 .github/skills/sdk-gen/scripts/regenerate_all_sdks.py --env prod --dry-run

# Regenerate specific services only
python3 .github/skills/sdk-gen/scripts/regenerate_all_sdks.py --env prod --services pods,files

# Skip rustfmt (not recommended)
python3 .github/skills/sdk-gen/scripts/regenerate_all_sdks.py --env prod --skip-format

# Run clippy auto-fix before formatting/build
python3 .github/skills/sdk-gen/scripts/regenerate_all_sdks.py --env prod --run-clippy
```

## Validate Before Publish

The automation already runs:
- `cargo clippy --fix --allow-dirty --workspace --all-targets` (if `--run-clippy`)
- `cargo fmt --all` (unless `--skip-format`)
- `cargo build --workspace --all-targets`

Optional explicit checks:

```bash
cargo build --workspace --all-targets
cargo clippy --workspace --all-targets
```

## Publish to crates.io

Publishing is integrated into the same automation script.

1. Set a crates.io token first:

```bash
export CARGO_REGISTRY_TOKEN=<your_crates_io_token>
```

2. Run publish:

```bash
python3 .github/skills/sdk-gen/scripts/regenerate_all_sdks.py --env prod --publish
```

Standalone publish script (sub-SDKs first, parent last):

```bash
bash .github/skills/sdk-parent/scripts/publish_all_sdks.sh
```

Useful options:

```bash
# Show publish order without publishing
bash .github/skills/sdk-parent/scripts/publish_all_sdks.sh --list

# Cargo publish dry-run for each crate
bash .github/skills/sdk-parent/scripts/publish_all_sdks.sh --dry-run
```

Retry controls:

```bash
PUBLISH_RETRIES=5 PUBLISH_RETRY_DELAY=30 bash .github/skills/sdk-parent/scripts/publish_all_sdks.sh
```

Notes:
- The script fails fast if `CARGO_REGISTRY_TOKEN` is not set.
- It publishes all sub-crates first, then the parent crate.
- It retries failed publish commands automatically.
- `--run-clippy` applies clippy auto-fixes before rustfmt/build; rustfmt is enabled by default.

## GitHub Action (Weekly Regeneration)

Workflow file:

`/Users/wei.zhang/Developer/git/TAPIS/tapis-rust-sdk/.github/workflows/daily-sdk-regeneration.yml`

Behavior:
- Runs weekly via cron (`0 6 * * 1`) and supports manual trigger (`workflow_dispatch`).
- Executes:
  `python3 .github/skills/sdk-gen/scripts/regenerate_all_sdks.py --env prod --skip-bump --run-clippy`
- Opens/updates an automated PR with regeneration changes.

If you want a different schedule, edit the `cron` expression in the workflow file.

## GitHub Action (Manual Publish to crates.io)

Workflow file:

`/Users/wei.zhang/Developer/git/TAPIS/tapis-rust-sdk/.github/workflows/manual-sdk-publish.yml`

Important:
- This workflow currently contains a **fake** `CARGO_REGISTRY_TOKEN` on purpose.
- It has a safety gate that fails immediately while the token starts with `FAKE_TOKEN_`.
- This prevents accidental publishing before the official TACC crates.io account/token is ready.

When ready for real publish:
1. Update `CARGO_REGISTRY_TOKEN` in the workflow (or move to GitHub Secrets).
2. Trigger the workflow manually from the Actions tab.

## crates.io Multi-Account Conflicts

Yes, conflicts can happen if multiple owners try to publish the same crate version.

- crates.io crate names are global.
- Only crate owners can publish new versions.
- A version can be published only once. If two owners publish `0.x.y`, the first one wins and the second fails.

Recommended policy to avoid conflicts:

1. Use one release identity (preferably CI bot token) for publishing.
2. Keep version bumps and publish in one protected release workflow.
3. Restrict crate owners to a small trusted team.
4. Publish only from tagged releases/merge-to-main.

---

## Client Middleware Design

Every generated service client (e.g. `TapisJobs`) is built on a
`reqwest_middleware::ClientWithMiddleware` with a fixed middleware chain:

```
LoggingMiddleware → HeaderInjectionMiddleware → TrackingIdMiddleware → RefreshMiddleware (optional)
```

Each middleware is defined in `<service-crate>/src/client.rs` and regenerated by
`regenerate_all_sdks.py`.

---

### HeaderInjectionMiddleware

**Purpose:** Allow the caller to attach arbitrary HTTP headers to a single async
call without touching client configuration.

**Mechanism:**

```rust
tokio::task_local! {
    static EXTRA_HEADERS: HeaderMap;
}
```

`tokio::task_local!` creates a task-scoped slot — each async task has its own
independent copy.  The public helper `with_headers` sets the slot for the
duration of a future:

```rust
pub async fn with_headers<F, T>(headers: HeaderMap, f: F) -> T
where
    F: std::future::Future<Output = T>,
{
    EXTRA_HEADERS.scope(headers, f).await
}
```

`HeaderInjectionMiddleware::handle` reads the slot (if populated) and inserts
every header into the outgoing request via `req.headers_mut().insert(k, v)`.
Because it uses `insert`, the injected headers **override** same-named
client-level defaults — this allows per-call token impersonation or tenant
switching.

**Usage pattern:**

```rust
let mut hdrs = HeaderMap::new();
hdrs.insert("X-Tapis-Impersonation-User", "alice".parse()?);

let resp = tapis_jobs::with_headers(hdrs, async {
    client.jobs.get_job("abc-123").await
}).await?;
```

Concurrent calls with different headers are safe because each `tokio::spawn`
task has an independent task-local slot.

---

### TrackingIdMiddleware

**Purpose:** Validate and canonicalize the `X-Tapis-Tracking-ID` header before
a request leaves the client.  Mirrors tapipy's Python validation.

**Validation rules** (all enforced by `validate_tracking_id()`):

| Rule | Detail |
|------|--------|
| ASCII only | Non-ASCII characters are rejected |
| Max 126 chars | Matches tapipy constraint |
| Exactly one `.` | Must be `<namespace>.<unique_id>` form |
| Neither segment empty | Cannot start or end with `.` |
| Namespace charset | Alphanumeric + underscore only |
| Unique-id charset | Alphanumeric + hyphen only |

**Header aliasing:** Both `x-tapis-tracking-id` (HTTP convention) and
`x_tapis_tracking_id` (Python/env-var convention) are accepted
case-insensitively.  After validation the header is re-emitted as the canonical
lower-case `x-tapis-tracking-id`.

**Error handling:** A validation failure returns
`reqwest_middleware::Error::Middleware(anyhow::anyhow!(...))` which surfaces to
the caller as an `Err` before any network I/O is attempted.

---

### `tapis-core` Crate and `TokenProvider` Trait

**Problem:** Implementing automatic token refresh requires a service client
(e.g. `tapis-jobs`) to call `tapis-authenticator`.  But `tapis-authenticator`
itself depends on the SDK base types, which would create a circular dependency:

```
tapis-jobs → tapis-authenticator → tapis-jobs   ← CYCLE
```

**Solution — `tapis-core`:** A minimal crate with *no* HTTP or service
dependencies that defines the shared `TokenProvider` trait:

```rust
#[async_trait::async_trait]
pub trait TokenProvider: Send + Sync {
    async fn get_token(&self) -> Option<String>;
}
```

The dependency graph becomes acyclic:

```
tapis-jobs ──dep──> tapis-core <──impl── tapis-authenticator
                        ↑
                 (no dep on tapis-authenticator)
```

`tapis-authenticator` can implement `TokenProvider` and hand an
`Arc<dyn TokenProvider>` to any service client at construction time —
without any service crate knowing about `tapis-authenticator`.

**`TokenProvider` contract:**
- Return `Some(jwt_string)` when a fresh token is available.
- Return `None` to leave the current token unchanged (e.g. on transient error).
- Must never panic; use `None` for error cases.

---

### RefreshMiddleware

**Purpose:** Proactively refresh the `X-Tapis-Token` header before it expires,
mirroring tapipy's `_access_token` expiry check (tapipy.py lines 1209–1236).

**Triggering condition:** The JWT exp claim minus current Unix time < 5 seconds.
The check is performed on every request, but the overhead is negligible (a
base64url decode + JSON parse on a small payload).

**JWT exp extraction:**  Done without a JWT library by:
1. Splitting on `.` to obtain the base64url-encoded payload segment.
2. Decoding with an inline `decode_base64url()` (avoids a `base64` crate dep).
3. Deserializing with `serde_json` and reading the `"exp"` field.
Signature verification is intentionally skipped — we only need the expiry time.

**Infinite-loop protection:** Refresh is skipped when the request URL contains
`/oauth2/tokens` or `/v3/tokens`.  This prevents `RefreshMiddleware` from
triggering a token refresh *while processing a token creation/refresh request*,
which would recurse and deadlock.

**Wiring at construction time:**

```rust
// Without automatic refresh (static token):
let client = TapisJobs::new(base_url, Some(jwt))?;

// With automatic refresh — chain with_token_provider after new():
use std::sync::Arc;
use tapis_core::TokenProvider;

struct MyRefresher { /* ... */ }

#[async_trait::async_trait]
impl TokenProvider for MyRefresher {
    async fn get_token(&self) -> Option<String> {
        // call tapis-authenticator or any token source
        Some("new-jwt".into())
    }
}

let client = TapisJobs::new(base_url, Some(initial_jwt))?
    .with_token_provider(Arc::new(MyRefresher { /* ... */ }))?;
```

`RefreshMiddleware` is only inserted into the middleware chain when
`token_provider` is `Some`, so callers that don't need refresh pay zero cost.

