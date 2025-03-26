#!/usr/bin/env bash
set -e

force_debug_mode() {
    echo "Using adb at $ADB"
    echo "Force a switch into the debug mode to enable ADB"
    "$SERIAL_PATH" --root
    echo -n "adb enabled, waiting for reboot..."
    wait_for_adb_shell
    echo " it's alive!"
    echo -n "waiting for atfwd_daemon to startup..."
    wait_for_atfwd_daemon
    echo " done!"
}

wait_for_atfwd_daemon() {
    until [ -n "$(_adb_shell 'pgrep atfwd_daemon')" ]
    do
        sleep 1
    done
}

wait_for_adb_shell() {
    until _adb_shell true 2> /dev/null
    do
        sleep 1
    done
}

setup_rootshell() {
    _adb_push rootshell /tmp/
    _at_syscmd "cp /tmp/rootshell /bin/rootshell"
    sleep 1
    _at_syscmd "chown root /bin/rootshell"
    sleep 1
    _at_syscmd "chmod 4755 /bin/rootshell"
    _adb_shell '/bin/rootshell -c id'
    echo "we have root!"
}

_adb_push() {
    "$ADB" push "$(dirname "$0")/$1" "$2"
}

_adb_shell() {
    "$ADB" shell "$1"
}

_at_syscmd() {
    "$SERIAL_PATH" "AT+SYSCMD=$1"
}

setup_rayhunter() {
    _at_syscmd "mkdir -p /data/rayhunter"
    _adb_push config.toml.example /tmp/config.toml
    _at_syscmd "mv /tmp/config.toml /data/rayhunter"
    _adb_push rayhunter-daemon /tmp/rayhunter-daemon
    _at_syscmd "mv /tmp/rayhunter-daemon /data/rayhunter"
    _adb_push scripts/rayhunter_daemon /tmp/rayhunter_daemon
    _at_syscmd "mv /tmp/rayhunter_daemon /etc/init.d/rayhunter_daemon"
    _adb_push scripts/misc-daemon /tmp/misc-daemon
    _at_syscmd "mv /tmp/misc-daemon /etc/init.d/misc-daemon"

    _at_syscmd "chmod 755 /etc/init.d/rayhunter_daemon"
    _at_syscmd "chmod 755 /etc/init.d/misc-daemon"

    echo -n "waiting for reboot..."
    _at_syscmd "shutdown -r -t 1 now"

    # first wait for shutdown (it can take ~10s)
    until ! _adb_shell true 2> /dev/null
    do
        sleep 1
    done

    # now wait for boot to finish
    wait_for_adb_shell

    echo " done!"
}

test_rayhunter() {
    URL="http://localhost:8080"
    "$ADB" forward tcp:8080 tcp:8080 > /dev/null
    echo -n "checking for rayhunter server..."

    SECONDS=0
    while (( SECONDS < 30 )); do
        if curl -L --fail-with-body "$URL" -o /dev/null -s; then
            echo "success!"
            echo "you can access rayhunter at $URL"
            return
        fi
        sleep 1
    done
    echo "timeout reached! failed to reach rayhunter url $URL, something went wrong :("
}

##### ##### #####
##### Main  #####
##### ##### #####
if [[ `uname -s` == "Linux" ]]; then
    export SERIAL_PATH="./serial-ubuntu-24/serial"
    export PLATFORM_TOOLS="platform-tools-latest-linux.zip"
elif [[ `uname -s` == "Darwin" ]]; then
    if [[ `uname -m` == "arm64" ]]; then
        export SERIAL_PATH="./serial-macos-arm/serial"
    elif [[ `uname -m` == "x86_64" ]]; then
        export SERIAL_PATH="./serial-macos-intel/serial"
    fi
    export PLATFORM_TOOLS="platform-tools-latest-darwin.zip"
    xattr -d com.apple.quarantine "$SERIAL_PATH"
else
    echo "This script only supports Linux or macOS"
    exit 1
fi

if [ ! -x "$SERIAL_PATH" ]; then
    echo "The serial binary cannot be found at $SERIAL_PATH. If you are running this from the git tree please instead run it from the latest release bundle https://github.com/EFForg/rayhunter/releases"
  exit 1
fi

if ! command -v adb &> /dev/null; then
    if [ ! -d ./platform-tools ] ; then
        echo "adb not found, downloading local copy"
        curl -O "https://dl.google.com/android/repository/${PLATFORM_TOOLS}"
        unzip $PLATFORM_TOOLS
    fi
    export ADB="./platform-tools/adb"
else
    export ADB=`which adb`
fi

force_debug_mode
setup_rootshell
setup_rayhunter
test_rayhunter
