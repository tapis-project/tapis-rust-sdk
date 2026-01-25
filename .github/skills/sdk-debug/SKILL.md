---
name: sdk-debug
description: Fix compilation errors, remove warnings, and resolve dependency issues in TAPIS Rust SDKs. Handles common OpenAPI generator issues like type mismatches, missing dependencies, and lint warnings. Use when SDK fails to compile or has many warnings after generation or wrapper creation.
license: Apache-2.0
---

# SDK Debug & Compilation Fix

## Overview

Systematically identify and fix compilation errors, warnings, and dependency issues in generated Rust SDKs.

**What This Skill Does:**
- Diagnose and fix compilation errors
- Resolve dependency conflicts
- Remove clippy warnings
- Fix formatting issues
- Handle OpenAPI generator quirks

**Prerequisites:**
- SDK generated and/or wrapper created
- Compilation attempted (`cargo build` run)

---

## Diagnostic Process

### Step 1: Identify Error Types

```bash
cargo build 2>&1 | tee build_errors.txt
```

**Common Error Categories:**
1. Missing dependencies
2. Type mismatches
3. Unused imports/variables
4. Lifetime errors
5. Trait bound issues

### Step 2: Count Issues

```bash
# Count errors
grep "error\[E" build_errors.txt | wc -l

# Count warnings
grep "warning:" build_errors.txt | wc -l

# List unique error codes
grep "error\[E" build_errors.txt | sed 's/.*error\[E\([0-9]*\)\].*/E\1/' | sort -u
```

---

## Common Errors & Fixes

### Error 1: Missing Dependencies

**Symptom:**
```
error[E0432]: unresolved import `http::HeaderMap`
```

**Solution:**
```toml
# Add to Cargo.toml [dependencies]
http = "^1.0"
```

**Common missing deps:**
```toml
[dependencies]
reqwest = { version = "^0.12", features = ["json", "multipart"] }
serde = { version = "^1.0", features = ["derive"] }
serde_json = "^1.0"
http = "^1.0"
tokio = { version = "^1.0", features = ["full"] }
```

### Error 2: Type Mismatch in Builder

**Symptom:**
```
error[E0308]: mismatched types
  expected struct `HeaderMap`
  found struct `std::collections::HashMap`
```

**Cause:** OpenAPI generator sometimes uses wrong types.

**Solution:**

Find the issue:
```bash
grep -n "HashMap" src/apis/configuration.rs
```

Fix in `src/apis/configuration.rs`:
```rust
// ❌ Wrong (generated)
use std::collections::HashMap;
pub default_headers: Option<HashMap<String, String>>,

// ✅ Correct
use http::HeaderMap;
pub default_headers: Option<HeaderMap>,
```

### Error 3: Unused Imports

**Symptom:**
```
warning: unused import: `std::collections::HashMap`
```

**Solution:**
```bash
# Find all unused imports
cargo build 2>&1 | grep "unused import"

# Remove them manually or use:
cargo fix --allow-dirty
```

### Error 4: Missing Lifetime Parameters

**Symptom:**
```
error[E0106]: missing lifetime specifier
```

**Solution:** Usually in configuration or client structs:

```rust
// ❌ Wrong
pub struct Configuration {
    pub base_path: String,
    pub client: reqwest::Client,
}

// ✅ Correct (if needed)
pub struct Configuration<'a> {
    pub base_path: String,
    pub client: &'a reqwest::Client,
}
```

**Note:** Usually not needed with `Arc` wrapping.

### Error 5: Trait Bound Not Satisfied

**Symptom:**
```
error[E0277]: the trait bound `MyType: Serialize` is not satisfied
```

**Solution:** Add derive macro:

```rust
// ❌ Missing derive
pub struct MyType {
    pub field: String,
}

// ✅ Add Serialize/Deserialize
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MyType {
    pub field: String,
}
```

### Error 6: Mutable Borrow Issues

**Symptom:**
```
error[E0502]: cannot borrow as mutable because it is also borrowed as immutable
```

**Solution:** Use cloning or refactor:

```rust
// ❌ Borrow conflict
let config_ref = &configuration;
let mut modified = configuration;

// ✅ Clone
let config_ref = &configuration;
let mut modified = configuration.clone();
```

---

## Fixing Warnings

### Run Clippy

```bash
cargo clippy --all-targets -- -D warnings
```

### Common Clippy Warnings

#### 1. Unnecessary `.into()`

**Warning:**
```
warning: unnecessary `.into()` call
```

**Fix:**
```rust
// ❌ Before
let value: String = some_value.into();

// ✅ After
let value = some_value;  // Already correct type
```

#### 2. Needless Borrow

**Warning:**
```
warning: this expression borrows a reference that is immediately dereferenced
```

**Fix:**
```rust
// ❌ Before
function_call(&some_ref);

// ✅ After
function_call(some_ref);  // Already a reference
```

#### 3. Redundant Clone

**Warning:**
```
warning: redundant clone
```

**Fix:**
```rust
// ❌ Before
let value = thing.clone();
return value;

// ✅ After
return thing;  // No clone needed
```

#### 4. Too Many Arguments

**Warning:**
```
warning: this function has too many arguments (8/7)
```

**Fix:** Use a struct:

```rust
// ❌ Before
pub fn create_pod(
    name: String,
    image: String,
    cpu: i32,
    memory: i32,
    // ... 8 parameters
) { }

// ✅ After
pub struct PodParams {
    pub name: String,
    pub image: String,
    pub cpu: i32,
    pub memory: i32,
    // ...
}

pub fn create_pod(params: PodParams) { }
```

#### 5. Large Enum Variants

**Warning:**
```
warning: large size difference between variants
```

**Fix:** Box large variants:

```rust
// ❌ Before
pub enum MyEnum {
    Small(i32),
    Large(VeryBigStruct),
}

// ✅ After
pub enum MyEnum {
    Small(i32),
    Large(Box<VeryBigStruct>),
}
```

---

## Formatting Issues

### Auto-Format

```bash
cargo fmt
```

### Check Formatting Without Changing

```bash
cargo fmt -- --check
```

### Common Format Issues

1. **Inconsistent indentation** → `cargo fmt` fixes
2. **Line too long** → `cargo fmt` wraps
3. **Missing trailing commas** → `cargo fmt` adds
4. **Inconsistent spacing** → `cargo fmt` normalizes

---

## Dependency Conflicts

### Symptom

```
error: failed to select a version for `reqwest`
  required by package `tapis-pods v1.0.0`
  versions that meet the requirements `^0.11` are: 0.11.27, ...
  the package `other-crate` depends on `reqwest`, with features: `json` but `reqwest` does not have these features.
```

### Solution 1: Align Versions

```toml
# Update all deps to compatible versions
[dependencies]
reqwest = { version = "^0.12", features = ["json", "multipart"] }
```

### Solution 2: Check Feature Flags

```bash
# See what features are available
cargo tree -e features | grep reqwest
```

### Solution 3: Update All

```bash
cargo update
```

---

## Generated Code Issues

### Issue: Redundant Code in Generated Files

**Example:** Duplicate header injection code in `src/apis/*_api.rs`:

```rust
// ❌ Found in generated code (breaks regeneration)
if let Some(ref tapis_token) = configuration.tapis_token {
    req_builder = req_builder.header("X-Tapis-Token", tapis_token);
}
```

**Solution:** 
- DO NOT modify generated files manually
- Use wrapper with global headers (see `sdk-wrapper` skill)
- If absolutely needed, create post-generation script

### Issue: Incorrect Import Paths

**Symptom:**
```
error[E0432]: unresolved import `crate::models::SomeModel`
```

**Solution:** Check module structure:

```bash
# Find where model is defined
find src -name "*.rs" -exec grep "pub struct SomeModel" {} +

# Fix import path
use crate::models::some_model::SomeModel;  # Correct path
```

---

## Systematic Debugging Process

### 1. Clean Build

```bash
cargo clean
cargo build 2>&1 | tee errors.txt
```

### 2. Fix Errors by Priority

**Order:**
1. Missing dependencies → Add to `Cargo.toml`
2. Type errors → Fix type signatures
3. Borrow checker → Add clones or refactor
4. Trait bounds → Add derives
5. Warnings → Run clippy and fix

### 3. Fix One Error at a Time

```bash
# Build, fix first error, repeat
cargo build
# Fix first error
cargo build
# Fix next error
# Continue until clean
```

### 4. Verify After Each Fix

```bash
# After each fix
cargo build

# If builds, run tests
cargo test

# Check for warnings
cargo clippy
```

---

## Testing After Fixes

### Unit Tests

```bash
cargo test
```

### Integration Test

Create `tests/integration_test.rs`:

```rust
use tapis_pods::client::TapisClient;

#[tokio::test]
async fn test_client_creation() {
    let client = TapisClient::new(
        "https://api.tapis.io/v3",
        "fake_token"
    );
    assert!(client.is_ok());
}

#[tokio::test]
async fn test_all_sub_clients() {
    let client = TapisClient::new(
        "https://api.tapis.io/v3",
        "fake_token"
    ).unwrap();
    
    // Verify all sub-clients are accessible
    let _ = &client.pods;
    let _ = &client.volumes;
    let _ = &client.templates;
    // Test all sub-clients...
}
```

Run:
```bash
cargo test --tests
```

---

## Prevention Strategies

### 1. Use Stable Generator Version

```bash
# Pin openapi-generator version
openapi-generator-cli version-manager set 7.2.0
```

### 2. Validate OpenAPI Spec

```bash
# Before generation
openapi-generator-cli validate -i tapis-pods-api.yaml
```

### 3. Use Generation Script

See `sdk-gen` skill for automated generation with validation.

### 4. Version Control Generated Code

```bash
# Commit generated code
git add src/
git commit -m "Generated SDK from spec v1.2.3"

# Easy to revert if needed
git diff HEAD~1 src/
```

---

## Emergency Fixes

### Complete Reset

```bash
# Nuclear option: regenerate everything
rm -rf src/ docs/
cargo clean

# Regenerate (see sdk-gen skill)
./sdk-tools/generate_rust_sdk.sh tapis-pods-api.yaml tapis_pods

# Recreate wrapper (see sdk-wrapper skill)
# ... implement client.rs again
```

### Selective Reset

```bash
# Reset just one API module
git checkout HEAD -- src/apis/pods_api.rs

# Regenerate if needed
```

---

## Verification Checklist

After all fixes:

- [ ] `cargo clean && cargo build` succeeds with no errors
- [ ] `cargo clippy -- -D warnings` passes with no warnings
- [ ] `cargo fmt -- --check` shows no format issues
- [ ] `cargo test` passes all tests
- [ ] Example code runs successfully
- [ ] All dependencies in `Cargo.toml` have versions
- [ ] No modifications to generated `apis/` or `models/` files (unless via script)

---

## Quick Reference

```bash
# Diagnose
cargo build 2>&1 | tee errors.txt
grep "error\[E" errors.txt | wc -l

# Fix dependencies
echo 'http = "^1.0"' >> Cargo.toml

# Fix warnings
cargo clippy --fix --allow-dirty
cargo fmt

# Test
cargo test

# Final check
cargo clean
cargo build
cargo clippy -- -D warnings
cargo test
```

---

## Getting Help

### Error Code Lookup

```bash
# Explain error code
rustc --explain E0308
```

### Dependency Information

```bash
# See dependency tree
cargo tree

# See feature flags
cargo tree -e features

# Update specific dep
cargo update -p reqwest
```

### Community Resources

- Rust error codes: https://doc.rust-lang.org/error-index.html
- Clippy lints: https://rust-lang.github.io/rust-clippy/
- Common patterns: https://rust-lang-nursery.github.io/rust-cookbook/

---

**Related Skills:**
- `sdk-gen` - Generate base SDK
- `sdk-wrapper` - Create client wrapper

**Additional Resources:**
- See `references/rust-error-codes.md` for detailed error explanations
