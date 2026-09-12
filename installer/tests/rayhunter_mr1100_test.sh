#!/bin/sh
# Focused, host-safe regression checks. The service itself is not started.
set -eu
SCRIPT=${1:-"$(dirname "$0")/../../dist/scripts/rayhunter_mr1100"}
fail() { echo "FAIL: $*" >&2; exit 1; }
has() { grep -F -- "$1" "$SCRIPT" >/dev/null || fail "missing: $1"; }
not_has() { ! grep -F -- "$1" "$SCRIPT" >/dev/null || fail "forbidden: $1"; }

sh -n "$SCRIPT" || fail "shell syntax"
usage=$("$SCRIPT" invalid 2>&1) && fail "invalid action succeeded"
[ "$?" -eq 2 ] || fail "invalid action status"
[ "$usage" = "Usage: $SCRIPT {start|stop|restart|status}" ] || fail "usage contract"

has 'ROOT=/data/rayhunter'
has 'RUN=/media/ram/rayhunter-mr1100'
has 'CHAIN=RH_MR1100'
has 'mounted ubi0:usrfs /data ubifs'
has 'mounted tmpfs /media/ram tmpfs'
has '[ "$(cat "$MARKER" 2>/dev/null)" = mr1100-v1 ]'
has '"$fw" -C "$CHAIN" -i lo -j ACCEPT'
has '"$fw" -C "$CHAIN" -m physdev --physdev-in rndis0 -j ACCEPT'
has '"$fw" -C "$CHAIN" -j REJECT'
has 'for port in 8080 23; do'
has '"$fw" -C INPUT -p tcp --dport "$port" -j "$CHAIN"'
has 'if [ -e /proc/net/if_inet6 ]; then'
not_has 'if [ -s /proc/net/if_inet6 ]; then'
has '"$fw" -I INPUT 1 -p tcp --dport "$port" -j "$CHAIN"'
has 'LOG=$RUN/rayhunter.log'
has 'exact_pid "$pid" || { msg "PID file refers to another process"; return 1; }'
has 'kill -INT "$pid"'
has 'kill -KILL "$pid"'
has '[ "$n" -lt 10 ]'
has 'mkdir "$LOCK"'
has 'sleep 30; exact_pid "$pid" || break'
not_has 'iptables -F'
not_has 'ip6tables -F'
not_has '/etc/init.d/'
not_has 'start-stop-daemon'

has 'S88rayhunter_mr1100_firewall) firewall; exit $?;;'
has 'abort_spawn; return 1'

# Exercise a PID-file write failure with an ordinary host sleep process.
# No device paths, firewall commands, or network sockets are used.
tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT HUP INT TERM
awk 'index($0, "case \"${0##*/}\" in") == 1 { exit } { print }' "$SCRIPT" > "$tmp/functions"
(
    . "$tmp/functions"
    LOG=$tmp/log
    PIDFILE=$tmp/missing/pid
    DAEMON=$(command -v sleep)
    CONFIG=30
    firewall() { return 0; }
    read_pid() { return 1; }
    exact_pid() { kill -0 "$1" 2>/dev/null; }
    if start_locked; then fail "PID-file failure reported success"; fi
    wait "$pid" 2>/dev/null || true
    if kill -0 "$pid" 2>/dev/null; then fail "new daemon survived PID-file failure"; fi
)

echo "PASS: rayhunter_mr1100 (including PID failure cleanup)"
