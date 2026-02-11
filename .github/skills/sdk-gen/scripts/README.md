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
```

Notes:
- `--services` scopes generation/fix/wrapper/example work, but parent workspace/dependency wiring remains complete for all known service crates.
- Final version bump runs last unless `--skip-bump` is set.
