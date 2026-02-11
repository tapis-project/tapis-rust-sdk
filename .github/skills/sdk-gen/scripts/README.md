# SDK Generation Script

Automation script for regenerating TAPIS Rust SDK service crates.

## Script

- `generate_rust_sdk.sh`
- `regenerate_all_sdks.py`

## Usage

From repository root:

```bash
bash .github/skills/sdk-gen/scripts/generate_rust_sdk.sh <env> <service>
```

If your workflow should not modify branches:

```bash
bash .github/skills/sdk-gen/scripts/generate_rust_sdk.sh --no-branch-switch <env> <service>
```

Examples:

```bash
bash .github/skills/sdk-gen/scripts/generate_rust_sdk.sh prod pods
bash .github/skills/sdk-gen/scripts/generate_rust_sdk.sh prod authenticator
bash .github/skills/sdk-gen/scripts/generate_rust_sdk.sh staging files
```

## Behavior

The script:
1. Maps env to branch (`prod->main`, `staging->staging`, `dev->dev`).
2. Reads spec URL from `.github/skills/sdk-gen/references/OpenAPI_specs.json`.
3. Downloads spec via `curl`.
4. Runs openapi-generator into `tapis-<service>/`.

## Requirements

- `openapi-generator`
- `jq`
- `curl`
- Clean git tree if branch switching is required

## Outputs

Generated service crate under repo root:

- `tapis-<service>/Cargo.toml`
- `tapis-<service>/src/apis/*`
- `tapis-<service>/src/models/*`
- `tapis-<service>/docs/*`

## Follow-up

After generation, run:

```bash
cargo build --workspace --all-targets
```

Then apply wrapper/parent fixes using `sdk-wrapper` and `sdk-parent` skills.

## Full Automation

For end-to-end regeneration (generation + known fixes + wrappers + examples + parent wiring + build + optional version bump), use:

```bash
python3 .github/skills/sdk-gen/scripts/regenerate_all_sdks.py --env prod
```

Useful flags:

```bash
# Preview actions only
python3 .github/skills/sdk-gen/scripts/regenerate_all_sdks.py --env prod --dry-run

# Regenerate specific services only
python3 .github/skills/sdk-gen/scripts/regenerate_all_sdks.py --env prod --services pods,authenticator

# Skip final bump (if you want manual control)
python3 .github/skills/sdk-gen/scripts/regenerate_all_sdks.py --env prod --skip-bump

# Let the generation script perform branch checkout (default is disabled)
python3 .github/skills/sdk-gen/scripts/regenerate_all_sdks.py --env prod --allow-branch-switch

# Skip DNS precheck (only if your environment has custom DNS/network behavior)
python3 .github/skills/sdk-gen/scripts/regenerate_all_sdks.py --env prod --skip-network-precheck

# Publish all sub-crates + parent crate to crates.io as final step
python3 .github/skills/sdk-gen/scripts/regenerate_all_sdks.py --env prod --publish

# Run clippy auto-fix before formatting/build
python3 .github/skills/sdk-gen/scripts/regenerate_all_sdks.py --env prod --run-clippy

# Skip rustfmt (enabled by default)
python3 .github/skills/sdk-gen/scripts/regenerate_all_sdks.py --env prod --skip-format
```

Notes:
- `regenerate_all_sdks.py` calls `generate_rust_sdk.sh --no-branch-switch` by default to avoid branch checkout failures in restricted or feature-branch workflows.
- `regenerate_all_sdks.py` runs a DNS precheck for spec hosts before generation and fails fast when hosts are not resolvable.
- `--run-clippy` applies `cargo clippy --fix --allow-dirty --workspace --all-targets` before rustfmt/build.
- `regenerate_all_sdks.py` runs `cargo fmt --all` by default to enforce consistent Rust formatting.
- `--services` scopes generation/fix/wrapper/example work, but parent workspace/dependency wiring remains complete for all known service crates.
- Version bump is skipped automatically if generation fails for every requested service.
- Final version bump runs last unless `--skip-bump` is set.
- Publishing requires `CARGO_REGISTRY_TOKEN` to be set:
  `export CARGO_REGISTRY_TOKEN=<your_crates_io_token>`
