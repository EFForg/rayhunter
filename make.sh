cross rustc --target armv7-unknown-linux-gnueabihf --  -C target-feature=+crt-static
adb push target/armv7-unknown-linux-gnueabihf/debug/diag /tmp/diag
