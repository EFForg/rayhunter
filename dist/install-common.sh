#!/bin/env bash

install() {
    if [[ -z "${SERIAL_PATH}" ]]; then
        echo "SERIAL_PATH not set, did you run this from install-linux.sh or install-mac.sh?"
        exit 1
    fi
    check_adb
    force_debug_mode
    setup_rootshell
    setup_rayhunter
}

check_adb() {
    if ! command -v adb &> /dev/null
    then
        echo "adb not found, please ensure it's installed or check the README.md"
        exit 1
    fi
}

force_debug_mode() {
    # Force a switch into the debug mode to enable ADB
    $(SERIAL_PATH) AT
    echo -n "adb enabled, waiting for reboot"
    until adb shell true 2> /dev/null
    do
        echo -n .
        sleep 1
    done
    echo
    echo "it's alive!"
}

setup_rootshell() {
    _adb_push rootshell /tmp/
    $(SERIAL_PATH) "AT+SYSCMD=mv /tmp/rootshell /bin/rootshell"
    sleep 1
    $(SERIAL_PATH) "AT+SYSCMD=chown root /bin/rootshell"
    sleep 1
    $(SERIAL_PATH) "AT+SYSCMD=chmod 4755 /bin/rootshell"
    echo "we have root!"
    adb shell /bin/rootshell -c id
}

_adb_push() {
    adb push "$(dirname "$0")/$1" "$2"
}

setup_rayhunter() {
    adb shell '/bin/rootshell -c "mkdir /data/rayhunter"'
    _adb_push config.toml.example /data/rayhunter/config.toml
    _adb_push rayhunter-daemon /data/rayhunter/
    _adb_push scripts/rayhunter_daemon /tmp/rayhunter_daemon
    _adb_push scripts/misc-daemon /tmp/misc-daemon
    adb shell '/bin/rootshell -c "mv /tmp/rayhunter_daemon /etc/init.d/rayhunter_daemon"'
    adb shell '/bin/rootshell -c "mv /tmp/misc-daemon /etc/init.d/misc-daemon"'
    adb shell '/bin/rootshell -c "chmod 755 /etc/init.d/rayhunter_daemon"'
    adb shell '/bin/rootshell -c "chmod 755 /etc/init.d/misc-daemon"'
    adb shell '/bin/rootshell -c "/etc/init.d/rayhunter_daemon start"'
}
