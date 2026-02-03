# Orbic/Kajeet RC400L

The Orbic RC400L is an inexpensive LTE modem primarily designed for the US market, and the original device for which Rayhunter is developed.

It is also sometimes sold under the brand Kajeet RC400L. This is the exact same hardware and can be treated the same. 

You can buy an Orbic [using bezos
bucks](https://www.amazon.com/Orbic-Verizon-Hotspot-Connect-Enabled/dp/B08N3CHC4Y),
or on [eBay](https://www.ebay.com/sch/i.html?_nkw=orbic+rc400l). You should not
pay more than 30 USD for such a device (without shipping).

[Please check whether the Orbic works in your country](https://www.frequencycheck.com/countries/), and whether the Orbic RC400L supports the right frequency bands for your purpose before buying.

## Supported Bands

| Frequency | Band          |
| ------- | ------------------ |
| 5G (wideband,midband,nationwide)  | n260/n261, n77, n2/5/48/66 |
| 4G |    2/4/5/12/13/48/66       |
| Global & Roaming | n257/n78     |
| Wifi 2.4Ghz | b/g/n |
| Wifi 5Ghz | a/ac/ax |
| Wifi 6 | 🮱 |

## Two kinds of installers

The orbic's installation routine underwent many different changes:

1. The ADB-based shellscript prior to version 0.3.0
2. The Rust-based, ADB-based installer since version 0.3.0
3. Then, starting with 0.6.0, an alternative installer `./installer
   orbic-network` that is supposed to work more reliably, can run over the
   Orbic's WiFi connection and without the need to manually install USB drivers
   on Windows.
4. Starting with 0.8.0, `orbic-network` has been renamed to `orbic`, and the
   old `./installer orbic` is now called `./installer orbic-usb`.
   
It's possible that many tutorials out there still refer to some of the old
installation routines.

<a name=shell></a>
## Obtaining a shell

After running the installer, there will not be a rootshell and ADB will not be
enabled. Instead you can use `./installer util orbic-shell`.

If you are using an installer prior to 0.7.0 or `orbic-usb` explicitly, you can
obtain a root shell by running `adb shell` or `./installer util shell`. Then,
inside of that shell you can run `/bin/rootshell` to obtain "fakeroot."
