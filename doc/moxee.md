# KonnectONE Moxee Hotspot (K779HSDL)

Supported in Rayhunter since version 0.6.0.

The Moxee Hotspot is a device very similar to the Orbic RC400L. It seems to be
primarily for the US market.

<div class="warning-box">

**WARNING: These devices are known to become completely bricked by installing Rayhunter.**

Do not buy this device nor try to install _nor upgrade_ Rayhunter on it.

We're still trying to figure out what's wrong in [this discussion](https://github.com/EFForg/rayhunter/issues/865).

</div>

- [KonnectONE product page](https://www.konnectone.com/specs-hotspot)
- [Moxee product page](https://www.moxee.com/hotspot)

## Supported bands

According to [FCC ID 2APQU-K779HSDL](https://fcc.report/FCC-ID/2APQU-K779HSDL), the device supports the following LTE bands:

| Band | Frequency               |
|------|-------------------------|
| 2    | 1900 MHz (PCS)          |
| 4    | 1700/2100 MHz (AWS-1)   |
| 5    | 850 MHz (CLR)           |
| 12   | 700 MHz (Lower SMH)     |
| 13   | 700 MHz (Upper SMH)     |
| 25   | 1900 MHz (Extended PCS) |
| 26   | 850 MHz (Extended)      |
| 41   | 2500 MHz (TDD)          |
| 66   | 1700/2100 MHz (E-AWS)   |
| 71   | 600 MHz                 |

## Installation

Connect to the hotspot's network using WiFi or USB tethering and run:

```sh
./installer moxee --admin-password 'mypassword'
```

The password (in place of `mypassword`) is under the battery.

`./installer moxee` is almost the same as `./installer orbic`, it just comes
with slightly better defaults that will give you more space for recordings.

## Obtaining a shell

```sh
./installer util orbic-shell
```
