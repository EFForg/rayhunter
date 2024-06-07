#!/bin/env bash

set -e
export SERIAL_PATH="./serial-mac-latest/serial"
. "$(dirname "$0")"/install-common.sh
install
