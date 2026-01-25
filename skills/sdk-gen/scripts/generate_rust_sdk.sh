#!/bin/bash
#
# Rust SDK Generator for TAPIS Services
#
# This script generates a Rust SDK from an OpenAPI specification using openapi-generator.
# It supports both YAML and JSON formats and applies consistent configuration.
#
# Usage:
#   ./generate_rust_sdk.sh <spec_file> <output_dir> [package_name]
#
# Examples:
#   ./generate_rust_sdk.sh openapi.yaml ../tapis-pods
#   ./generate_rust_sdk.sh openapi.json ../tapis-files tapis_files
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

# Check if openapi-generator is installed
check_openapi_generator() {
    if ! command -v openapi-generator &> /dev/null; then
        print_msg $RED "❌ Error: openapi-generator is not installed"
        echo ""
        echo "Installation options:"
        echo "  1. Via npm (recommended):"
        echo "     npm install @openapitools/openapi-generator-cli -g"
        echo ""
        echo "  2. Via Homebrew (macOS):"
        echo "     brew install openapi-generator"
        echo ""
        echo "  3. Manual download:"
        echo "     https://openapi-generator.tech/docs/installation/"
        exit 1
    fi
}

# Validate input parameters
if [ $# -lt 2 ]; then
    print_msg $RED "Usage: $0 <spec_file> <output_dir> [package_name]"
    echo ""
    echo "Arguments:"
    echo "  spec_file    - Path to OpenAPI spec (YAML or JSON)"
    echo "  output_dir   - Directory where SDK will be generated"
    echo "  package_name - Optional: Rust package name (default: derived from spec)"
    exit 1
fi

SPEC_FILE="$1"
OUTPUT_DIR="$2"
PACKAGE_NAME="${3:-}"

# Validate spec file exists
if [ ! -f "$SPEC_FILE" ]; then
    print_msg $RED "❌ Error: Spec file not found: $SPEC_FILE"
    exit 1
fi

# Determine package name from spec file if not provided
if [ -z "$PACKAGE_NAME" ]; then
    PACKAGE_NAME=$(basename "$SPEC_FILE" | sed 's/\.[^.]*$//' | tr '[:upper:]' '[:lower:]' | tr '-' '_')
    print_msg $YELLOW "📦 Package name not provided, using: $PACKAGE_NAME"
fi

print_msg $BLUE "════════════════════════════════════════════════════════════"
print_msg $BLUE "  Rust SDK Generator for TAPIS Services"
print_msg $BLUE "════════════════════════════════════════════════════════════"
echo ""
print_msg $GREEN "📄 OpenAPI Spec: $SPEC_FILE"
print_msg $GREEN "📁 Output Directory: $OUTPUT_DIR"
print_msg $GREEN "📦 Package Name: $PACKAGE_NAME"
echo ""

# Check dependencies
check_openapi_generator

# Create output directory if it doesn't exist
mkdir -p "$OUTPUT_DIR"

print_msg $YELLOW "🔧 Generating Rust SDK..."
echo ""

# Generate the SDK
openapi-generator generate \
-i "$SPEC_FILE" \
-g rust \
-o "$OUTPUT_DIR" \
--package-name "$PACKAGE_NAME" \
--additional-properties=packageVersion=1.0.0 \
--additional-properties=useSingleRequestParameter=false \
--additional-properties=supportAsync=true \
--skip-validate-spec

if [ $? -eq 0 ]; then
    print_msg $GREEN "✓ SDK generation completed successfully!"
else
    print_msg $RED "❌ SDK generation failed"
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
