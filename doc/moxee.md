# Moxee Hotspot

Supported in Rayhunter since version 0.6.0.

The [Moxee Hotspot](https://www.moxee.com/hotspot) is a device very similar to
the Orbic RC400L. It seems to be primarily for the US market.

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
