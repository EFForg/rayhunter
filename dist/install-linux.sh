#!/bin/env bash

set -e
if ! command -v adb &> /dev/null; then
    echo "adb not found, downloading local copy"
    curl -o "https://dl.google.com/android/repository/platform-tools-latest-linux.zip"
    unzip platform-tools-latest-linux.zip
    export ADB=`./platform-tools/adb`
else
    export ADB=`which adb`
fi

export SERIAL_PATH="./serial-ubuntu-latest/serial"
. "$(dirname "$0")"/install-common.sh
install
