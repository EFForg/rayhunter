# Uninstalling

## Orbic

To uninstall Rayhunter, power on your Orbic device and connect to it via USB. Then, start a rootshell on it by running `adb shell`, followed by `rootshell`.

Once in a rootshell, run:

```shell
echo 3 > /usrdata/mode.cfg
rm -rf /data/rayhunter /etc/init.d/rayhunter-daemon /bin/rootshell.sh
reboot
```

Your device is now Rayhunter-free, and should no longer be in a rooted ADB-enabled mode.

## TPLink

1. Run `./installer util tplink-start-telnet`
2. Telnet into the device `telnet 192.168.0.1`
3. `rm /data/rayhunter /etc/init.d/rayhunter_daemon`
4. `update-rc.d rayhunter_daemon remove`
5. (hardware revision v4.0+ only) In `Settings > NAT Settings > Port Triggers` in TP-Link's admin UI, remove any leftover port triggers.

