# SDK Generation Scripts

Automation scripts for generating TAPIS Rust SDKs from OpenAPI specifications.

## Available Scripts

### [generate_rust_sdk.sh](./generate_rust_sdk.sh)

Automated SDK generation with validation and colored output.

**Usage (from OpenAPI_specs.json):**
```bash
./generate_rust_sdk.sh --from-json <environment> <component>
```

**Usage (from local/URL spec file):**
```bash
./generate_rust_sdk.sh <openapi-spec> <package-name> [output-dir]
```

**Examples:**
```bSupports OpenAPI_specs.json for TAPIS components (all environments)
- ✅ Validates inputs and checks prerequisites
- ✅ Generates SDK with proper configuration
- ✅ Colored output for easy reading
- ✅ Error handling and validation
- ✅ Creates output directory automatically in workspace root
- ✅ Follows naming convention: `tapis-<component>` for directories

**Requirements:**
- openapi-generator-cli installed
- jq or python3 (for --from-json mode)
- Valid OpenAPI spec file (YAML or JSON) OR environment/component name
- Valid Rust package name (lowercase with underscores)

**Available TAPIS Components:**
actors, authenticator, meta, files, sk, streams, systems, tenants, tokens, pgrest, pods, jobs, apps, workflows, notifications, globus-proxy
Generate TAPIS component SDK (recommended)
cd skills/sdk-gen/scripts
chmod +x generate_rust_sdk.sh
./generate_rust_sdk.sh --from-json prod pods

# Or from custom spec
./generate_rust_sdk.sh /path/to/spec.yaml tapis_component ../../tapis-component
```

The script will:
1. Download the spec (if using --from-json)
2. Generate the SDK in `../../tapis-<component>/` directory
3. Use package name `tapis_<component>`
4. Set up proper configuration

## Output Structure

Generated SDKs are placed in the workspace root:

```
tapis-rust-sdk/
├── skills/
├── tapis-pods/          ← Example
├── tapis-files/         ← Generated
├── tapis-systems/       ← Generated
└── tapis-<component>/   ← Your new SDK
```

Each directory is a complete Rust project ready for wrapper creation
- ✅ Validates inputs and checks prerequisites
- ✅ Generates SDK with proper configuration
- ✅ Colored output for easy reading
- ✅ Error handling and validation
- ✅ Creates output directory automatically

**Requirements:**
- openapi-generator-cli installed
- Valid OpenAPI spec file (YAML or JSON)
- Valid Rust package name (lowercase with underscores)

## Usage from Skill

When following the `sdk-gen` skill, you can use this script instead of running `openapi-generator-cli` manually:

```bash
# Instead of manual generation
cd skills/sdk-gen/scripts
chmod +x generate_rust_sdk.sh
./generate_rust_sdk.sh ../../../../path/to/spec.yaml tapis_service
```

The script will handle all the configuration and validation automatically.
