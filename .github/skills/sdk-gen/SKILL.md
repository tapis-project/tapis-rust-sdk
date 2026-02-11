---
name: sdk-gen
description: Regenerate TAPIS Rust SDK service crates from OpenAPI specs using this repository's generation script. Use when starting a new service crate or refreshing generated code after API changes.
license: Apache-2.0
---

# SDK Generation from OpenAPI

## Scope

Use this skill to regenerate low-level Rust API/model code in `tapis-<service>/` crates.

This skill does:
- Run the repo's generation script against the OpenAPI registry
- Regenerate `src/apis`, `src/models`, docs, and crate manifest
- Capture known generator-specific issues to fix immediately after generation

This skill does not:
- Maintain wrapper ergonomics (`src/client.rs`) -> use `sdk-wrapper`
- Maintain parent crate re-exports/dependency wiring -> use `sdk-parent`
- Diagnose arbitrary compiler failures -> use `sdk-debug`

## Prerequisites

Required tools:
- `openapi-generator` (tested with 7.x)
- `jq`
- `curl`
- Rust toolchain

Verify:

```bash
openapi-generator version
jq --version
cargo --version
```

## Canonical Generation Command

Run from repository root:

```bash
bash .github/skills/sdk-gen/scripts/generate_rust_sdk.sh <env> <service>
```

Arguments:
- `<env>`: `prod`, `staging`, or `dev`
- `<service>`: service key in `.github/skills/sdk-gen/references/OpenAPI_specs.json`

Examples:

```bash
bash .github/skills/sdk-gen/scripts/generate_rust_sdk.sh prod pods
bash .github/skills/sdk-gen/scripts/generate_rust_sdk.sh prod authenticator
```

## What the Script Does

1. Finds repo root.
2. Maps environment to branch (`prod->main`, `staging->staging`, `dev->dev`).
3. Looks up spec URL from `.github/skills/sdk-gen/references/OpenAPI_specs.json`.
4. Downloads spec and runs `openapi-generator` into `tapis-<service>/`.

Operational notes:
- The script requires a clean git working tree when branch switching is needed.
- Running in `prod` while already on `main` avoids branch switching.

## Post-Generation Rules

Treat these as generated and avoid manual edits unless a documented generator bug requires it:
- `src/apis/*.rs`
- `src/models/*.rs`
- `src/apis/mod.rs`
- `src/models/mod.rs`

Safe files for manual integration:
- `src/client.rs`
- `src/lib.rs`
- crate `Cargo.toml`
- parent crate `Cargo.toml` / `src/lib.rs`

## Known Generator Issues (Current Repo)

### 1) Empty enum in `delete_client_200_response.rs`

Symptom:
- `expected identifier, found '}'` in `tapis-authenticator/src/models/delete_client_200_response.rs`

Fix:
- Replace `result: Option<Option<Result>>` with `result: Option<Option<serde_json::Value>>`
- Remove the empty `Result` enum and its `Default` impl

### 2) Multipart upload `Form::file` missing

Symptom:
- `no method named file found for struct Form`

Fix:
- Ensure Pods crate enables reqwest `stream` feature:

```toml
reqwest = { version = "^0.12", default-features = false, features = ["json", "multipart", "stream"] }
```

### 3) Wrapper exports dropped from generated `src/lib.rs`

Symptom:
- Wrapper compiles but is not exported

Fix:
- Re-add in service crate:

```rust
pub mod client;
pub use client::Tapis<ServiceName>;
```

## Verification Checklist

After regenerating and applying wrapper/parent integration:

```bash
cargo build --workspace --all-targets
```

Wrapper parity check (service example):

```bash
rg '^pub async fn ' tapis-pods/src/apis/*_api.rs | wc -l
rg '^[[:space:]]*pub async fn ' tapis-pods/src/client.rs | wc -l
```

Expected:
- Build succeeds
- Method counts match for each wrapped service

## Related Files

- Script: `scripts/generate_rust_sdk.sh`
- Spec registry: `references/OpenAPI_specs.json`
- Script docs: `scripts/README.md`
- Extra references:
  - `references/openapi-generator-config.md`
  - `references/tapis-api-specs.md`
  - `references/common-issues.md`
  - `references/best-practices.md`

## Handoff

After this skill:
1. Use `sdk-wrapper` to restore/update wrappers.
2. Use `sdk-parent` to verify parent dependency wiring and re-exports.
3. Use `sdk-debug` if build failures remain.
4. As the final step after workflow completion, run:
   `bash .github/skills/sdk-parent/scripts/bump_minor_version.sh`
