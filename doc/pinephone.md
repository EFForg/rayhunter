# PinePhone and PinePhone Pro

The PinePhone and PinePhone Pro both use a Qualcomm mdm9607 modem as part of their [Quectel EG25-G LTE module](https://www.quectel.com/product/lte-eg25-g/). The EG25-G has global LTE band support and contains a GNSS positioning module. Rayhunter does not currently make direct use of GNSS.

The modem is fully capable of running Rayhunter, but lacks both a screen and a network connection. The modem exposes an AT interface that can enable adb.

## Hardware
- <https://pine64.org/devices/pinephone/>
- <https://pine64.org/devices/pinephone_pro/>

## Supported bands

| Band | Frequency         |
| ---- | ----------------- |
|    1 | 2100 MHz (IMT)    |
|    2 | 1900 MHz (PCS)    |
|    3 | 1800 MHz (DCS)    |
|    4 | 1700 MHz (AWS-1)  |
|    5 | 850 MHz (CLR)     |
|    7 | 2600 MHz (IMT-E)  |
|    8 | 900 MHz (E-GSM)   |
|   12 | 700 MHz (LSMH)    |
|   13 | 700 MHz (USMH)    |
|   18 | 850 MHz (LSMH)    |
|   19 | 850 MHz (L800)    |
|   20 | 800 MHz (DD)      |
|   25 | 1900 MHz (E-PCS)  |
|   26 | 850 MHz (E-CLR)   |
|   28 | 700 MHz (APT)     |
|   38 | 2600 MHz (IMT-E)  |
|   39 | 850 MHz (E-CLR)   |
|   40 | 2300 MHz (S-Band) |
|   41 | 2500 MHz (BRS)    |

Note that the Quectel EG25-G does not support LTE band 48 (CBRS 3500MHz), used in the US for unlicensed 4G/5G connectivity.

## Installing
Download and extract the installer *on a shell on the PinePhone itself*. Unlike other Rayhunter installers, this has to be run on the device itself. Then run:

```sh
./installer pinephone
```

## Accessing Rayhunter
Because the modem does not have its own display or network interface, Rayhunter is only accessible on the pinephone by forwarding tcp over adb.

```sh
adb forward tcp:8080 tcp:8080
```

## Shell access
Use this command to enable adb access:

```sh
./installer util pinephone-start-adb
adb shell
```

## Power saving (disable adb)
The modem won't be able to sleep (power save) with adb enabled, even if Rayhunter is stopped. Disable adb with the following command:

```sh
./installer util pinephone-stop-adb
```
