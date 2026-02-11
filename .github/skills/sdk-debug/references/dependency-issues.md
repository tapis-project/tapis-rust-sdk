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
