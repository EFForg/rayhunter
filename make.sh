cargo build
# Force a switch into the debug mode to enable ADB
target/x86_64-unknown-linux-gnu/debug/serial AT
adb push target/armv7-unknown-linux-gnueabihf/debug/rootshell /tmp/
target/x86_64-unknown-linux-gnu/debug/serial "AT+SYSCMD=mv /tmp/rootshell /bin/rootshell"
sleep 1
target/x86_64-unknown-linux-gnu/debug/serial "AT+SYSCMD=chown root /bin/rootshell"
sleep 1
target/x86_64-unknown-linux-gnu/debug/serial "AT+SYSCMD=chmod 4755 /bin/rootshell"
adb push target/armv7-unknown-linux-gnueabihf/debug/wavehunter /data/wavehunter/wavehunter
adb push target/armv7-unknown-linux-gnueabihf/debug/wavehunter-reader /data/wavehunter/wavehunter-reader
