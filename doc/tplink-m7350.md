# TP-Link M7350

Supported in Rayhunter since version 0.3.0.

The TP-Link M7350 supports many more frequency bands than Orbic and therefore works in Europe and also in some Asian and African countries.

<div class="warning-box">

**WARNING: Do not buy hardware version `10.0`. Rayhunter does not work on it, and most likely never will.**

New M7350 units currently on sale are `10.0`. If you want an M7350, buy
second-hand and confirm the hardware version with the seller before paying.

See [issue #1105](https://github.com/EFForg/rayhunter/issues/1105) for details.

</div>

## Supported Bands

| Technology | Bands |
| ---------- | ----- |
| 4G LTE | B1/B3/B7/B8/B20 (2100/1800/2600/900/800 MHz) |
| 3G | B1/B8 (2100/900 MHz) |
| 2G | 850/900/1800/1900 MHz |

*Source: [TP-Link Official Product Page](https://www.tp-link.com/baltic/service-provider/lte-3g/m7350/)*

## Hardware versions

The TP-Link comes in many different *hardware versions*. Support for installation varies:

* `1.0`, Confirmed working. Successfully tested by a user with the Windows installer (rayhunter-v0.10.2-windows-x86_64). Ensure the SD card is formatted as FAT32 before installation.
* `2.0`: **Not supported**, devs are not able to obtain a device
* `3.0`, `3.2`, `5.0`, `5.2`, `7.0`, `8.0`: **Tested, no known issues since 0.3.0.**
* `6.2`: **One user reported it is working, not tested**
* `4.0`: **Manual firmware downgrade required** ([issue](https://github.com/EFForg/rayhunter/issues/332))
* `9.0`: **Working since 0.3.2.**
* `10.0`: **Not working** ([issue](https://github.com/EFForg/rayhunter/issues/1105)). Do not buy this version, see the warning above.

TP-Link versions newer than `3.0` have cyan packaging and a color display. Version `3.0` has a one-bit display and white packaging.

You can find the exact hardware version of each device under the battery or next to the barcode on the outer packaging, for example `V3.0` or `V5.2`. 

When filing bug reports, particularly with the installer, please always specify the exact hardware version.

You can get your TP-Link M7350 from the sources below. **Buy second-hand and
verify the hardware version is not `10.0` before paying** -- new stock is `10.0`
and does not work.

* First check for used offers on local sites, sometimes it's much cheaper there.
* [Geizhals price comparison](https://geizhals.eu/?fs=tp-link+m7350).
* [Ebay](https://www.ebay.com/sch/i.html?_nkw=tp-link+m7350&_sacat=0&_from=R40&_trksid=p4432023.m570.l1313).
* Can also be found sold as the 'Vodafone Pocket Wifi 5' in Australia 

## Installing

To get started, follow the [release installation guide](./installing-from-release.md). Then run the installer with the following command:

```bash
./installer tplink
```

The Rayhunter UI will be available at <http://192.168.0.1:8080>.

<a name=shell></a>
## Obtaining a shell

You can obtain a root shell with the following command:

```sh
./installer util tplink-shell
```

## Display states

If your device has a color display, Rayhunter will show the same red/green/white line at the top of the display as it does on Orbic, each color meaning "warning"/"recording"/"paused" respectively. See [Using Rayhunter](./using-rayhunter.md).

If your device has a one-bit (black-and-white) display, Rayhunter will instead show an emoji to indicate status:

* `!` means "warning (potential IMSI catcher)"
* `:)` (smiling) means "recording"
* `:` (face with no mouth) means "paused"

## Power-saving mode/sleep

By default the device will go to sleep after N minutes of no devices being connected. In that mode it will also turn off connections to cell phone towers.
In order for Rayhunter to record continuously, you have to turn off this sleep mode in TP-Link's admin panel (go to **Advanced** - **Power Saving**) or keep e.g. your phone connected on the TP-Link's WiFi.

## Port triggers

On hardware revisions starting with v4.0, the installer will modify settings to
add two port triggers. You can look at `Settings > NAT Settings > Port
Triggers` in TP-Link's admin UI to see them.

1. One port trigger "rayhunter-root" to launch the telnet shell. This is only needed for installation, and can be removed after upgrade. You can reinstall it using `./installer util tplink-shell`.
2. One port trigger "rayhunter-daemon" to auto-start Rayhunter on boot. If you remove this, Rayhunter will have to be started manually from shell.

## Other links

For more information on the device and instructions on how to install Rayhunter without an installer (i.e. manually), please see [rayhunter-tplink-m7350](https://github.com/m0veax/rayhunter-tplink-m7350/)
