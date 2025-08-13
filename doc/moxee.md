# KonnectONE Moxee Hotspot (K779HSDL)

Supported in Rayhunter since version 0.6.0.

The Moxee Hotspot is a device very similar to the Orbic RC400L. It seems to be
primarily for the US market.

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
./installer orbic-network
```

The installation will ask you to log into the admin UI using a custom URL. The
password for that is under the battery.

## Obtaining a shell

```sh
./installer util orbic-start-telnet
```
