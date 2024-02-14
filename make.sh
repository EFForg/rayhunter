#!/bin/sh
cargo build --release --target="armv7-unknown-linux-gnueabihf" --bin rayhunter-daemon
adb push target/armv7-unknown-linux-gnueabihf/release/rayhunter-daemon /data/rayhunter/
adb shell '/bin/rootshell -c "/etc/init.d/rayhunter_daemon restart"'
