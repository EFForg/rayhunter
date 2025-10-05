# Orbic/Kajeet RC400L

The Orbic RC400L is an inexpensive LTE modem primarily designed for the US market, and the original device for which Rayhunter is developed.

It is also sometimes sold under the brand Kajeet RC400L. This is the exact same hardware and can be treated the same. 

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

## The Network Installer

Since Rayhunter 0.6.0 there is an alternative, experimental installation
procedure at `./installer orbic-network` that is supposed to eventually replace
`./installer orbic`. It does not require any USB driver installation and works
identically on Windows, Mac and Linux. From our testing it works much more
reliably on Windows than `./installer orbic` does.

The drawback is that the device's admin password is required. 

1. Connect to the Orbic's network via WiFi or USB tethering
2. Run `./installer orbic-network --admin-password 'mypassword'`

   * On Verizon Orbic, the password is the WiFi password.
   * On Kajeet/Smartspot devices, the default password is `$m@rt$p0tc0nf!g`
   * On Moxee-brand devices, check under the battery for the password.
   * You can reset the password by pressing the button under the back case until the unit restarts.

3. The installer will eventually reboot the device, at which point the device is up and running.

## Obtaining a shell

After running through the installation procedure, you can obtain a root shell
by running `adb shell` or `./installer util shell`. Then, inside of that shell
you can run `/bin/rootshell` to obtain "fakeroot."

If you are using the network installer, there will not be a rootshell and ADB will not be enabled by the installer. Instead you can use `./installer util orbic-start-telnet` and connect to the hotspot using `nc 192.168.1.1 23`. On Windows you might not have `nc` and will have to use WSL for that.
