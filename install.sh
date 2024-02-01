cd serial 
cargo build_pc
cd ..
cd rootshell
cargo build --release
cd ..
# Force a switch into the debug mode to enable ADB
target/x86_64-unknown-linux-gnu/debug/serial AT
echo -n "device rooted, waiting for reboot"
until adb shell true 2> /dev/null
do
    echo -n .
    sleep 1
done
echo
echo "it's alive!"
adb push target/armv7-unknown-linux-gnueabihf/release/rootshell /tmp/
target/x86_64-unknown-linux-gnu/debug/serial "AT+SYSCMD=mv /tmp/rootshell /bin/rootshell"
sleep 1
target/x86_64-unknown-linux-gnu/debug/serial "AT+SYSCMD=chown root /bin/rootshell"
sleep 1
target/x86_64-unknown-linux-gnu/debug/serial "AT+SYSCMD=chmod 4755 /bin/rootshell"
echo "we have root!"
adb shell /bin/rootshell -c id
./make.sh
