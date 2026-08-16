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

The forward belongs to the adb connection to the modem, so it is dropped whenever that connection
is re-established — for example after the modem resets. When this happens `adb devices` still lists
the modem and Rayhunter keeps recording, but the web UI stops answering. Re-run the `adb forward`
command to get it back:

```sh
adb forward --list          # empty means the forward is gone
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

## Toggling adb resets the modem

Both `pinephone-start-adb` and `pinephone-stop-adb` change the modem's USB composition with
`AT+QCFG="usbcfg"`, and the modem resets whenever that value is written — even if the requested
composition is the one already in use. The modem then needs roughly a minute to boot, during which
the phone has **no cellular service**.

Take this into account when scripting the commands: only enable adb when it is actually absent, and
give the modem time to come back before trying again. Calling `pinephone-start-adb` in a retry loop
that is faster than the modem's boot time keeps the modem in a permanent reset cycle. In that state
ModemManager never completes its QMI probe (`port cdc-wdm0 timed out N consecutive times`, then
`modem couldn't be initialized: Failed to load current capabilities`), and the phone loses mobile
data entirely until the modem is left alone long enough to finish booting.

## `Resource busy` when enabling adb

On distributions where ModemManager (or another modem daemon such as `eg25-manager`) manages the
EG25-G, it claims the AT interface that the installer needs, and enabling adb fails:

```text
Failed to start adb on the PinePhone's modem

Caused by:
    0: detach_and_claim_interface({USB_INTERFACE_NUMBER}) failed
    1: Resource busy (os error 16)
```

Stop the daemon for the duration of the call and start it again afterwards. The modem resets as a
result of the usbcfg write anyway, so ModemManager re-probes it when it comes back:

```sh
sudo systemctl stop ModemManager
sudo ./installer util pinephone-start-adb
sudo systemctl start ModemManager
```
