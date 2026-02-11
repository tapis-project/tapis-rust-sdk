# Common SDK Generation Issues

## 1) Spec download fails

Symptom:
- `curl: Could not resolve host` or HTTP 404

Checks:
- Verify internet/network access
- Verify URL in `OpenAPI_specs.json`
- Verify selected environment/service key exists

## 2) Empty enum generated

Symptom:
- `expected identifier, found '}'` in a generated model enum

Fix pattern:
- Replace the problematic enum field type with `serde_json::Value`
- Remove empty enum + `Default` impl

## 3) Multipart upload compile failure

Symptom:
- `no method named file found for struct Form`

Fix:
- Enable reqwest `stream` feature in crate `Cargo.toml`

## 4) Wrapper disappears after regeneration

Symptom:
- `src/lib.rs` no longer exports client wrapper

Fix:
- Re-add `pub mod client;`
- Re-add `pub use client::Tapis<Service>;`

## 5) Parent crate dependency mismatch

Symptom:
- `no matching package found` for service crate

Fix:
- Use dependency `package = "..."` mapping when dependency key differs from generated package name.
