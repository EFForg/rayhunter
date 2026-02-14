#!/bin/sh
# WiFi client mode for Rayhunter - connects wlan1 to an existing network
# Reads credentials from /data/rayhunter/wifi-creds.conf
# Format:
#   ssid=YourNetworkName
#   password=YourPassword

LOG="/tmp/wifi-client.log"
exec > "$LOG" 2>&1

CRED_FILE="/data/rayhunter/wifi-creds.conf"
WPA_BIN="/data/rayhunter/bin/wpa_supplicant"
WPA_CONF="/tmp/wpa_sta.conf"
WPA_PID="/tmp/wpa_sta.pid"
DHCP_PID="/tmp/udhcpc_wlan1.pid"
IFACE="wlan1"
RT_TABLE=100

stop() {
    [ -f "$WPA_PID" ] && kill "$(cat "$WPA_PID")" 2>/dev/null && rm -f "$WPA_PID"
    [ -f "$DHCP_PID" ] && kill "$(cat "$DHCP_PID")" 2>/dev/null && rm -f "$DHCP_PID"
    ip link set "$IFACE" down 2>/dev/null
}

start() {
    if [ ! -f "$CRED_FILE" ]; then
        echo "No credentials file at $CRED_FILE"
        exit 1
    fi

    SSID=$(grep '^ssid=' "$CRED_FILE" | cut -d= -f2-)
    PSK=$(grep '^password=' "$CRED_FILE" | cut -d= -f2-)

    if [ -z "$SSID" ] || [ -z "$PSK" ]; then
        echo "Missing ssid or password in $CRED_FILE"
        exit 1
    fi

    # Wait for the wireless interface to appear (created asynchronously by QCMAP/hostapd)
    for i in $(seq 1 30); do
        [ -d "/sys/class/net/$IFACE" ] && break
        [ "$i" = "1" ] && echo "Waiting for $IFACE..."
        sleep 1
    done
    if [ ! -d "/sys/class/net/$IFACE" ]; then
        echo "$IFACE not found after 30s, giving up"
        exit 1
    fi

    stop 2>/dev/null
    sleep 1

    echo "Configuring $IFACE for station mode"
    iw dev "$IFACE" set type managed
    ip link set "$IFACE" up

    cat > "$WPA_CONF" <<WPAEOF
ctrl_interface=/var/run/wpa_supplicant
network={
    ssid="$SSID"
    psk="$PSK"
    key_mgmt=WPA-PSK
}
WPAEOF

    echo "Starting wpa_supplicant"
    "$WPA_BIN" -i "$IFACE" -Dnl80211 -c "$WPA_CONF" -B -P "$WPA_PID"
    sleep 5

    echo "wpa_supplicant status:"
    iw dev "$IFACE" link

    echo "Starting DHCP"
    udhcpc -i "$IFACE" -s /etc/udhcpc.d/50default -p "$DHCP_PID" -t 10 -A 3 -b
    sleep 3

    WLAN1_IP=$(ip addr show "$IFACE" | grep 'inet ' | awk '{print $2}' | cut -d/ -f1)
    WLAN1_CIDR=$(ip addr show "$IFACE" | grep 'inet ' | awk '{print $2}')
    WLAN1_SUBNET=$(ip route show dev "$IFACE" | grep 'proto kernel' | awk '{print $1}')
    WLAN1_GW=$(ip route show dev "$IFACE" | grep 'proto kernel' | awk '{print $1}' | cut -d/ -f1)
    WLAN1_GW="${WLAN1_GW%.*}.1"

    if [ -z "$WLAN1_IP" ]; then
        echo "Failed to get IP on $IFACE"
        exit 1
    fi

    echo "IP: $WLAN1_IP  Subnet: $WLAN1_SUBNET  CIDR: $WLAN1_CIDR  Gateway: $WLAN1_GW"

    # Fix default route: ensure it goes through wlan1, not bridge0
    GATEWAY=$(ip route show default | grep "dev bridge0" | awk '{print $3}')
    if [ -n "$GATEWAY" ]; then
        echo "Fixing default route: bridge0 -> wlan1"
        ip route del default dev bridge0 2>/dev/null
    fi
    ip route replace default via "$WLAN1_GW" dev "$IFACE" metric 10

    # Policy routing: force traffic from our DHCP IP out wlan1
    # (needed because bridge0 shares the same subnet)
    ip rule del from "$WLAN1_IP" table $RT_TABLE 2>/dev/null
    ip route flush table $RT_TABLE 2>/dev/null
    ip rule add from "$WLAN1_IP" table $RT_TABLE
    ip route add "$WLAN1_SUBNET" dev "$IFACE" src "$WLAN1_IP" table $RT_TABLE
    ip route add default via "$WLAN1_GW" dev "$IFACE" table $RT_TABLE

    echo "nameserver 8.8.8.8" > /etc/resolv.conf

    # Allow inbound traffic on wlan1
    iptables -I INPUT -i "$IFACE" -j ACCEPT
    iptables -I FORWARD -i "$IFACE" -j ACCEPT

    # Block stock Orbic daemons from phoning home (dmclient, upgrade, etc.)
    # Allow only: replies to incoming connections, DHCP renewal, DNS, and HTTPS
    # (needed for ntfy notifications).
    iptables -A OUTPUT -o "$IFACE" -m state --state ESTABLISHED,RELATED -j ACCEPT
    iptables -A OUTPUT -o "$IFACE" -p udp --dport 67:68 -j ACCEPT
    iptables -A OUTPUT -o "$IFACE" -p udp --dport 53 -j ACCEPT
    iptables -A OUTPUT -o "$IFACE" -p tcp --dport 53 -j ACCEPT
    iptables -A OUTPUT -o "$IFACE" -p tcp --dport 443 -j ACCEPT
    iptables -A OUTPUT -o "$IFACE" -j DROP

    echo 0 > /proc/sys/net/bridge/bridge-nf-call-iptables

    echo "=== iptables OUTPUT ==="
    iptables -L OUTPUT -v -n 2>&1

    echo "=== policy routing ==="
    ip rule show
    echo "--- table $RT_TABLE ---"
    ip route show table $RT_TABLE

    echo "=== network state ==="
    ip addr show "$IFACE" | grep 'inet '
    ip route show

    echo "Internet test:"
    wget -q -O /dev/null http://detectportal.firefox.com/success.txt && echo "OK" || echo "FAILED"
}

status() {
    if [ -f "$WPA_PID" ] && kill -0 "$(cat "$WPA_PID")" 2>/dev/null; then
        ip addr show "$IFACE" | grep 'inet ' | awk '{print $2}'
    else
        echo "disconnected"
        return 1
    fi
}

case "$1" in
    start) start ;;
    stop) stop ;;
    status) status ;;
    *) echo "Usage: $0 {start|stop|status}" >&2; exit 1 ;;
esac
