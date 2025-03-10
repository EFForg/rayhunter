Root device using open.sh

https://github.com/m0veax/tplink_m7350/blob/main/open.sh

activate adb:

https://github.com/m0veax/tplink_m7350?tab=readme-ov-file#start-adbd

download dist pack from release tab in original repo

https://github.com/EFForg/rayhunter/releases

open 2 terminals

adb shell in one terminal (prefix *adb*)
terminal with path of extracted tar in another one (prefix *terminal*)

*adb*: mkdir -p /data/rayhuntter

*terminal*: adb push config.toml.example /tmp/config.toml

*adb*: mv /tmp/config.toml /data/rayhunter

*terminal*: adb push rayhunter-daemon /dev/shm/rayhunter-daemon

*terminal*: adb push scripts/rayhunter_daemon /tmp/rayhunter_daemon

*adb*: mv /tmp/rayhunter_daemon /etc/init.d/rayhunter_daemon

*adb*: vi /etc/init.d/rayhunter_daemon -> replace string "/data/rayhunter/rayhunter-daemon" with /dev/shm/rayhunter-daemon

*terminal*: adb push scripts/misc-daemon /etc/init.d/misc-daemon

*adb*: chmod 755 /etc/init.d/rayhunter_daemon
*adb*: chmod 755 /etc/init.d/misc-daemon

*adb*: /etc/init.d/misc-daemon

rayhunter-daemon error message appears

*adb*: /dev/shm/rayhunter-daemon /data/rayhunter/config.toml