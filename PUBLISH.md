# Publishing a New Version to Crates.io

This document describes the end-to-end workflow for publishing a new version of the TAPIS Rust SDK.

## Prerequisites

Before publishing, replace the placeholder token in `.github/workflows/manual-sdk-publish.yml`:

```yaml
# Change this:
CARGO_REGISTRY_TOKEN: "FAKE_TOKEN_REPLACE_WITH_TACC_CRATES_IO_TOKEN"

# To a real token (or move it to GitHub Secrets and reference it):
CARGO_REGISTRY_TOKEN: ${{ secrets.CARGO_REGISTRY_TOKEN }}
```

---

## Step 0 — Compilation Test (No Version Bump)

Run a full regeneration from live OpenAPI specs, apply all code fixes, and validate with Clippy — **without bumping versions**. Use this to verify the workspace is clean before committing.

```bash
python3 .github/skills/sdk-gen/scripts/regenerate_all_sdks.py --env prod --skip-bump --run-clippy
```

This will:
- Fetch live OpenAPI specs from the Tapis production environment
- Regenerate all 16 service crates
- Apply all code fix passes (`fix_empty_enum_defaults`, `fix_empty_docs`, etc.)
- Run `cargo fmt --all`
- Run `cargo clippy` (zero-warning policy)
- Run `cargo build`

If this completes with `Automation workflow completed.` and no warnings or errors, the workspace is ready to publish.

---

## Step 1 — Regenerate and Bump Versions

Run the full regeneration pipeline **with** version bumping:

```bash
python3 .github/skills/sdk-gen/scripts/regenerate_all_sdks.py --env prod --run-clippy
```

This performs the same steps as Step 0 and additionally increments the patch version in every crate's `Cargo.toml`.

---

## Step 2 — Review the Changes

Inspect the diff to confirm:

- All `Cargo.toml` files have the new version number
- No unexpected changes to generated models or APIs
- `tapis-core` and the root `Cargo.toml` / `src/lib.rs` are consistent

```bash
git diff --stat
```

---

## Step 3 — Commit to `main`

```bash
git add -A
git commit -m "chore: regenerate and bump SDK to vX.Y.Z"
git push origin main
```

> The CI regeneration workflow (if configured) runs with `--skip-bump` and does **not** publish. The publish step must be triggered manually.

---

## Step 4 — Trigger the Publish Workflow

1. Go to the repository on GitHub.
2. Navigate to **Actions → Manual SDK Publish (Crates.io)**.
3. Click **Run workflow** and select `main`.

The workflow will:
1. Validate with `cargo fmt --all --check`, `cargo clippy`, and `cargo build`
2. Check that `CARGO_REGISTRY_TOKEN` is not the fake placeholder
3. Publish each sub-crate in dependency order (`tapis-core` first, then service crates, then the root `tapis-sdk`)

---

## Quick Reference

| Command | Purpose |
|---|---|
| `--skip-bump --run-clippy` | Compile test, no version change |
| `--run-clippy` | Full regenerate + bump + clippy |
| `--skip-generate --skip-bump` | Re-apply code fixes only (no OpenAPI fetch, no bump) |
| `--skip-bump` | Full regenerate, no bump (CI default) |
