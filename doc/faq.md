# Frequently Asked Questions

### Do I need an active SIM card to use Rayhunter?

**It Depends**. Operation of Rayhunter does require the insertion of a SIM card into the device, but that sim card does not have to be actively registered with a service plan. If you want to use the device as a hotspot in addition to a research device, or get [notifications](./configuration.md), an active plan would of course be necessary.

### How can I test that my device is working?
You can enable the `Test Heuristic` under `Analyzer Heuristic Settings` in the config section on your web dashboard. This will cause an alert to trigger every time your device sees a cell tower, you might need to reboot your device or move around a bit to get this one to trigger, but it will be very noisy once it does. People have also tested it by building IMSI catchers at home, but we don't recommend that, since it violates FCC regulations and will probably upset your neighbors.

<a name="red"></a>

### Help, Rayhunter's line is red/orange/yellow/dotted/dashed! What should I do?

Unfortunately, the circumstances that might lead to a positive cell site simulator (CSS) signal are quite varied, so we don't have a universal recommendation for how to deal with the a positive signal. Depending on your circumstances and threat model, you may want to turn off your phone until you are out of the area and tell your friends to do the same!

If you've received a Rayhunter warning and would like to help us with our research, please send your Rayhunter data captures (Zip file downloaded from the web interface) to us at our [Signal](https://signal.org/) username [**ElectronicFrontierFoundation.90**](https://signal.me/#eu/HZbPPED5LyMkbTxJsG2PtWc2TXxPUR1OxBMcJGLOPeeCDGPuaTpOi5cfGRY6RrGf) with the following information: capture date, capture location, device, device model, and Rayhunter version. If you're unfamiliar with Signal, feel free to check out our [Security Self Defense guide on it](https://ssd.eff.org/module/how-to-use-signal).

Please note that this file may contain sensitive information such as your IMSI and the unique IDs of cell towers you were near which could be used to ascertain your location at the time.


### Should I get a locked or unlocked orbic device? What is the difference?

If you want to use a non-Verizon SIM card you will probably need an unlocked device. But it's not clear which devices are locked nor how to unlock them, we welcome any experimentation and information regarding the use of unlocked devices. So far most verizon branded orbic devices we have encountered are actually unlocked.

### How do I re-enable USB tethering after installing Rayhunter?

If you have installed with `./installer orbic-usb`, you might find that USB
tethering is now disabled. If you have run `./installer orbic`, this section is not
relevant as it does not use or touch USB.

[First obtain a shell](./orbic.md#shell), then:


```sh
# inside of Orbic's shell:
echo 9 > /usrdata/mode.cfg
reboot
```

Make sure USB tethering is also enabled in the Orbic's UI.

To disable tethering again:

```sh
# inside of Orbic's shell:
echo 3 > /usrdata/mode.cfg
reboot
```

See `/data/usb/boot_hsusb_composition` for a list of USB modes and Android USB gadget settings.


### How do I disable the WiFi hotspot on the Orbic RC400L?

To disable both WiFi bands, [first obtain a shell](./orbic.md#shell), then:

```sh
# inside of Orbic's shell:
sed -i 's/<wlan><Feature><state>1<\/state>/<wlan><Feature><state>0<\/state>/g' /usrdata/data/usr/wlan/wlan_conf_6174.xml && reboot
```

To re-enable WiFi:

```sh
# inside of Orbic's shell:
sed -i 's/<wlan><Feature><state>0<\/state>/<wlan><Feature><state>1<\/state>/g' /usrdata/data/usr/wlan/wlan_conf_6174.xml && reboot
```
