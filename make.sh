#!/bin/sh
cargo build --release --target="armv7-unknown-linux-gnueabihf"
adb push target/armv7-unknown-linux-gnueabihf/release/rayhunter-daemon /data/rayhunter/rayhunter
adb shell '/bin/rootshell -c "/etc/init.d/rayhunter_daemon restart"'
