# TAPIS API Specifications

Where to find OpenAPI specifications for TAPIS services.

## Quick Access: OpenAPI_specs.json

**The easiest way:** Use the provided [OpenAPI_specs.json](./OpenAPI_specs.json) file which contains URLs for all TAPIS components across all environments.

**Structure:**
```json
{
  "prod": {
    "pods": "https://raw.githubusercontent.com/.../openapi_v3-pods.yml",
    "files": "https://raw.githubusercontent.com/.../FilesAPI.yaml",
    ...
  },
  "staging": { ... },
  "dev": { ... }
}
```

**Usage with generation script:**
```bash
cd ../scripts
./generate_rust_sdk.sh --from-json prod pods
./generate_rust_sdk.sh --from-json staging files
```

The script will automatically fetch the correct spec URL and generate the SDK.

---

## Official Sources

### Live Documentation
- **URL:** https://tapis-project.github.io/live-docs/
- **Format:** Interactive Swagger UI
- **Download:** Look for "openapi.json" or "openapi.yaml" links

### GitHub Repository
- **URL:** https://github.com/tapis-project
- **Location:** Usually in `src/` or `docs/` directories
- **Files:** Look for `openapi.yaml`, `swagger.yaml`, or `api-spec.yaml`

## Available TAPIS Services

### Pods Service
- **Purpose:** Container orchestration and management
- **Spec URL:** Check https://github.com/tapis-project/pods_service
- **Package Name:** `tapis_pods`

### Files Service
- **Purpose:** File management and transfers
- **Spec URL:** Check https://github.com/tapis-project/tapis-files
- **Package Name:** `tapis_files`

### Systems Service
- **Purpose:** System definitions and credentials
- **Spec URL:** Check https://github.com/tapis-project/tapis-systems
- **Package Name:** `tapis_systems`

### Apps Service
- **Purpose:** Application definitions and execution
- **Spec URL:** Check https://github.com/tapis-project/tapis-apps
- **Package Name:** `tapis_apps`

### Jobs Service
- **Purpose:** Job submission and monitoring
- **Spec URL:** Check https://github.com/tapis-project/tapis-jobs
- **Package Name:** `tapis_jobs`

### Authenticator Service
- **Purpose:** Authentication and token management
- **Spec URL:** Check https://github.com/tapis-project/authenticator
- **Package Name:** `tapis_authenticator`

### Security Kernels Service
- **Purpose:** Security policies and permissions
- **Spec URL:** Check https://github.com/tapis-project/tapis-security
- **Package Name:** `tapis_security`

### Streams Service
- **Purpose:** Real-time data streaming
- **Spec URL:** Check https://github.com/tapis-project/streams-api
- **Package Name:** `tapis_streams`

### Actors Service
- **Purpose:** Function-as-a-Service execution
- **Spec URL:** Check https://github.com/tapis-project/abaco
- **Package Name:** `tapis_actors`

### Meta Service
- **Purpose:** Metadata management
- **Spec URL:** Check https://github.com/tapis-project/tapis-meta
- **Package Name:** `tapis_meta`

## Downloading Specifications

### From Live Docs

```bash
# Using curl
curl -o tapis-pods-api.yaml \
  https://tapis-project.github.io/live-docs/pods/openapi.yaml

# Using wget
wget -O tapis-pods-api.yaml \
  https://tapis-project.github.io/live-docs/pods/openapi.yaml
```

### From GitHub

```bash
# Clone repository
git clone https://github.com/tapis-project/pods_service
cd pods_service

# Find spec file
find . -name "*.yaml" -o -name "*.json" | grep -i openapi
```

### From Running Service

If you have access to a TAPIS deployment:

```bash
# Get spec from service endpoint
curl https://api.tapis.io/v3/pods/openapi.json > tapis-pods-api.json
```

## Validation

Always validate downloaded specs:

```bash
# Install validator
npm install -g @apidevtools/swagger-cli

# Validate spec
swagger-cli validate tapis-pods-api.yaml
```

Or use OpenAPI Generator:

```bash
openapi-generator-cli validate -i tapis-pods-api.yaml
```

## Spec Quality

### Check Before Generation

```bash
# Check for required fields
grep -E "(operationId|parameters|responses)" tapis-pods-api.yaml | wc -l

# Check OpenAPI version
head -10 tapis-pods-api.yaml | grep openapi
```

### Common Issues

#### Missing operationId
Some specs lack `operationId`, causing ugly function names:

```yaml
# Bad (no operationId)
paths:
  /pods:
    get:
      summary: List pods
      # Missing operationId

# Good (with operationId)
paths:
  /pods:
    get:
      operationId: listPods
      summary: List pods
```

#### Inconsistent Tags
Tags should be consistent for proper API grouping:

```yaml
# Good
paths:
  /pods:
    get:
      tags: [Pods]
  /pods/{pod_id}:
    get:
      tags: [Pods]
```

## Version Management

Track which spec version generated your SDK:

```bash
# Add metadata to README
echo "Generated from: tapis-pods-api.yaml" >> README.md
echo "Spec version: $(grep 'version:' tapis-pods-api.yaml | head -1)" >> README.md
echo "Generated on: $(date)" >> README.md
echo "Generator version: $(openapi-generator version)" >> README.md
```

## Local Modifications

If you need to modify the spec before generation:

### Create Modified Version

```bash
# Copy original
cp tapis-pods-api.yaml tapis-pods-api-modified.yaml

# Edit as needed
vim tapis-pods-api-modified.yaml

# Generate from modified version
openapi-generator-cli generate -i tapis-pods-api-modified.yaml -g rust
```

### Document Changes

```yaml
# Add to spec header
info:
  title: TAPIS Pods API
  version: 1.0.0
  description: |
    Modified for Rust SDK generation
    Changes:
    - Added missing operationIds
    - Fixed inconsistent tags
    - Added examples
```

## Resources

- **TAPIS Documentation:** https://tapis.readthedocs.io/
- **GitHub Organization:** https://github.com/tapis-project
- **Live Docs:** https://tapis-project.github.io/live-docs/
- **API Reference:** https://tapis.readthedocs.io/en/latest/technical/index.html
