---
name: sdk-parent
description: Maintains parent Rust SDK crate structure with module re-exports and version synchronization between parent and sub-crates. Use when updating version numbers, adding new service crates, or modifying the tapis-sdk parent crate structure.
metadata:
  author: tapis
  version: "1.0"
---

# Tapis SDK Parent Crate Management

This skill ensures proper maintenance of the parent `tapis-sdk` crate that re-exports multiple service-specific crates (like `tapis-authenticator`, `tapis-pods`, etc.).

## Structure Overview

The project uses a Rust workspace with:
- **Root crate (`tapis-sdk`)**: Parent/umbrella crate that re-exports all service crates
- **Service crates**: Individual crates for each Tapis service (`tapis-authenticator`, `tapis-pods`, etc.)
- All crates share the same version number for consistency

## Key Rules

### 1. Module Re-Export Pattern

The parent crate (`src/lib.rs`) MUST use **namespaced module re-exports** to avoid name conflicts:

```rust
/// Tapis Authenticator service client
pub mod authenticator {
    pub use tapis_authenticator::*;
}

/// Tapis Pods service client
pub mod pods {
    pub use tapis_pods::*;
}
```

**DO NOT** use flat glob re-exports as they cause ambiguous name conflicts:

```rust
// ❌ WRONG - causes ambiguous glob re-export warnings
pub use tapis_authenticator::*;
pub use tapis_pods::*;
```

### 2. Version Synchronization

All crates MUST maintain the **same version number**:

- Root `Cargo.toml` (`tapis-sdk`): `version = "X.Y.Z"`
- `tapis-authenticator/Cargo.toml`: `version = "X.Y.Z"`
- `tapis-pods/Cargo.toml`: `version = "X.Y.Z"`
- Any other service crates: `version = "X.Y.Z"`

**When updating versions:**

1. Update the version in ALL `Cargo.toml` files simultaneously
2. Use the `multi_replace_string_in_file` tool to update all files at once
3. Ensure consistency across the workspace

### 3. Dependency Management

**During Development (path dependencies):**

```toml
[dependencies]
tapis-authenticator = { path = "./tapis-authenticator" }
tapis-pods = { path = "./tapis-pods" }
```

**Before Publishing to crates.io (version dependencies):**

```toml
[dependencies]
tapis-authenticator = "X.Y.Z"
tapis-pods = "X.Y.Z"
```

### 4. Publishing Order

When publishing to crates.io:

1. **First**: Publish individual service crates (`tapis-authenticator`, `tapis-pods`)
2. **Then**: Update parent `Cargo.toml` to use version-based dependencies
3. **Finally**: Publish the parent crate (`tapis-sdk`)

crates.io does not support path dependencies, so the individual crates must be published first.

## Common Tasks

### Adding a New Service Crate

1. Create the service crate in the workspace
2. Add it to `[workspace]` members in root `Cargo.toml`
3. Add path dependency in root `[dependencies]`
4. Add module re-export in `src/lib.rs`:
   ```rust
   /// Tapis NewService client
   pub mod newservice {
       pub use tapis_newservice::*;
   }
   ```
5. Ensure the new crate has the same version number

### Updating Version Numbers

Use `multi_replace_string_in_file` to update all `Cargo.toml` files:

```rust
multi_replace_string_in_file([
    {
        filePath: "Cargo.toml",
        oldString: 'version = "1.0.0"',
        newString: 'version = "1.1.0"'
    },
    {
        filePath: "tapis-authenticator/Cargo.toml",
        oldString: 'version = "1.0.0"',
        newString: 'version = "1.1.0"'
    },
    {
        filePath: "tapis-pods/Cargo.toml",
        oldString: 'version = "1.0.0"',
        newString: 'version = "1.1.0"'
    }
])
```

### Preparing for Publication

1. Verify all versions are synchronized
2. Test compilation: `cargo build --release`
3. Check for warnings: `cargo clippy`
4. Replace path dependencies with version dependencies in root `Cargo.toml`
5. Update documentation if needed

## Troubleshooting

### Ambiguous Glob Re-export Warnings

**Symptom**: Warnings like `ambiguous glob re-exports` for names like `apis`, `client`, `models`

**Cause**: Using flat glob re-exports (`pub use tapis_authenticator::*`)

**Solution**: Use namespaced module re-exports as shown in Rule #1

### Version Mismatch Errors

**Symptom**: Dependency resolution errors or unexpected behavior

**Cause**: Version numbers not synchronized across crates

**Solution**: Update all `Cargo.toml` files to use the same version

### Publication Fails for Parent Crate

**Symptom**: `cargo publish` fails with errors about path dependencies

**Cause**: Parent crate still uses path dependencies instead of version dependencies

**Solution**: Change path dependencies to version dependencies after publishing sub-crates

## References

- Workspace structure: Root directory contains all service crates
- Each service crate is independently published to crates.io
- Parent crate provides convenience for users wanting all services
