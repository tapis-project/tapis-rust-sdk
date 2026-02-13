#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../../../.." && pwd)"
ROOT_MANIFEST="$REPO_ROOT/Cargo.toml"

PUBLISH_RETRIES="${PUBLISH_RETRIES:-3}"
PUBLISH_RETRY_DELAY="${PUBLISH_RETRY_DELAY:-20}"
INTER_CRATE_DELAY="${INTER_CRATE_DELAY:-60}"
SKIP_DELAY="${SKIP_DELAY:-2}"

LIST_ONLY=0
CONTINUE_ON_ERROR=0
EXTRA_ARGS=()
DRY_RUN_REQUESTED=0
for arg in "$@"; do
    if [[ "$arg" == "--list" ]]; then
        LIST_ONLY=1
        elif [[ "$arg" == "--continue-on-error" ]]; then
        CONTINUE_ON_ERROR=1
    else
        EXTRA_ARGS+=("$arg")
        if [[ "$arg" == "--dry-run" ]]; then
            DRY_RUN_REQUESTED=1
        fi
    fi
done

manifest_field() {
    local manifest_path="$1"
    local field_name="$2"
    awk -v key="$field_name" '
        /^\[package\]/ { in_pkg=1; next }
        in_pkg && /^\[/ { in_pkg=0 }
        in_pkg && $0 ~ "^[[:space:]]*" key "[[:space:]]*=" {
            if (match($0, /"[^"]+"/)) {
                value = substr($0, RSTART + 1, RLENGTH - 2)
                print value
                exit
            }
        }
    ' "$manifest_path"
}

check_version_published() {
    local package_name="$1"
    local package_version="$2"
    local api_url="https://crates.io/api/v1/crates/$package_name"
    local response
    
    if ! command -v curl &>/dev/null; then
        echo "curl not found, skipping version check" >&2
        return 1
    fi
    
    response=$(curl -sf "$api_url" 2>/dev/null || echo "")
    if [[ -z "$response" ]]; then
        return 1
    fi
    
    if echo "$response" | grep -q "\"num\":\"$package_version\""; then
        return 0
    fi
    
    return 1
}

publish_dir() {
    local crate_dir="$1"
    local crate_manifest="$crate_dir/Cargo.toml"
    local package_name
    local package_version
    local attempt
    local publish_log
    local publish_status
    
    if [[ ! -f "$crate_manifest" ]]; then
        echo "Missing manifest: $crate_manifest" >&2
        return 1
    fi
    
    package_name="$(manifest_field "$crate_manifest" "name")"
    package_version="$(manifest_field "$crate_manifest" "version")"
    
    if [[ "$LIST_ONLY" -eq 1 ]]; then
        echo "$package_name v$package_version ($crate_dir)"
        return 0
    fi
    
    if [[ "$DRY_RUN_REQUESTED" -eq 0 ]] && check_version_published "$package_name" "$package_version"; then
        echo "⏭️  Skipping $package_name v$package_version (already published)"
        return 10
    fi
    
    for attempt in $(seq 1 "$PUBLISH_RETRIES"); do
        echo "Publishing $package_name v$package_version (attempt $attempt/$PUBLISH_RETRIES)"
        publish_log="$(mktemp)"
        if [[ "${#EXTRA_ARGS[@]}" -gt 0 ]]; then
            if (cd "$crate_dir" && cargo publish --locked "${EXTRA_ARGS[@]}") >"$publish_log" 2>&1; then
                cat "$publish_log"
                rm -f "$publish_log"
                return 0
            else
                publish_status=$?
            fi
            elif (cd "$crate_dir" && cargo publish --locked) >"$publish_log" 2>&1; then
            cat "$publish_log"
            rm -f "$publish_log"
            return 0
        else
            publish_status=$?
        fi
        
        cat "$publish_log"
        
        if grep -Eqi \
        "crate was previously named|unknown or invalid license expression|status 40[0-9]|already exists|already uploaded|is already uploaded" \
        "$publish_log"; then
            echo "Non-retryable publish error for $package_name. Aborting retries." >&2
            rm -f "$publish_log"
            return "$publish_status"
        fi
        rm -f "$publish_log"
        
        if [[ "$attempt" -lt "$PUBLISH_RETRIES" ]]; then
            echo "Retrying $package_name in ${PUBLISH_RETRY_DELAY}s"
            sleep "$PUBLISH_RETRY_DELAY"
        fi
    done
    
    echo "Failed to publish $package_name after $PUBLISH_RETRIES attempt(s)" >&2
    return 1
}

if [[ "$LIST_ONLY" -eq 0 ]] && [[ "$DRY_RUN_REQUESTED" -eq 0 ]]; then
    if [[ -z "${CARGO_REGISTRY_TOKEN:-}" ]]; then
        echo "CARGO_REGISTRY_TOKEN is not set." >&2
        echo "Export it before publishing, for example:" >&2
        echo "  export CARGO_REGISTRY_TOKEN=<your_crates_io_token>" >&2
        exit 1
    fi
fi

if [[ ! -f "$ROOT_MANIFEST" ]]; then
    echo "Root manifest not found: $ROOT_MANIFEST" >&2
    exit 1
fi

WORKSPACE_MEMBERS=()
while IFS= read -r member; do
    [[ -n "$member" ]] && WORKSPACE_MEMBERS+=("$member")
    done < <(
    awk '
        /^\[workspace\]/ { in_ws=1; next }
        in_ws && /^\[/ { in_ws=0 }
        in_ws {
            while (match($0, /"[^"]+"/)) {
                member = substr($0, RSTART + 1, RLENGTH - 2)
                print member
                $0 = substr($0, RSTART + RLENGTH)
            }
        }
    ' "$ROOT_MANIFEST"
)

if [[ "${#WORKSPACE_MEMBERS[@]}" -eq 0 ]]; then
    echo "No workspace members found in $ROOT_MANIFEST" >&2
    exit 1
fi

echo "Publishing workspace members first (${#WORKSPACE_MEMBERS[@]} crates), then parent crate."

FAILED_MEMBERS=()
for idx in "${!WORKSPACE_MEMBERS[@]}"; do
    member="${WORKSPACE_MEMBERS[$idx]}"
    set +e
    publish_dir "$REPO_ROOT/$member"
    status=$?
    set -e
    
    if [[ $status -ne 0 ]] && [[ $status -ne 10 ]]; then
        if [[ "$CONTINUE_ON_ERROR" -eq 1 ]]; then
            echo "Skipping failed crate and continuing: $member" >&2
            FAILED_MEMBERS+=("$member")
            continue
        fi
        exit 1
    fi
    
    if [[ "$idx" -lt "$((${#WORKSPACE_MEMBERS[@]} - 1))" ]] && [[ "$DRY_RUN_REQUESTED" -eq 0 ]]; then
        if [[ $status -eq 10 ]]; then
            if [[ "$SKIP_DELAY" -gt 0 ]]; then
                echo "⏱️  Waiting ${SKIP_DELAY}s before next crate..."
                sleep "$SKIP_DELAY"
            fi
            elif [[ "$INTER_CRATE_DELAY" -gt 0 ]]; then
            echo "⏱️  Waiting ${INTER_CRATE_DELAY}s before next publish (rate limit compliance)..."
            sleep "$INTER_CRATE_DELAY"
        fi
    fi
done

if [[ "${#FAILED_MEMBERS[@]}" -gt 0 ]]; then
    echo "Sub-crate publish failures detected: ${FAILED_MEMBERS[*]}" >&2
    echo "Skipping parent crate publish because parent depends on all sub-crates." >&2
    exit 2
fi

if [[ "$DRY_RUN_REQUESTED" -eq 0 ]]; then
    if [[ "$INTER_CRATE_DELAY" -gt 0 ]]; then
        echo "⏱️  Waiting ${INTER_CRATE_DELAY}s before parent crate publish..."
        sleep "$INTER_CRATE_DELAY"
    fi
fi

set +e
publish_dir "$REPO_ROOT"
parent_status=$?
set -e
if [[ $parent_status -ne 0 ]] && [[ $parent_status -ne 10 ]]; then
    exit 1
fi

echo "Publish sequence completed."
