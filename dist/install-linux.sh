#!/bin/env bash

set -e
export SERIAL_PATH="./serial-ubuntu-latest/serial"

if [ ! -x "$SERIAL_PATH" ]; then
  echo "The serial binary cannot be found at $SERIAL_PATH. If you are running this from the git tree please instead run it from the latest release bundle https://github.com/EFForg/rayhunter/releases"
  exit 1
fi

if ! command -v adb &> /dev/null; then
    if [ ! -d ./platform-tools ] ; then
        echo "adb not found, downloading local copy"
        curl -O "https://dl.google.com/android/repository/platform-tools-latest-linux.zip"
        unzip platform-tools-latest-linux.zip
    fi
    export ADB="./platform-tools/adb"
else
    export ADB=`which adb`
fi

. "$(dirname "$0")"/install-common.sh
install
