# SDK Generation Best Practices

## Keep Generated vs Manual Boundaries Clear

Generated (avoid edits):
- `src/apis/*`
- `src/models/*`

Manual integration (safe):
- `src/client.rs`
- `src/lib.rs`
- crate `Cargo.toml`
- parent crate wiring

## Regenerate, Then Integrate

Preferred sequence:
1. Run generation script.
2. Repair/update wrapper coverage.
3. Ensure parent crate mappings/re-exports.
4. Build workspace all targets.
5. Run repo-wide version bump script as the final step.

## Verify Method Parity

After every regeneration:

```bash
rg '^pub async fn ' src/apis/*_api.rs | wc -l
rg '^[[:space:]]*pub async fn ' src/client.rs | wc -l
```

## Run Targeted Post-Generation Scans

Before final build, run:

```bash
# broken empty enums
rg -n 'Self::\\s*$' tapis-*/src/models

# invalid serde_json path emitted by some specs
rg -n 'models::serde_json::Value' tapis-*/src/apis tapis-*/src/client.rs
```

If either scan returns matches, patch before moving forward.

## Keep Fixes Minimal and Repeatable

When generator bugs appear, apply smallest patch possible and document it in skill references so future runs are deterministic.
