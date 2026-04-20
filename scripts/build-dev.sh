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
}

build_frontend() {
    echo "Building web frontend..."
    pushd daemon/web > /dev/null
    npm install
    npm run build
    popd > /dev/null
}

build_wifi_tools() {
    if [ -f "tools/build-wpa-supplicant/out/wpa_supplicant" ] \
        && [ -f "tools/build-wpa-supplicant/out/wpa_cli" ] \
        && [ -f "tools/build-wpa-supplicant/out/iw" ]; then
        echo "WiFi tools already built, skipping."
        return
    fi

    if ! command -v arm-linux-musleabihf-gcc &> /dev/null; then
        echo "Error: arm-linux-musleabihf-gcc not found."
        echo "Install with: brew install FiloSottile/musl-cross/musl-cross"
        echo "(Required because the installer bundles wpa_supplicant, wpa_cli, and iw for orbic-family devices.)"
        exit 1
    fi

    echo "Building WiFi tools..."
    ./scripts/build-wpa-supplicant.sh
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
        build_wifi_tools
        build_daemon
        echo ""
        echo "Build complete! To install to a device, run:"
        echo "  ./scripts/install-dev.sh <device>"
        echo ""
        echo "Replace <device> with your device type (e.g. orbic, tplink)."
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
