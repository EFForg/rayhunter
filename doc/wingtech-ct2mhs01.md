# Wingtech CT2MHS01

The Wingtech CT2MHS01 hotspot is a Qualcomm mdm9650-based device with a screen available for US$15-35. This device is often used as a base platform for white labeled versions like the T-Mobile TMOHS1. AT&T branded versions of the hotspot seem to be the most abundant.

## Hardware
Wingtechs are abundant on ebay and can also be found on Amazon:
- https://www.amazon.com/AT-Turbo-Hotspot-256-Black/dp/B09YWLXVWT
- https://www.ebay.com/itm/135205906535
- https://www.ebay.com/itm/126987839936
- https://www.ebay.com/itm/127147132518

Rayhunter has been tested on 

```sh
WT_INNER_VERSION=SW_Q89323AA1_V057_M10_CRICKET_USR_MP
WT_PRODUCTION_VERSION=CT2MHS01_0.04.55
WT_HARDWARE_VERSION=89323_1_20
```

Please consider sharing the contents of your device's /etc/wt_version file here.

## Supported bands

There are likely variants of the device for all three ITU regions.

According to FCC ID 2APXW-CT2MHS01 Test Report No. I20N02441-RF-LTE, the ITU Region 2 American version of the device supports the following LTE bands:

| Band | Frequency        |
| ---- | ---------------- |
|    2 | 1900 MHz (PCS)   |
|    5 | 850 MHz (CLR)    |
|   12 | 700 MHz (LSMH)   |
|   14 | 700 MHz (USMH)   |
|   30 | 2300 MHz (WCS)   |
|   66 | 1700 MHz (E-AWS) |

Note that Band 5 (850 MHz, CLR) is suitable for roaming in ITU regions 2 and 3.

## Developing
The device has a framebuffer-driven screen at /dev/fb0 that behaves
similarly to the Orbic RC400L, although the userspace program
`displaygui` refreshes the screen significantly more often than on the
Orbic. This causes the green line on the screen to subtly flicker and
only be displayed during some frames. Subsequent work to fully control
the display without removing the OEM interface is desired.
