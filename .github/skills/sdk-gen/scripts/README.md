# SDK Generation Script

Automation script for regenerating TAPIS Rust SDK service crates.

## Script

- `generate_rust_sdk.sh`

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
