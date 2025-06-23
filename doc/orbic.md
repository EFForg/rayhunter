# Orbic RC400L

The Orbic RC400L is an inexpensive LTE modem primarily designed for the US marked, and the original device for which Rayhunter is developed.

You can buy an Orbic [using bezos
bucks](https://www.amazon.com/Orbic-Verizon-Hotspot-Connect-Enabled/dp/B08N3CHC4Y),
or on [eBay](https://www.ebay.com/sch/i.html?_nkw=orbic+rc400l).

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

## Obtaining a shell

After running through the installation procedure, you can obtain a root shell
by running `adb shell` or `./installer util shell`. Then, inside of that shell
you can run `/bin/rootshell` to obtain "fakeroot."
