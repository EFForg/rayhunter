#!/bin/bash -e
pushd daemon/web
    npm run build
popd
cargo build --profile firmware-devel --bin rayhunter-daemon --target="armv7-unknown-linux-musleabihf" #--features debug
adb shell '/bin/rootshell -c "/etc/init.d/rayhunter_daemon stop"'
adb push target/armv7-unknown-linux-musleabihf/firmware-devel/rayhunter-daemon \
    /data/rayhunter/rayhunter-daemon
echo "rebooting the device..."
adb shell '/bin/rootshell -c "reboot"'
