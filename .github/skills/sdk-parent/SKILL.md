---
name: sdk-parent
description: Maintain the tapis-sdk parent crate: workspace membership, parent dependency mappings, and namespaced re-exports for all service crates.
metadata:
  author: tapis
  version: "1.1"
---

# Parent Crate Management

This skill maintains the root `tapis-sdk` crate that re-exports service crates.

## Rules

### 1) Keep namespaced re-exports

In root `src/lib.rs`, re-export each service in its own module:

```rust
pub mod authenticator {
    pub use tapis_authenticator::*;
}

pub mod pods {
    pub use tapis_pods::*;
}
```

Do not flatten with multiple `pub use ...::*;` at crate root.

### 2) Handle package-name vs dependency-key mismatches

OpenAPI-generated package names may be underscore-style (`tapis_authenticator`, `tapis_pods`) while parent dependency keys are often hyphen-style (`tapis-authenticator`, `tapis-pods`).

Use explicit package mapping in root `Cargo.toml`:

```toml
[dependencies]
tapis-authenticator = { package = "tapis_authenticator", version = "1.0.0", path = "./tapis-authenticator" }
tapis-pods = { package = "tapis_pods", version = "1.0.0", path = "./tapis-pods" }
```

### 3) Workspace membership

Ensure every service crate is present in `[workspace].members`.

### 4) Service export expectations

Each service crate should export its wrapper from `src/lib.rs`:

```rust
pub mod client;
pub use client::TapisPods; // service-specific type
```

## Verification

```bash
cargo build --workspace --all-targets
```

## Version Bump Script (Final Workflow Step)

After the full SDK workflow succeeds (generation + wrapper + parent wiring + debug/build), bump versions in one shot:

```bash
bash .github/skills/sdk-parent/scripts/bump_minor_version.sh
```

What it does:
- Reads root `Cargo.toml` package version.
- Bumps to next minor (`MAJOR.(MINOR+1).0`).
- Applies the same version to root and all workspace member crate package versions.
- Updates inline path dependency `version = "..."` fields in workspace manifests.

Dry run:

```bash
bash .github/skills/sdk-parent/scripts/bump_minor_version.sh --dry-run
```

If adding a new service crate:
1. Add workspace member.
2. Add mapped dependency in root `Cargo.toml`.
3. Add namespaced re-export module in root `src/lib.rs`.
4. Build workspace.
5. Run the version bump script as the final step.
