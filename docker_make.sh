#!/bin/bash -e
pushd bin/web
    npm run build
popd
#docker build -t rayhunter-devenv -f tools/devenv.dockerfile .
docker run --user $UID:$GID -v ./:/workdir -w /workdir -it rayhunter-devenv sh -c 'cargo build --release --target="armv7-unknown-linux-gnueabihf"'
adb shell '/bin/rootshell -c "/etc/init.d/rayhunter_daemon stop"'
adb push target/armv7-unknown-linux-gnueabihf/release/rayhunter-daemon /data/rayhunter/rayhunter-daemon
echo "rebooting the device..."
adb shell '/bin/rootshell -c "reboot"'
