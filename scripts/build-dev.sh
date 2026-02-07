#!/bin/bash
# Build Rayhunter from source for development.
# Prerequisites: Rust (rustup) and Node.js (npm).
#
# Usage: ./scripts/build-dev.sh [build|frontend|check]

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

cd "$PROJECT_DIR"

check_dependencies() {
    local missing=0

    if ! command -v cargo &> /dev/null; then
        echo "Error: cargo not found. Install Rust via https://www.rust-lang.org/tools/install"
        missing=1
    fi

    if ! command -v npm &> /dev/null; then
        echo "Error: npm not found. Install Node.js via https://docs.npmjs.com/downloading-and-installing-node-js-and-npm"
        missing=1
    fi

    if [ "$missing" -eq 1 ]; then
        exit 1
    fi

    # Ensure the ARM cross-compilation target is installed
    if ! rustup target list --installed | grep -q "armv7-unknown-linux-musleabihf"; then
        echo "Installing ARM target (armv7-unknown-linux-musleabihf)..."
        rustup target add armv7-unknown-linux-musleabihf
    fi

    echo "All dependencies found."
}

build_frontend() {
    echo "Building web frontend..."
    pushd daemon/web > /dev/null
    npm install
    npm run build
    popd > /dev/null
}

build_daemon() {
    echo "Building daemon..."
    cargo build-daemon-firmware-devel

    echo "Building rootshell..."
    cargo build-rootshell-firmware-devel
}

COMMAND="${1:-build}"

case "$COMMAND" in
    build)
        check_dependencies
        build_frontend
        build_daemon
        echo ""
        echo "Build complete! To install to a device, run:"
        echo "  FIRMWARE_PROFILE=firmware-devel cargo run -p installer --bin installer <device>"
        echo ""
        echo "Replace <device> with your device type (e.g. orbic, tplink)."
        echo "Run 'cargo run --bin installer help' for a list of supported devices."
        ;;
    frontend)
        build_frontend
        ;;
    check)
        check_dependencies
        ;;
    help|--help|-h)
        echo "Usage: $0 [command]"
        echo ""
        echo "Commands:"
        echo "  build     Build frontend, daemon, and rootshell (default)"
        echo "  frontend  Build only the web frontend"
        echo "  check     Check dependencies only"
        ;;
    *)
        echo "Unknown command: $COMMAND"
        echo "Run '$0 help' for usage."
        exit 1
        ;;
esac
