# TP-Link M7350

The TP-Link M7350 is supported by Rayhunter as of 0.3.0. It supports many more frequency bands than Orbic and therefore works in Europe.

## Hardware versions

The TP-Link comes in many different *hardware versions*. Support for installation varies:

* `1.0`, `2.0`: **Not suported**, probably impossible to obtain anymore (even second-hand)
* `3.0`, `3.2`, `5.0`, `5.2`, `7.0`, `8.0`: **Tested, no known issues.**
* `4.0`: **Not working.** [issue](https://github.com/EFForg/rayhunter/issues/332)
* `9.0`: **Not working.** [issue](https://github.com/EFForg/rayhunter/issues/325)

TP-Link versions newer than `3.0` have cyan packaging and a color display.
Version `3.0` has a one-bit display and white packaging.

You can find the exact hardware version of each device under the battery or
next to the barcode on the outer packaging, for example `V3.0` or `V5.2`. 

When filing bug reports, particularly with the installer, please always
specify the exact hardware version.

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

## Power-saving mode/sleep

By default the device will go to sleep after N minutes of no devices being
connected. In that mode it will also turn off connections to cell phone towers.
In order for Rayhunter to record continuously, you have to turn off this sleep
mode in TP-Link's admin panel or keep e.g. your phone in the TP-Link's WiFi.

## Other links

For more information on the device and instructions on how to install Rayhunter without an installer, see [rayhunter-tplink-m7350](https://github.com/m0veax/rayhunter-tplink-m7350/)
