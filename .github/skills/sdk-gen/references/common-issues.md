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

## 6) Multiple services generate empty enum defaults

Symptom:
- `expected identifier, found '}'` in generated model files

Observed files:
- `tapis-actors/src/models/delete_actor_200_response.rs`
- `tapis-authenticator/src/models/delete_client_200_response.rs`
- `tapis-tenants/src/models/delete_site_200_response.rs`

Fix pattern:
- Replace `Option<Option<Result>>` with `Option<Option<serde_json::Value>>`
- Remove empty `Result` enum + broken `Default` impl

Quick detector:
```bash
rg -n 'Self::\\s*$' tapis-*/src/models
```

## 7) Invalid generated serde_json type path in some APIs

Symptom:
- `could not find serde_json in models`

Observed in:
- `tapis-meta/src/apis/collection_api.rs`
- `tapis-streams/src/apis/measurements_api.rs`

Fix:
- Replace `models::serde_json::Value` with `serde_json::Value`
- Apply same fix in wrapper signatures if wrappers mirror those generated signatures

## 8) Files range header model missing Display implementation

Symptom:
- `HeaderByteRange doesn't implement std::fmt::Display`
- Triggered by `.to_string()` usage in generated API request building

Observed in:
- `tapis-files/src/models/header_byte_range.rs`

Fix:
- Add `impl std::fmt::Display for HeaderByteRange` to format as `bytes=<min>-<max>`

## 9) Regeneration resets crate versions

Symptom:
- Service crates revert to `1.0.0` after `openapi-generator`

Fix:
- Always run version sync as final step:
```bash
bash .github/skills/sdk-parent/scripts/bump_minor_version.sh
```
