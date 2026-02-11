#!/usr/bin/env bash
set -euo pipefail

usage() {
    cat <<'USAGE'
Usage:
  bump_minor_version.sh [--dry-run]

Behavior:
  - Reads root package version from Cargo.toml
  - Computes next minor version: MAJOR.(MINOR+1).0
  - Updates [package] version in root + all workspace member Cargo.toml files
  - Updates inline path dependency version fields in those manifests

Examples:
  .github/skills/sdk-parent/scripts/bump_minor_version.sh
  .github/skills/sdk-parent/scripts/bump_minor_version.sh --dry-run
USAGE
}

dry_run=0
if [[ "${1:-}" == "--dry-run" ]]; then
    dry_run=1
    shift
fi

if [[ $# -ne 0 ]]; then
    usage
    exit 1
fi

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repo_root="$(cd "${script_dir}/../../../.." && pwd)"
root_manifest="${repo_root}/Cargo.toml"

if [[ ! -f "${root_manifest}" ]]; then
    echo "Error: root Cargo.toml not found at ${root_manifest}" >&2
    exit 1
fi

if ! command -v cargo >/dev/null 2>&1; then
    echo "Error: cargo is required but not found in PATH." >&2
    exit 1
fi

if ! command -v jq >/dev/null 2>&1; then
    echo "Error: jq is required but not found in PATH." >&2
    exit 1
fi

current_version="$(
    awk '
        BEGIN { in_package = 0 }
        /^\[package\][[:space:]]*$/ { in_package = 1; next }
        in_package && /^\[[^]]+\]/ { in_package = 0 }
        in_package && /^version[[:space:]]*=/ {
            line = $0
            sub(/^[^"]*"/, "", line)
            sub(/".*$/, "", line)
            print line
            exit
        }
    ' "${root_manifest}"
)"

if [[ -z "${current_version}" ]]; then
    echo "Error: could not parse root package version from ${root_manifest}" >&2
    exit 1
fi

if [[ ! "${current_version}" =~ ^([0-9]+)\.([0-9]+)\.([0-9]+)$ ]]; then
    echo "Error: current version '${current_version}' is not MAJOR.MINOR.PATCH." >&2
    exit 1
fi

major="${BASH_REMATCH[1]}"
minor="${BASH_REMATCH[2]}"
new_version="${major}.$((minor + 1)).0"

manifests=()
while IFS= read -r manifest; do
    manifests+=("${manifest}")
done < <(
    cargo metadata \
        --manifest-path "${root_manifest}" \
        --no-deps \
        --format-version 1 |
        jq -r '.packages[].manifest_path' |
        sort -u
)

if [[ ${#manifests[@]} -eq 0 ]]; then
    echo "Error: no workspace manifests found via cargo metadata." >&2
    exit 1
fi

echo "Current version: ${current_version}"
echo "New version:     ${new_version}"
echo "Manifests:"
for manifest in "${manifests[@]}"; do
    echo "  - ${manifest}"
done

if [[ ${dry_run} -eq 1 ]]; then
    echo "Dry run complete. No files changed."
    exit 0
fi

update_package_version() {
    local manifest="$1"
    local tmp_file
    tmp_file="$(mktemp)"

    awk -v version="${new_version}" '
        BEGIN {
            in_package = 0
            package_version_updated = 0
        }
        /^\[package\][[:space:]]*$/ {
            in_package = 1
            print
            next
        }
        in_package && /^\[[^]]+\]/ {
            in_package = 0
        }
        in_package && !package_version_updated && /^version[[:space:]]*=/ {
            sub(/"[^"]*"/, "\"" version "\"")
            package_version_updated = 1
        }
        {
            print
        }
        END {
            if (!package_version_updated) {
                exit 2
            }
        }
    ' "${manifest}" >"${tmp_file}" || {
        rm -f "${tmp_file}"
        echo "Error: failed to update package version in ${manifest}" >&2
        exit 1
    }

    mv "${tmp_file}" "${manifest}"
}

update_inline_path_dependency_versions() {
    local manifest="$1"
    local tmp_file
    tmp_file="$(mktemp)"

    awk -v version="${new_version}" '
        {
            if ($0 ~ /path[[:space:]]*=[[:space:]]*"/ && $0 ~ /version[[:space:]]*=[[:space:]]*"/) {
                gsub(/version[[:space:]]*=[[:space:]]*"[^"]*"/, "version = \"" version "\"")
            }
            print
        }
    ' "${manifest}" >"${tmp_file}"

    mv "${tmp_file}" "${manifest}"
}

for manifest in "${manifests[@]}"; do
    update_package_version "${manifest}"
done

for manifest in "${manifests[@]}"; do
    update_inline_path_dependency_versions "${manifest}"
done

echo "Version bump complete: ${current_version} -> ${new_version}"
