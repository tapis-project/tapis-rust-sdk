# TAPIS Rust SDK Agent Skills

This directory contains modular agent skills for working with TAPIS Rust SDKs.

## Available Skills

### [sdk-gen](./sdk-gen/SKILL.md)
Generate base Rust SDK crates from TAPIS OpenAPI specs using the repository automation script.

Use this when:
- Regenerating an existing service crate (`tapis-pods`, `tapis-authenticator`, etc.)
- Adding a new TAPIS service crate from the OpenAPI registry

### [sdk-wrapper](./sdk-wrapper/SKILL.md)
Create or repair high-level wrapper clients (`Tapis*`) on top of generated APIs.

Use this when:
- `src/lib.rs` was overwritten by regeneration and wrapper exports were dropped
- New endpoints were generated and wrapper coverage is no longer 100%

### [sdk-debug](./sdk-debug/SKILL.md)
Fix build failures and warning regressions after generation/wrapper updates.

Use this when:
- `cargo build` fails
- New generator output introduces known Rust/OpenAPI quirks

### [sdk-parent](./sdk-parent/SKILL.md)
Maintain the parent `tapis-sdk` crate and workspace exports.

Use this when:
- Service crate names or package IDs change
- Parent re-exports or dependency mappings need updates

## Canonical Workflow

1. `sdk-gen` - Regenerate service crate(s) from OpenAPI.
2. `sdk-wrapper` - Restore/update wrappers and `lib.rs` exports.
3. `sdk-parent` - Ensure parent dependency mappings and re-exports are correct.
4. `sdk-debug` - Build all targets and clear errors/warnings.
5. `sdk-parent` - As the final step, bump repo-wide minor version for root + all sub-crates:
   `bash .github/skills/sdk-parent/scripts/bump_minor_version.sh`

## Verification Baseline

Run from repository root:

```bash
cargo build --workspace --all-targets
```

If regeneration touched wrappers, also verify wrapper parity:

```bash
# service example
rg '^pub async fn ' tapis-pods/src/apis/*_api.rs | wc -l
rg '^[[:space:]]*pub async fn ' tapis-pods/src/client.rs | wc -l
```
