# SDK Generation Scripts

Automation scripts for generating TAPIS Rust SDKs from OpenAPI specifications.

## Available Scripts

### [generate_rust_sdk.sh](./generate_rust_sdk.sh)

Automated SDK generation with environment-based branch switching and spec URL lookup.

**Usage:**
```bash
./generate_rust_sdk.sh <env> <service>
```

**Arguments:**
- `env` - Environment: `prod`, `staging`, or `dev`
- `service` - TAPIS service name (e.g., pods, files, systems, actors)

**Examples:**
```bash
# Generate pods SDK for production (uses main branch)
./generate_rust_sdk.sh prod pods

# Generate files SDK for staging (uses staging branch)
./generate_rust_sdk.sh staging files

# Generate systems SDK for dev (uses dev branch)
./generate_rust_sdk.sh dev systems
```

**What the script does:**
1. 🔀 Switches to the appropriate git branch (prod→main, staging→staging, dev→dev)
2. 🔍 Looks up OpenAPI spec URL from `references/OpenAPI_specs.json`
3. ⬇️  Downloads the spec file from GitHub
4. 🔧 Generates SDK in `<repo_root>/tapis-<service>/`
5. 📦 Uses package name `tapis_<service_name>`
6. ✅ Validates all inputs and prerequisites

**Features:**
- ✅ Automatic git branch management aligned with environments
- ✅ Validates uncommitted changes before branch switching
- ✅ Downloads specs from OpenAPI_specs.json registry
- ✅ Finds repository root automatically (where LICENSE file exists)
- ✅ Generates SDK with proper reqwest configuration
- ✅ Colored output for easy reading
- ✅ Error handling and validation
- ✅ Follows naming convention: `tapis-<service>` directories, `tapis_<service>` packages

**Requirements:**
- `openapi-generator` installed (via npm or brew)
- `jq` installed (for JSON parsing)
- `curl` installed (for downloading specs)
- Clean git working directory (no uncommitted changes)

**Available TAPIS Services:**
actors, authenticator, meta, files, sk, streams, systems, tenants, tokens, pgrest, pods, jobs, apps, workflows, notifications, globus-proxy

**Available Environments:**
- `prod` - Production environment (→ main branch)
- `staging` - Staging environment (→ staging branch)
- `dev` - Development environment (→ dev branch)

**Available Environments:**
- `prod` - Production environment (→ main branch)
- `staging` - Staging environment (→ staging branch)
- `dev` - Development environment (→ dev branch)

## Output Structure

Generated SDKs are placed in the repository root:

```
tapis-rust-sdk/
├── LICENSE              ← Repository root marker
├── skills/
│   └── sdk-gen/
│       └── scripts/     ← You are here
├── tapis-pods/          ← Example existing SDK
├── tapis-files/         ← Your generated SDK
├── tapis-systems/       ← Your generated SDK
└── tapis-<service>/     ← Generated SDK location
```

Each directory is a complete Rust project ready for wrapper creation.

## Installation Guide

**Install openapi-generator:**
```bash
# Via npm (recommended)
npm install -g @openapitools/openapi-generator-cli

# Via Homebrew (macOS)
brew install openapi-generator
```

**Install jq:**
```bash
# Via Homebrew (macOS)
brew install jq

# Via apt (Ubuntu/Debian)
sudo apt-get install jq
```

## Usage from Skill

When following the `sdk-gen` skill:

```bash
cd skills/sdk-gen/scripts

# Generate SDK for desired environment
./generate_rust_sdk.sh prod pods
./generate_rust_sdk.sh staging files
./generate_rust_sdk.sh dev systems
```

The script handles:
- Git branch management
- OpenAPI spec URL lookup
- Spec file download
- SDK generation with proper configuration
- Cleanup of temporary files

## Troubleshooting

**Issue: Uncommitted changes**
```
Error: You have uncommitted changes. Please commit or stash them first.
```
Solution: Commit or stash your changes before running the script.

**Issue: Branch doesn't exist**
```
Error: Failed to switch to branch staging
```
Solution: Create the branch first:
```bash
git checkout -b staging
```

**Issue: Service not found**
```
Error: No spec URL found for service 'xyz' in environment 'prod'
```
Solution: Check available services by running the script without arguments or verify the service name in `references/OpenAPI_specs.json`.
