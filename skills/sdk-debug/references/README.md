# SDK Debug Reference Materials

Extended troubleshooting guides for TAPIS Rust SDK debugging.

## Contents

- [Rust Error Codes](./rust-error-codes.md) - Detailed explanations of common Rust errors
- [Clippy Lints](./clippy-lints.md) - Complete guide to fixing clippy warnings
- [Dependency Troubleshooting](./dependency-issues.md) - Resolving dependency conflicts

## Quick Diagnostics

### Get Error Summary

```bash
# Count errors by type
cargo build 2>&1 | grep "error\[E" | sed 's/.*error\[E\([0-9]*\)\].*/E\1/' | sort | uniq -c | sort -rn
```

### Get Warning Summary

```bash
# Count warnings by type
cargo build 2>&1 | grep "warning:" | sed 's/warning: \([^,]*\).*/\1/' | sort | uniq -c | sort -rn
```

### Check Dependency Tree

```bash
# See what's pulling in conflicting deps
cargo tree -i reqwest
cargo tree -d  # Show duplicates
```

## Emergency Commands

```bash
# Nuclear option: complete reset
cargo clean
rm -rf target/
rm Cargo.lock

# Regenerate lock file
cargo generate-lockfile

# Try build again
cargo build
```

## Related Documentation

- Main skill: [../SKILL.md](../SKILL.md)
- Rust error index: https://doc.rust-lang.org/error-index.html
- Example implementation: `../../tapis-pods/src/`
