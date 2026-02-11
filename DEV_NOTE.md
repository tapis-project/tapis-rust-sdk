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
