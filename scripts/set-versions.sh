#!/bin/bash
# Sets Rayhunter package versions in preparation for a release.
#
# Usage: ./scripts/set-versions.sh VERSION_NUM
# Example: ./scripts/set-versions.sh 0.12.3

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

cd "$PROJECT_DIR"

if [ -z "$1" ]; then
    echo "Error: Missing required version number argument."
    exit 1
fi

SED_COMMAND="s/^version = \".*\"/version = \"$1\"/"
TOML_FILES=(*/Cargo.toml installer-gui/src-tauri/Cargo.toml)

echo "Updating Cargo.toml files"
if sed --version > /dev/null 2>&1; then
    # we have GNU sed
    sed -i -E "$SED_COMMAND" "${TOML_FILES[@]}"
else
    # we have macOS/BSD sed
    sed -i "" -E "$SED_COMMAND" "${TOML_FILES[@]}"
fi

echo "Updating Cargo.lock"
cargo update --workspace
