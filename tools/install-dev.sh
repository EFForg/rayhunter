#!/bin/env bash

set -e

mkdir build
cd build
curl -LOs "https://github.com/EFForg/rayhunter/releases/latest/download/release.tar"
curl -LOs "https://github.com/EFForg/rayhunter/releases/latest/download/release.tar.sha256"
if ! sha256sum -c --quiet release.tar.sha256; then
    echo "Download corrupted! (╯°□°)╯︵ ┻━┻"
    exit 1
fi

tar -xf release.tar
./install.sh

cd ..
rm -rf build
