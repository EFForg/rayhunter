#!/bin/bash -e
pushd daemon/web
    npm install
    npm run build
popd
cargo build-daemon-firmware-devel
adb shell '/bin/rootshell -c "/etc/init.d/rayhunter_daemon stop"'
adb push target/armv7-unknown-linux-musleabihf/firmware-devel/rayhunter-daemon \
    /data/rayhunter/rayhunter-daemon
echo "rebooting the device..."
adb shell '/bin/rootshell -c "reboot"'
