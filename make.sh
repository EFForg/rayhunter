#!/bin/sh
cargo build --release --target="armv7-unknown-linux-gnueabihf" #--features debug
adb push target/armv7-unknown-linux-gnueabihf/release/rayhunter-daemon /data/rayhunter/rayhunter-daemon
adb shell '/bin/rootshell -c "/etc/init.d/rayhunter_daemon restart"'
