# TP-Link M7200

Supported in Rayhunter since approximately version 0.3.0.

The TP-Link M7200 is a cheaper copy of the [M7350](./tplink-m7350.md). Many of
the things described on the M7350 page (port triggers, power saving, bands
supported) apply to the M7200.

There is no official information on supported bands, and it may vary by local
SKU.

Compared to the M7350 it has much less storage (around 10 MB free), no SD-card
slot, and no display. Indicator LEDs are unsupported in Rayhunter.

## Hardware versions

| Hardware version | Status |
| ---------------- | ------ |
| `3.0` | **Tested, no known issues** |
| `4.0` | **Works on firmware 4.0.1.** Newer firmware breaks the installer, a manual downgrade is required ([issue](https://github.com/EFForg/rayhunter/issues/1157)) |

You can find the exact hardware version under the battery or next to the
barcode on the outer packaging, for example `V3.0`.

When filing bug reports, particularly with the installer, please always specify
the exact hardware version and firmware version.

## Installing

To install, run `installer tplink --skip-sdcard`. Like on the M7350 this will
guide you to open the admin page and enter your password.
