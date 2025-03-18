#v3

Root device using open.sh

https://github.com/m0veax/tplink_m7350/blob/main/open.sh

activate adb:

https://github.com/m0veax/tplink_m7350?tab=readme-ov-file#start-adbd

mount sd card:

https://github.com/m0veax/tplink_m7350?tab=readme-ov-file#sdcard

download dist pack from release tab in original repo

https://github.com/EFForg/rayhunter/releases

open 2 terminals

adb shell in one terminal (prefix *adb*)
terminal with path of extracted tar in another one (prefix *terminal*)

*adb*: `mkdir -p /data/rayhunter`

*terminal*: `vi config.toml.example` -> change path for logs to `/mnt/card/qmdl`

*terminal*: `adb push config.toml.example /tmp/config.toml`

*adb*: `mv /tmp/config.toml /data/rayhunter`

*terminal*: `adb push rayhunter-daemon /mnt/rayhunter-daemon`

*terminal*: `adb push scripts/rayhunter_daemon /tmp/rayhunter_daemon`

*adb*: `mv /tmp/rayhunter_daemon /etc/init.d/rayhunter_daemon`

*adb*: `vi /etc/init.d/rayhunter_daemon` -> replace string `/data/rayhunter/rayhunter-daemon` with `/mnt/card/rayhunter-daemon`

*terminal*: `adb push scripts/misc-daemon /etc/init.d/misc-daemon`

*adb*: `chmod 755 /etc/init.d/rayhunter-daemon`
*adb*: `chmod 755 /etc/init.d/misc-daemon`

*adb*: `/etc/init.d/misc-daemon`

rayhunter-daemon error message appears

*adb*: `/mnt/rayhunter-daemon /data/rayhunter/config.toml`

# v4

used arch while installing

root device:

```bash
sudo pacman -S ruby
git clone git@github.com:ecdsa521/tpown.git
cd tpown
gem install bundler
bundle config path ~/.config/.ruby_gems
mkdir -p ~/.config/.ruby_gems
bundle install
ruby tp.rb -t 192.168.0.1 -p admin
```

telnet into the device and activate adb

```
local $>nc 192.168.0.1 23
#/> usb_composition
902B
n
y
y
n
#/> exit
local $> adb shell
#/>

```
