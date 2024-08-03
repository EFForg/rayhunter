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
    test_rayhunter
}

check_adb() {
    if ! command -v adb &> /dev/null
    then
        echo "adb not found, please ensure it's installed or check the README.md"
        exit 1
    fi
}

force_debug_mode() {
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
    until [ -n "$(adb shell 'pgrep atfwd_daemon')" ]
    do
        sleep 1
    done
}

wait_for_adb_shell() {
    until adb shell true 2> /dev/null
    do
        sleep 1
    done
}

setup_rootshell() {
    _adb_push rootshell /tmp/
    "$SERIAL_PATH" "AT+SYSCMD=cp /tmp/rootshell /bin/rootshell"
    sleep 1
    "$SERIAL_PATH" "AT+SYSCMD=chown root /bin/rootshell"
    sleep 1
    "$SERIAL_PATH" "AT+SYSCMD=chmod 4755 /bin/rootshell"
    adb shell /bin/rootshell -c id
    echo "we have root!"
}

_adb_push() {
    adb push "$(dirname "$0")/$1" "$2"
}

setup_rayhunter() {
    adb shell '/bin/rootshell -c "mkdir -p /data/rayhunter"'
    _adb_push config.toml.example /data/rayhunter/config.toml
    _adb_push rayhunter-daemon /data/rayhunter/
    _adb_push scripts/rayhunter_daemon /tmp/rayhunter_daemon
    _adb_push scripts/misc-daemon /tmp/misc-daemon
    adb shell '/bin/rootshell -c "cp /tmp/rayhunter_daemon /etc/init.d/rayhunter_daemon"'
    adb shell '/bin/rootshell -c "cp /tmp/misc-daemon /etc/init.d/misc-daemon"'
    adb shell '/bin/rootshell -c "chmod 755 /etc/init.d/rayhunter_daemon"'
    adb shell '/bin/rootshell -c "chmod 755 /etc/init.d/misc-daemon"'
    echo -n "waiting for reboot..."
    adb shell '/bin/rootshell -c reboot'

    # first wait for shutdown (it can take ~10s)
    until ! adb shell true 2> /dev/null
    do
        sleep 1
    done

    # now wait for boot to finish
    wait_for_adb_shell

    echo " done!"
}

test_rayhunter() {
    URL="http://localhost:8080"
    adb forward tcp:8080 tcp:8080 > /dev/null
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
