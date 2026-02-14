#!/bin/sh
# Pushes all client-mode files to the Orbic device via ADB.
# Run from the rayhunter repo root.
set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
WPA_DIR="$SCRIPT_DIR/../../tools/build-wpa-supplicant/out"

if ! adb devices | grep -q device$; then
    echo "No ADB device found"
    exit 1
fi

echo "Pushing scripts..."
adb shell "mkdir -p /data/rayhunter/scripts /data/rayhunter/bin"
adb push "$SCRIPT_DIR/wifi-client.sh" /data/rayhunter/scripts/wifi-client.sh

if [ -f "$WPA_DIR/wpa_supplicant" ]; then
    echo "Pushing wpa_supplicant binaries..."
    adb push "$WPA_DIR/wpa_supplicant" /data/rayhunter/bin/wpa_supplicant
    adb push "$WPA_DIR/wpa_cli" /data/rayhunter/bin/wpa_cli
else
    echo "wpa_supplicant binaries not found at $WPA_DIR"
    echo "Build them first: see tools/build-wpa-supplicant/Dockerfile"
    exit 1
fi

echo ""
echo "Files pushed. Set WiFi credentials via the web UI or installer,"
echo "then reboot. WiFi client starts automatically on boot."
