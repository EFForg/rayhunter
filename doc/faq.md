# Frequently Asked Questions

### Do I need an active SIM card to use Rayhunter?

**It Depends**. Operation of Rayhunter does require the insertion of a SIM card into the device, but whether that SIM card has to be currently active for our tests to work is still under investigation. If you want to use the device as a hotspot in addition to a research device an active plan would of course be necessary, however we have not done enough testing yet to know whether an active subscription is required for detection. If you want to test the device with an inactive SIM card, we would certainly be interested in seeing any data you collect, and especially any runs that trigger an alert!

<a name="red"></a>

### Help, Rayhunter's line is red! What should I do?

Unfortunately, the circumstances that might lead to a positive cell site simulator (CSS) signal are quite varied, so we don't have a universal recommendation for how to deal with the a positive signal. Depending on your circumstances and threat model, you may want to turn off your phone until you are out of the area (or put it on airplane mode) and tell your friends to do the same!

If you've received a Rayhunter warning and would like to help us with our research, please send your Rayhunter data captures (QMDL and PCAP logs) to us at our [Signal](https://signal.org/) username [**ElectronicFrontierFoundation.90**](https://signal.me/#eu/HZbPPED5LyMkbTxJsG2PtWc2TXxPUR1OxBMcJGLOPeeCDGPuaTpOi5cfGRY6RrGf) with the following information: capture date, capture location, device, device model, and Rayhunter version. If you're unfamiliar with Signal, feel free to check out our [Security Self Defense guide on it](https://ssd.eff.org/module/how-to-use-signal).

Please note that this file may contain sensitive information such as your IMSI and the unique IDs of cell towers you were near which could be used to ascertain your location at the time.


### Should I get a locked or unlocked orbic device? What is the difference?

If you want to use a non-Verizon SIM card you will probably need an unlocked device. But it's not clear how locked the locked devices are nor how to unlock them, we welcome any experimentation and information regarding the use of unlocked devices.


### How do I re-enable USB tethering after installing Rayhunter?

Make sure USB tethering is also enabled in the Orbic's UI, and then run the following commands:

```sh
./installer util shell "echo 9 > /usrdata/mode.cfg"
./installer util shell reboot
```

To disable tethering again:

```sh
./installer util shell "echo 3 > /usrdata/mode.cfg"
./installer util shell reboot
```

See `/data/usb/boot_hsusb_composition` for a list of USB modes and Android USB gadget settings.


### How do I disable the WiFi hotspot on the Orbic RC400L?

To disable both WiFi bands:

```sh
adb shell
/bin/rootshell -c "sed -i 's/<wlan><Feature><state>1<\/state>/<wlan><Feature><state>0<\/state>/g' /usrdata/data/usr/wlan/wlan_conf_6174.xml && reboot"
```

To re-enable WiFi:

```sh
adb shell
/bin/rootshell -c "sed -i 's/<wlan><Feature><state>0<\/state>/<wlan><Feature><state>1<\/state>/g' /usrdata/data/usr/wlan/wlan_conf_6174.xml && reboot"
```
