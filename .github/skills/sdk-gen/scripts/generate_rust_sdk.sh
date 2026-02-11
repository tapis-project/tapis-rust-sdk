#!/bin/bash
#
# Rust SDK Generator for TAPIS Services
#
# This script generates a Rust SDK from an OpenAPI specification using openapi-generator.
# It supports both direct spec file input and environment-based generation from OpenAPI_specs.json.
#
# Usage:
#   ./generate_rust_sdk.sh [--no-branch-switch] <env> <service>
#
# Arguments:
#   --no-branch-switch - Optional. Skip git checkout and generate from current branch.
#   env                - Environment: prod, staging, or dev
#   service            - TAPIS service name (e.g., pods, files, systems, actors, etc.)
#
# Examples:
#   ./generate_rust_sdk.sh prod pods
#   ./generate_rust_sdk.sh staging files
#   ./generate_rust_sdk.sh dev systems
#
# The script will:
#   1. Switch to the appropriate git branch (prod→main, staging→staging, dev→dev)
#   2. Look up the OpenAPI spec URL from references/OpenAPI_specs.json
#   3. Download the spec file
#   4. Generate the SDK in <repo_root>/tapis-<service>
#

set -e  # Exit on error

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Print colored message
print_msg() {
    local color=$1
    shift
    echo -e "${color}$@${NC}"
}

# Check if a command exists
check_command() {
    local cmd=$1
    local install_msg=$2
    if ! command -v "$cmd" &> /dev/null; then
        print_msg $RED "❌ Error: $cmd is not installed"
        echo ""
        echo "$install_msg"
        exit 1
    fi
}

# Check if openapi-generator is installed
check_openapi_generator() {
    check_command "openapi-generator" \
    "Installation options:
  1. Via npm (recommended):
     npm install @openapitools/openapi-generator-cli -g

  2. Via Homebrew (macOS):
     brew install openapi-generator

  3. Manual download:
    https://openapi-generator.tech/docs/installation/"
}

# Check if jq is installed
check_jq() {
    check_command "jq" \
    "Installation options:
  1. Via Homebrew (macOS):
     brew install jq

  2. Via apt (Ubuntu/Debian):
     sudo apt-get install jq

  3. Via yum (RHEL/CentOS):
    sudo yum install jq"
}

# Find the git repository root (where LICENSE file exists)
find_repo_root() {
    local current_dir="$PWD"
    while [ "$current_dir" != "/" ]; do
        if [ -f "$current_dir/LICENSE" ]; then
            echo "$current_dir"
            return 0
        fi
        current_dir=$(dirname "$current_dir")
    done
    print_msg $RED "❌ Error: Could not find repository root (LICENSE file not found)"
    exit 1
}

# Map environment to git branch
get_branch_for_env() {
    local env=$1
    case "$env" in
        prod)
            echo "main"
        ;;
        staging)
            echo "staging"
        ;;
        dev)
            echo "dev"
        ;;
        *)
            print_msg $RED "❌ Error: Invalid environment: $env"
            echo "Valid environments: prod, staging, dev"
            exit 1
        ;;
    esac
}

# Optional branch-switch control.
# Can be set via flag or environment variable TAPIS_SDK_NO_BRANCH_SWITCH=1
NO_BRANCH_SWITCH="${TAPIS_SDK_NO_BRANCH_SWITCH:-0}"
if [ "${1:-}" == "--no-branch-switch" ]; then
    NO_BRANCH_SWITCH=1
    shift
fi

# Validate input parameters
if [ $# -ne 2 ]; then
    print_msg $RED "Usage: $0 [--no-branch-switch] <env> <service>"
    echo ""
    echo "Arguments:"
    echo "  env     - Environment: prod, staging, or dev"
    echo "  service - TAPIS service name (e.g., pods, files, systems, actors)"
    echo ""
    echo "Examples:"
    echo "  $0 prod pods"
    echo "  $0 staging files"
    echo "  $0 dev systems"
    echo ""
    echo "Available services:"
    echo "  actors, authenticator, meta, files, sk, streams, systems,"
    echo "  tenants, tokens, pgrest, pods, jobs, apps, workflows,"
    echo "  notifications, globus-proxy"
    exit 1
fi

ENV="$1"
SERVICE="$2"

# Find repository root
REPO_ROOT=$(find_repo_root)
OPENAPI_SPECS_FILE="$REPO_ROOT/.github/skills/sdk-gen/references/OpenAPI_specs.json"

print_msg $BLUE "════════════════════════════════════════════════════════════"
print_msg $BLUE "  Rust SDK Generator for TAPIS Services"
print_msg $BLUE "════════════════════════════════════════════════════════════"
echo ""
print_msg $GREEN "🌍 Environment: $ENV"
print_msg $GREEN "📦 Service: $SERVICE"
print_msg $GREEN "📂 Repository Root: $REPO_ROOT"
echo ""

# Check dependencies
check_openapi_generator
check_jq

# Validate OpenAPI_specs.json exists
if [ ! -f "$OPENAPI_SPECS_FILE" ]; then
    print_msg $RED "❌ Error: OpenAPI_specs.json not found at: $OPENAPI_SPECS_FILE"
    exit 1
fi

# Determine target branch
TARGET_BRANCH=$(get_branch_for_env "$ENV")
cd "$REPO_ROOT"
CURRENT_BRANCH=$(git rev-parse --abbrev-ref HEAD 2>/dev/null || echo "unknown")

if [ "$NO_BRANCH_SWITCH" = "1" ]; then
    print_msg $YELLOW "⏭️  Skipping branch switch (current: $CURRENT_BRANCH, target for env: $TARGET_BRANCH)"
else
    print_msg $YELLOW "🔀 Switching to branch: $TARGET_BRANCH"

    if [ "$CURRENT_BRANCH" != "$TARGET_BRANCH" ]; then
        print_msg $YELLOW "   Current branch: $CURRENT_BRANCH → $TARGET_BRANCH"

        # Check for uncommitted changes
        if ! git diff-index --quiet HEAD -- 2>/dev/null; then
            print_msg $RED "❌ Error: You have uncommitted changes. Please commit or stash them first."
            echo ""
            git status --short
            exit 1
        fi

        # Switch branch
        if ! git checkout "$TARGET_BRANCH"; then
            print_msg $RED "❌ Error: Failed to switch to branch $TARGET_BRANCH"
            print_msg $YELLOW "   Available branches:"
            git branch -a
            exit 1
        fi

        print_msg $GREEN "✓ Switched to branch: $TARGET_BRANCH"
    else
        print_msg $GREEN "✓ Already on branch: $TARGET_BRANCH"
    fi
fi

echo ""

# Look up OpenAPI spec URL from JSON file
print_msg $YELLOW "🔍 Looking up OpenAPI spec URL..."
SPEC_URL=$(jq -r ".${ENV}.\"${SERVICE}\"" "$OPENAPI_SPECS_FILE")

if [ "$SPEC_URL" == "null" ] || [ -z "$SPEC_URL" ]; then
    print_msg $RED "❌ Error: No spec URL found for service '$SERVICE' in environment '$ENV'"
    echo ""
    echo "Available services in $ENV environment:"
    jq -r ".${ENV} | keys[]" "$OPENAPI_SPECS_FILE" | sort
    exit 1
fi

print_msg $GREEN "✓ Found spec URL: $SPEC_URL"
echo ""

# Download the spec file to a temporary location
TEMP_SPEC="/tmp/openapi_${SERVICE}_${ENV}_$$.yaml"
print_msg $YELLOW "⬇️  Downloading OpenAPI spec..."

if ! curl -fsSL "$SPEC_URL" -o "$TEMP_SPEC"; then
    print_msg $RED "❌ Error: Failed to download spec from: $SPEC_URL"
    exit 1
fi

print_msg $GREEN "✓ Downloaded spec to: $TEMP_SPEC"
echo ""

# Set up output directory and package name
OUTPUT_DIR="$REPO_ROOT/tapis-${SERVICE}"
PACKAGE_NAME="tapis_$(echo $SERVICE | tr '-' '_')"

print_msg $GREEN "📁 Output Directory: $OUTPUT_DIR"
print_msg $GREEN "📦 Package Name: $PACKAGE_NAME"
echo ""

# Create output directory if it doesn't exist
mkdir -p "$OUTPUT_DIR"

print_msg $YELLOW "🔧 Generating Rust SDK..."
echo ""

# Generate the SDK
if openapi-generator generate \
-i "$TEMP_SPEC" \
-g rust \
-o "$OUTPUT_DIR" \
--package-name "$PACKAGE_NAME" \
--additional-properties=packageVersion=1.0.0 \
--additional-properties=useSingleRequestParameter=false \
--additional-properties=supportAsync=true \
--library=reqwest \
--skip-validate-spec; then
    
    print_msg $GREEN "✓ SDK generation completed successfully!"
    
    # Clean up temp file
    rm -f "$TEMP_SPEC"
else
    print_msg $RED "❌ SDK generation failed"
    rm -f "$TEMP_SPEC"
    exit 1
fi

echo ""
print_msg $BLUE "════════════════════════════════════════════════════════════"
print_msg $BLUE "  Next Steps"
print_msg $BLUE "════════════════════════════════════════════════════════════"
echo ""
echo "1. Review the generated SDK in: $OUTPUT_DIR"
echo ""
echo "2. Build the SDK:"
echo "   cd $OUTPUT_DIR"
echo "   cargo build"
echo ""
echo "3. Add a high-level client wrapper:"
echo "   See: WRAPPER_GUIDE.md for detailed instructions"
echo ""
echo "4. Create examples:"
echo "   mkdir examples"
echo "   # Create example files demonstrating usage"
echo ""
echo "5. Update documentation:"
echo "   # Add README.md with usage examples"
echo "   # Document authentication requirements"
echo ""
print_msg $GREEN "✓ Ready to build your Rust SDK!"
echo ""
