<<<<<<< Updated upstream
cargo build --release
adb push target/armv7-unknown-linux-gnueabihf/release/rayhunter /data/rayhunter/rayhunter
adb shell '/bin/rootshell -c "/etc/init.d/rayhunter_daemon restart"'
=======
# the "arm" profile inherits from "release", so this is an optimized build
cargo build --profile arm
adb push target/arm/rayhunter-daemon /data/rayhunter/
adb shell '/bin/rootshell -c "/etc/init.d/rayhunter_daemon restart"'
>>>>>>> Stashed changes
