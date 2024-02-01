cargo build --release
adb push target/armv7-unknown-linux-gnueabihf/release/rayhunter /data/rayhunter/rayhunter
adb shell '/bin/rootshell -c "/etc/init.d/rayhunter_daemon restart"'