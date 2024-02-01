cd serial 
cargo build_pc
cd ..
cd rootshell
cargo build --release
cd ..
# Force a switch into the debug mode to enable ADB
target/x86_64-unknown-linux-gnu/debug/serial AT
echo -n "adb enabled, waiting for reboot"
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
adb shell '/bin/rootshell -c "mkdir /data/rayhunter"'
adb push config.toml.example /data/rayhunter/config.toml
adb push scripts/rayhunter_daemon /tmp/rayhunter_daemon
adb push scripts/misc-daemon /tmp/misc-daemon
adb shell '/bin/rootshell -c "mv /tmp/rayhunter_daemon /etc/init.d/rayhunter_daemon"'
adb shell '/bin/rootshell -c "mv /tmp/misc-daemon /etc/init.d/misc-daemon"'
adb shell '/bin/rootshell -c "chmod 755 /etc/init.d/rayhunter_daemon"'
adb shell '/bin/rootshell -c "chmod 755 /etc/init.d/misc-daemon"'
./make.sh
adb shell '/bin/rootshell -c "/etc/init.d/rayhunter_daemon start"'
