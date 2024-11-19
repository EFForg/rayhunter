#!/usr/bin/env bash
install() {
    if [[ -z "${SERIAL_PATH}" ]]; then
        echo "\$SERIAL_PATH not set, did you run this from install-linux.sh or install-mac.sh?"
        exit 1
    fi
    if [[ -z "${ADB}" ]]; then
        echo "\$ADB not set, did you run this from install-linux.sh or install-mac.sh?"
        exit 1
    fi
    force_debug_mode
    setup_rootshell
    setup_rayhunter
    test_rayhunter
}

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
    _at_syscmd reboot

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
