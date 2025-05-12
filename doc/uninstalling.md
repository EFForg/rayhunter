# Uninstalling

## Orbic

To uninstall Rayhunter, power on your Orbic device and connect to it via USB. Then, start a rootshell on it by running `adb shell`, followed by `rootshell`.

Once in a rootshell, run:

```
echo 3 > /usrdata/mode.cfg
rm -rf /data/rayhunter /etc/init.d/rayhunter-daemon /bin/rootshell.sh
reboot
```

Your device is now Rayhunter-free, and should no longer be in a rooted ADB-enabled mode.

## TPLink

TODO
