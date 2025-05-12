# TP-Link M7350

Rayhunter is currently working on support for the TP-Link M7350. This
device supports many more frequency bands than the Orbic RC400L, meaning it
works in the EU, for example.

You can get it [on
Ebay](https://www.ebay.com/sch/i.html?_nkw=tp-link+m7350&_sacat=0&_from=R40&_trksid=p4432023.m570.l1313)
on Amazon, but particularly in the EU it is often significantly cheaper
second-hand on local forums, ranging anywhere from 15 EUR to 50 EUR (used)

As of 0.2.8, the official Rayhunter release contains a
"rayhunter-daemon-tplink" binary that can be manually installed onto the
device. Work on an official installer like `install.sh` is in progress.

For information on manual installation see
[rayhunter-tplink-m7350](https://github.com/m0veax/rayhunter-tplink-m7350/)

## Hardware versions

The TP-Link comes in many different *hardware versions*. You can find the
hardware version of each device under the battery or next to the barcode on the
outer packaging, for example `V3.0` or `V5.2`. Support for installation varies:

* `1.0-2.0`: Not tested, probably impossible to obtain anymore (even second-hand)
* `3.0`, `3.2`, `5.0`, `5.2`, `7.0`, `8.0`: Tested, no issues.
* `9.0`: Recording might be broken, could be fixed if there is demand.

Otherwise is mostly no difference to the user, except that versions after `3.0`
have a color display.

## Display states

If your device has a color display, Rayhunter will show the same
red/green/white line at the top of the display as it does on Orbic, each color
meaning "warning"/"recording"/"paused" respectively.

If your device has a one-bit (black-and-white) display, Rayhunter will instead
show an emoji to indicate status. `!` means "warning", `:)` (smiling) means
"recording", `:` (face with no mouth) means "paused".
