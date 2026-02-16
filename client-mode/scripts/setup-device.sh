#!/bin/sh
# Dev tool: pushes WiFi client-mode files to a device via ADB.
# For production installs, use the installer instead: ./installer moxee --admin-password X
#
# Usage: ./setup-device.sh [orbic|moxee]
# If no device specified, auto-detects via ADB uid (root=Moxee, shell=Orbic).
# Run from the rayhunter repo root.
set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
WPA_DIR="$SCRIPT_DIR/../../tools/build-wpa-supplicant/out"

if ! adb devices | grep -q device$; then
    echo "No ADB device found"
    exit 1
fi

DEVICE="$1"
if [ -z "$DEVICE" ]; then
    ADB_UID=$(adb shell id -u | tr -d '\r')
    if [ "$ADB_UID" = "0" ]; then
        DEVICE="moxee"
    else
        DEVICE="orbic"
    fi
    echo "Auto-detected device: $DEVICE (uid=$ADB_UID)"
fi

case "$DEVICE" in
    moxee)
        DEST="/cache/rayhunter"
        ;;
    orbic)
        DEST="/data/rayhunter"
        ;;
    *)
        echo "Unknown device: $DEVICE (expected 'orbic' or 'moxee')" >&2
        exit 1
        ;;
esac

echo "Pushing scripts to $DEST/scripts/..."
adb shell "mkdir -p $DEST/scripts $DEST/bin"
adb push "$SCRIPT_DIR/wifi-client.sh" "$DEST/scripts/wifi-client.sh"

if [ -f "$WPA_DIR/wpa_supplicant" ]; then
    echo "Pushing wpa_supplicant binaries to $DEST/bin/..."
    adb push "$WPA_DIR/wpa_supplicant" "$DEST/bin/wpa_supplicant"
    adb push "$WPA_DIR/wpa_cli" "$DEST/bin/wpa_cli"
else
    echo "wpa_supplicant binaries not found at $WPA_DIR"
    echo "Build them first: see tools/build-wpa-supplicant/Dockerfile"
    exit 1
fi

echo ""
echo "Files pushed to $DEST. Set WiFi credentials via the web UI or installer,"
echo "then reboot. WiFi client starts automatically on boot."
