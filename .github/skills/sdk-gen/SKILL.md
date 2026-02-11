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

To avoid branch modifications during generation:

```bash
bash .github/skills/sdk-gen/scripts/generate_rust_sdk.sh --no-branch-switch <env> <service>
```

For full end-to-end automation across selected services:

```bash
python3 .github/skills/sdk-gen/scripts/regenerate_all_sdks.py --env prod
```

Arguments:
- `<env>`: `prod`, `staging`, or `dev`
- `<service>`: service key in `.github/skills/sdk-gen/references/OpenAPI_specs.json`

Examples:

```bash
bash .github/skills/sdk-gen/scripts/generate_rust_sdk.sh prod pods
bash .github/skills/sdk-gen/scripts/generate_rust_sdk.sh prod authenticator
```

Automation examples:

```bash
# Dry-run preview (recommended first)
python3 .github/skills/sdk-gen/scripts/regenerate_all_sdks.py --env prod --dry-run

# Only regenerate specific services
python3 .github/skills/sdk-gen/scripts/regenerate_all_sdks.py --env prod --services pods,authenticator

# Enable branch checkout in generation script (disabled by default)
python3 .github/skills/sdk-gen/scripts/regenerate_all_sdks.py --env prod --allow-branch-switch

# Skip DNS precheck for spec hosts (advanced)
python3 .github/skills/sdk-gen/scripts/regenerate_all_sdks.py --env prod --skip-network-precheck
```

Automation notes:
- `--services` limits generation/fixes/wrappers/examples to those services only.
- `regenerate_all_sdks.py` uses `--no-branch-switch` for generation by default, which avoids git checkout failures in restricted environments and on feature branches.
- `regenerate_all_sdks.py` performs a DNS precheck for spec hosts and fails fast when host resolution is unavailable.
- Parent workspace members and root dependency mappings are still refreshed for all known service crates, so subset runs do not drop crates from the parent SDK.
- Version bump is skipped automatically when generation fails for all requested services.

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

### 4) Invalid `models::serde_json::Value` emitted in some generated APIs

Symptom:
- `could not find serde_json in models`

Observed services:
- `tapis-meta`
- `tapis-streams`

Fix:
- Replace `models::serde_json::Value` with `serde_json::Value` in affected generated API files.
- If wrapper method signatures copied the bad type, patch wrappers too.

### 5) Files `HeaderByteRange` missing `Display`

Symptom:
- `HeaderByteRange doesn't implement std::fmt::Display`

Fix:
- Add `impl std::fmt::Display for HeaderByteRange` in `tapis-files/src/models/header_byte_range.rs`.

### 6) Service versions reset by generator

Symptom:
- Regenerated crates revert to `1.0.0`.

Fix:
- After all generation/wrapper/debug tasks are complete, run:
  `bash .github/skills/sdk-parent/scripts/bump_minor_version.sh`

### 7) Branch checkout can fail in restricted environments

Symptom:
- Generation fails while trying to `git checkout` environment branch.

Fix:
- Run generation with `--no-branch-switch`.
- `regenerate_all_sdks.py` already does this by default; use `--allow-branch-switch` only when branch switching is explicitly desired.

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

Targeted bug scans:

```bash
rg -n 'Self::\\s*$' tapis-*/src/models
rg -n 'models::serde_json::Value' tapis-*/src/apis tapis-*/src/client.rs
```

Expected:
- Build succeeds
- Method counts match for each wrapped service

## Related Files

- Script: `scripts/generate_rust_sdk.sh`
- Full automation script: `scripts/regenerate_all_sdks.py`
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
