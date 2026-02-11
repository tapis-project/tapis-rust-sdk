# Dependency Troubleshooting

## Common cases

### reqwest multipart upload errors

If generated code uses `multipart::Form::file(...)`, ensure reqwest `stream` feature is enabled.

### http header types missing

If wrapper uses `HeaderMap`/`HeaderValue`, add:

```toml
http = "^1.0"
```

### Parent dependency resolution failure

If parent cannot find service package, map dependency key to generated package name using `package = "..."`.

### Empty enum compile failures in generated models

If a model has an empty enum with `Self::` default (parse error), replace the enum field type with `serde_json::Value` and remove the empty enum/default block.

### Invalid `models::serde_json::Value` path

If compile errors say `could not find serde_json in models`, replace `models::serde_json::Value` with `serde_json::Value` in affected generated files.

### Deprecated generated API usage warnings

Some wrappers call deprecated generated methods. If warnings must be removed without behavior changes, add `#![allow(deprecated)]` at crate root (`src/lib.rs`).
