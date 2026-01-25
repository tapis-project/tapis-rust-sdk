# OpenAPI Generator Configuration Reference

Detailed configuration options for generating TAPIS Rust SDKs.

## Basic Command Structure

```bash
openapi-generator-cli generate \
  -i <spec-file> \
  -g rust \
  -o <output-dir> \
  --package-name <package-name> \
  --additional-properties <properties>
```

## Essential Properties

### Package Name
```bash
--package-name tapis_pods
```
- Use lowercase with underscores
- Must be valid Rust crate name
- Becomes `Cargo.toml` package name

### Library Name
```bash
--library reqwest
```
- `reqwest` (recommended) - Async HTTP client
- `hyper` - Lower-level HTTP library
- `ureq` - Synchronous HTTP client

## Additional Properties

### Generated Code Options

```bash
--additional-properties=\
packageVersion=1.0.0,\
supportAsync=true,\
useSingleRequestParameter=true,\
preferUnsignedInt=true,\
avoidBoxedModels=false
```

#### Key Properties

| Property | Default | Description |
|----------|---------|-------------|
| `packageVersion` | `1.0.0` | Version in Cargo.toml |
| `supportAsync` | `true` | Generate async/await code |
| `useSingleRequestParameter` | `false` | Group params into struct |
| `preferUnsignedInt` | `false` | Use `u32` instead of `i32` |
| `avoidBoxedModels` | `false` | Avoid `Box<T>` for large types |

### API Documentation

```bash
--additional-properties=\
hideGenerationTimestamp=true,\
withXml=false,\
withAWSV4Signature=false
```

### Serialization Options

```bash
--additional-properties=\
enumNameSuffix=,\
supportMiddleware=false
```

## Custom Templates

### Using Custom Templates

```bash
-t /path/to/custom/templates
```

Create custom templates to:
- Modify generated code structure
- Add custom imports
- Change documentation format
- Customize error handling

### Template Structure

```
custom-templates/
├── api.mustache           # API module template
├── model.mustache         # Model struct template
├── cargo.mustache         # Cargo.toml template
└── configuration.mustache # Config template
```

## Global Options

### Configuration File

Create `openapitools.json`:

```json
{
  "generator-cli": {
    "version": "7.2.0",
    "generators": {
      "rust": {
        "generatorName": "rust",
        "library": "reqwest",
        "additionalProperties": {
          "packageVersion": "1.0.0",
          "supportAsync": true
        }
      }
    }
  }
}
```

Use with:
```bash
openapi-generator-cli generate -g rust -i spec.yaml
```

### Ignore File

Create `.openapi-generator-ignore`:

```
# Don't overwrite these files
README.md
.gitignore
examples/

# Don't generate these
docs/README.md
```

## TAPIS-Specific Configuration

### Recommended Settings

```bash
openapi-generator-cli generate \
  -i tapis-pods-api.yaml \
  -g rust \
  -o ./tapis-pods \
  --package-name tapis_pods \
  --library reqwest \
  --additional-properties=\
packageVersion=1.0.0,\
supportAsync=true,\
preferUnsignedInt=false,\
avoidBoxedModels=false
```

### Why These Settings?

- **`reqwest`:** TAPIS APIs are HTTP/REST, reqwest is best fit
- **`supportAsync=true`:** TAPIS operations are I/O-bound, async is essential
- **`preferUnsignedInt=false`:** TAPIS specs use signed integers
- **`avoidBoxedModels=false`:** Some TAPIS models are large, boxing helps

## Validation

### Pre-Generation Validation

```bash
# Validate spec before generation
openapi-generator-cli validate -i tapis-pods-api.yaml
```

### Post-Generation Validation

```bash
# Check generated code compiles
cd output-dir
cargo build
cargo test
cargo clippy
```

## Troubleshooting

### Issue: Wrong Dependency Versions

**Problem:** Generated `Cargo.toml` has outdated dependencies

**Solution:** Use `packageVersion` and update dependencies manually:

```toml
[dependencies]
reqwest = { version = "^0.12", features = ["json", "multipart"] }
serde = { version = "^1.0", features = ["derive"] }
serde_json = "^1.0"
```

### Issue: Too Much Boilerplate

**Problem:** Generated code has unnecessary complexity

**Solution:** Use custom templates or post-process with scripts

### Issue: Missing Features

**Problem:** Multipart upload support not generated

**Solution:** Ensure spec includes `multipart/form-data` and add feature flag:

```toml
[dependencies.reqwest]
version = "^0.12"
features = ["json", "multipart", "stream"]
```

## Advanced: Multi-Service Generation

Generate multiple TAPIS SDKs in one workspace:

```bash
# Generate multiple services
for service in pods files systems apps; do
  openapi-generator-cli generate \
    -i tapis-${service}-api.yaml \
    -g rust \
    -o ./tapis-${service} \
    --package-name tapis_${service} \
    --library reqwest
done
```

Create workspace `Cargo.toml`:

```toml
[workspace]
members = [
    "tapis-pods",
    "tapis-files",
    "tapis-systems",
    "tapis-apps",
]
resolver = "2"
```

## Resources

- [Generator Options](https://openapi-generator.tech/docs/generators/rust/)
- [Mustache Templates](https://openapi-generator.tech/docs/templating/)
- [Configuration File](https://openapi-generator.tech/docs/configuration/)
