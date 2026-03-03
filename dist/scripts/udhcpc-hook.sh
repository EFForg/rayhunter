#!/bin/sh
# udhcpc hook script for rayhunter WiFi client mode.
# Saves DHCP lease info (gateway, DNS) so the daemon can read the real
# gateway even when subnets collide. Routing is handled by the daemon.
#
# Deployed to /data/rayhunter/udhcpc-hook.sh by the installer.
# Any installer that adds wifi-client support must also deploy this script.
LEASE_FILE="/data/rayhunter/dhcp_lease"

case "$1" in
    bound|renew)
        ip addr flush dev "$interface"
        ip addr add "$ip/$mask" dev "$interface"
        echo "gateway=$router" > "$LEASE_FILE"
        echo "dns=$dns" >> "$LEASE_FILE"
        ;;
    deconfig)
        ip addr flush dev "$interface"
        rm -f "$LEASE_FILE"
        ;;
esac
