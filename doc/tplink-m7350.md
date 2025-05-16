# TP-Link M7350

The TP-Link M7350 is supported by Rayhunter as of 0.2.9. It supports many more frequency bands than Orbic and therefore works in Europe.

You can get it from:

* First check for used offers on Ebay or equivalent, sometimes it's much cheaper there.
* [Geizhals price comparison](https://geizhals.eu/?fs=tp-link+m7350)
* [Ebay](https://www.ebay.com/sch/i.html?_nkw=tp-link+m7350&_sacat=0&_from=R40&_trksid=p4432023.m570.l1313)

## Installation & Usage

Follow the [release installation guide](./installing-from-release.md). Substitute `./installer orbic` for `./installer tplink` in other documentation. The rayhunter UI will be available at [http://192.168.0.1:8080](http://192.168.0.1:8080).

Unlike on Orbic, the installer will not enable ADB. Instead, you can do this to obtain a root shell:

```sh
./installer util tplink-start-telnet
telnet 192.168.0.1
```

## Display states

If your device has a color display, Rayhunter will show the same
red/green/white line at the top of the display as it does on Orbic, each color
meaning "warning"/"recording"/"paused" respectively. See [Using Rayhunter](./using-rayhunter.md).

If your device has a one-bit (black-and-white) display, Rayhunter will instead
show an emoji to indicate status:

* `!` means "warning (potential IMSI catcher)"
* `:)` (smiling) means "recording"
* `:` (face with no mouth) means "paused"

## Hardware versions

The TP-Link comes in many different *hardware versions*. Support for installation varies:

* `1.0-2.0`: Not tested, probably impossible to obtain anymore (even second-hand)
* `3.0`, `3.2`, `5.0`, `5.2`, `7.0`, `8.0`: Tested, no issues.
* `9.0`: Recording might be broken, could be fixed if there is demand.

TP-Link versions newer than `3.0` have cyan packaging and a color display.
Version `3.0` has a one-bit display and white packaging.

You can find the exact hardware version of each device under the battery or
next to the barcode on the outer packaging, for example `V3.0` or `V5.2`. 

When filing bug reports, particularly with the installer, please always
specify the exact hardware version.

## Other links

For more information on the device and instructions on how to install Rayhunter without an installer, see [rayhunter-tplink-m7350](https://github.com/m0veax/rayhunter-tplink-m7350/)
