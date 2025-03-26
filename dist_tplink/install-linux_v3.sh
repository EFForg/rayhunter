#!/bin/env bash

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

. "$(dirname "$0")"/install-common_v3.sh
install
