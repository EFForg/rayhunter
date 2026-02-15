# ZTE MF920V

Supported in Rayhunter since version 0.11.0.

The ZTE MF920V is a cheaper alternative to the TP-Link M7350. Many carriers in Europe sell a branded (and often locked) version of this device along with their plans.

You can find this device second-hand for as low as 10 EUR.

Compared to the TP-Link, it has many downsides:

* Has no SD card slot, and internal storage is limited to 20 MB. This is ~10 MB for recordings, which is similar to Orbic etc.
* The battery cannot be removed. The batterypack can be accessed without screws but is glued in.
* only LEDs instead of a display

## Known Variants

This device is sold under different names depending on region and carrier. Each
variant has different frequency bands unlocked (with no clear documentation
found online) and might be locked to a particular carrier:

| Brand Name | Region | Verified working
| ---------- | ------ |
| ZTE MF920V | Global | ✅
| Vodafone R218 | Europe |
| DreiPocket | Austria |
| Megafon MR150-5 | Russia |
| MTS 835F | Russia |
| Beeline MF920 | Russia |
| Tele2 MF920 | Russia |
| Yota MF920 | Russia |
| Airtel MF920V | India |

We have not tested Rayhunter on all of these variants.

If you have this device under a different brand name or can confirm it works under one of the above, please add to this document.

## Installation & Usage

Follow the [release installation guide](./installing-from-release.md). Use `./installer zte --admin-password <your-password>` where `<your-password>` is the admin password for the device's web interface.

Note that when freshly reset, the admin portal has no password and first needs to be set.

The Rayhunter UI will be available at <http://192.168.0.1:8080>.

## Obtaining a shell

You can obtain a root shell with the following command:

```sh
./installer util zte-shell --admin-password <your-password>
```

## Physical LEDs

The ZTE MF920V has 4 distinct LED indicators on the front panel.

Rayhunter uses the **Network LED** (bottom right, signal bars 📶 icon) to indicate status, as it is the only LED with RGB capability:

| Color | Meaning |
| ----- | ------- |
| Green | Recording (changed to dark blue in colorblind mode) |
| White | Paused (slightly tinted blue due to hardware limitations) |
| Red | Warning (potential IMSI catcher detected) |

These align with the colors used by the red/green bar on devices with LCD screens.

## Storage

The ZTE MF920V has limited internal storage. Rayhunter stores data on `/cache` (approximately 24MB available) which persists across reboots. A symlink at `/data/rayhunter` points to `/cache/rayhunter`.
