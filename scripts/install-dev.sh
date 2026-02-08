#!/bin/bash
# Install a development build of Rayhunter to a device.
# Run ./scripts/build-dev.sh first.
#
# Usage: ./scripts/install-dev.sh <device> [options...]
# Example: ./scripts/install-dev.sh orbic --admin-password mypass

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

cd "$PROJECT_DIR"
cargo run -p installer --bin installer -- "$@"
