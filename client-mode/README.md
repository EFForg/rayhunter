# WiFi Client Mode for Rayhunter (Orbic RC400L)

Connect the Orbic to an existing WiFi network while keeping its AP running.
This enables internet access (for ntfy notifications, etc.) and allows
accessing the Rayhunter web UI from any device on your network.

## How It Works

The Orbic's QCA6174 supports concurrent AP + station mode. `wlan0` runs
the AP (via hostapd/QCMAP), and `wlan1` is configured as a station using
a cross-compiled `wpa_supplicant`.

## Quick Start

1. Build wpa_supplicant (one-time):
   ```
   cd tools/build-wpa-supplicant
   docker build --platform linux/amd64 --target export --output type=local,dest=./out .
   ```

2. Push files to device:
   ```
   sh client-mode/scripts/setup-device.sh
   ```

3. Set credentials via the Rayhunter web UI (Settings > WiFi Client Mode),
   or via the installer:
   ```
   ./installer orbic --admin-password YOUR_PASS --wifi-ssid MyNetwork --wifi-password MyPass
   ```

4. Reboot. WiFi client starts automatically. Check the log:
   ```
   adb shell cat /tmp/wifi-client.log
   ```

## File Layout on Device

```
/data/rayhunter/
  bin/wpa_supplicant     # Static ARMv7 binary
  bin/wpa_cli            # Static ARMv7 binary
  scripts/wifi-client.sh # Main script (start/stop/status)
  wifi-creds.conf        # Credentials (ssid=X / password=Y)
```

## What the Script Does

1. Waits for wlan1 to appear (up to 30s)
2. Sets wlan1 to managed mode, starts wpa_supplicant
3. Obtains IP via DHCP
4. Fixes routing: replaces bridge0 default route, adds policy routing
   (table 100) so replies from wlan1's IP always exit via wlan1
5. Sets DNS to 8.8.8.8
6. Configures iptables: allows inbound on wlan1, blocks outbound except
   ESTABLISHED/RELATED, DHCP, DNS, and HTTPS (port 443 for ntfy)

## AT+SYSCMD

Commands needing `CAP_NET_ADMIN` (iw, iptables, ip rule) cannot run through
rootshell -- ADB's capability bounding set is too restrictive. The init
script triggers wifi-client.sh which runs with full capabilities.

Key constraint: AT+SYSCMD via `/dev/smd8` is **one-shot per boot**. The
installer uses USB bulk transfers and can send multiple commands.

## Disabling

Delete or rename the credentials file, then reboot:
```
adb shell "mv /data/rayhunter/wifi-creds.conf /data/rayhunter/wifi-creds.conf.disabled"
```

All network changes are runtime-only -- a reboot always restores defaults.

## Troubleshooting

Check the log first: `adb shell cat /tmp/wifi-client.log`

- **No log file**: wifi-client.sh didn't run. Check that wifi-creds.conf
  exists and the init script has the PRESTART replacement.
- **wpa_supplicant connects but no IP**: Check udhcpc uses
  `-s /etc/udhcpc.d/50default`.
- **Can't reach device from LAN**: Likely a policy routing issue. The
  script handles this, but if bridge0 and wlan1 share a subnet
  (both 192.168.1.0/24), check `ip rule show` and `ip route show table 100`.
