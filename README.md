![Rayhunter Logo - An Orca taking a bite out of a cellular signal bar](https://www.eff.org/files/styles/media_browser_preview/public/banner_library/rayhunter-banner.png)
# Rayhunter for TP-Link M7350

![Tests](https://github.com/EFForg/rayhunter/actions/workflows/check-and-test.yml/badge.svg)

Rayhunter is an IMSI Catcher Catcher originally developed for the Orbic mobile hotspot (available in U. S. market) and now ported to TP-Link M7350 mobile hotspot (available in European market and worldwide).

**THIS CODE IS A PROOF OF CONCEPT AND SHOULD NOT BE RELIED UPON IN HIGH RISK SITUATIONS!**

## The Hardware

Rayhunter has been built and tested for the Orbic RC400L mobile hotspot. Currently is being ported to TP-Link M7350 mobile hotspot and already works on the several hardware revisions.
It may work on other orbics and other linux/qualcom devices, but this is the only one we have tested on.
Buy the orbic [using bezos bucks](https://www.amazon.com/Orbic-Verizon-Hotspot-Connect-Enabled/dp/B08N3CHC4Y),
or on [Ebay](https://www.ebay.com/sch/i.html?_nkw=orbic+rc400l).

TP-Link M7350 needs to be rooted first, then Rayhunter can be installed on it.

| HW revision | rooting  | Rayhunter  |
| :---:   | :---: | :---: |
| v1 | no info               | no info            |
| v2 | no info               | no info            |
| v3 | yes, with [open-v3.sh](https://github.com/m0veax/tplink_m7350/blob/main/open-v3.sh) script  | yes              |
| v3.2 | no info               | no info            |
| v4 | yes, with [open-v4.sh](https://github.com/m0veax/tplink_m7350/blob/main/open-v4.sh) script or [tpown](https://github.com/m0veax/rayhunter-tplink-m7350/blob/installer/PoC.md#v4) | yes ([with modifications](https://github.com/ping2A/rayhunter)) |
| v5 | yes, probably [tpown](https://github.com/m0veax/rayhunter-tplink-m7350/blob/installer/PoC.md#v4) | [no](https://github.com/m0veax/rayhunter-tplink-m7350/issues/2), but probably will, with modification) |
| v6.2 | yes               | yes ([with modifications](https://github.com/ping2A/rayhunter)) |
| v7 | no info               | no info |
| v8 | yes | yes ([with modifications](https://github.com/ping2A/rayhunter)) |


1. Download the latest [Rayhunter release bundle](https://github.com/EFForg/rayhunter/releases) and extract it.
**If you are installing from the cloned github repository please see the development instructions below, running `install.sh` from the git tree will not work.**
2. Turn on the Orbic device and plug it into your computer using a USB-C Cable.
3. On MacOS or Linux run the install script `install.sh`. 
4. Once finished, Rayhunter should be running! You can verify this by visiting the web UI as described below.

### Notes 
  * The install script has only been tested for Linux on the latest version of Ubuntu. If it fails you will need to follow the install steps outlined in **Development** below.
  * The install script also won't work on older macs with intel chips, for those macs you will need to follow the instructions at https://github.com/EFForg/rayhunter/wiki/Install-Rayhunter-on-Mac-Intel-devices
  * We don't currently support automated installs on windows, you will have to follow the manual install instructions below*

## Installing Rayhunter

TP-Link M7350 mobile hotspot needs to be rooted first. For rooting use one of the methods described in the table above. Also, you will need to install `ADB` first.

After that, Rayhunter binary can be installed on the device. Manual procedure is the following (we assume you are using (Ubuntu/Debian) Linux environment for installation).

Open two terminals:
- terminal 1 (adb shell)
- terminal 2 (local)

**Terminal 1 (adb shell)**:
```
adb shell
mkdir /data/rayhunter
```

Download release file from original EFF repository and unpack it to `~/rayhunter/release`: 
https://github.com/EFForg/rayhunter/releases/tag/v0.2.5

**Terminal 2 (local)**:
```
cd ~/rayhunter/release

nano config.toml.example
```

Now change path for logs to `/media/card/qmdl`. **You must have SD card (SDHC, 32 GB max!) inserted in the device**. Path should be:
```
qmdl_store_path = "/media/card/qmdl"
```

Now download the patch from ping2A:
``` 
mv rayhunter-daemon rayhunter-daemon-old
wget https://github.com/ping2A/rayhunter/releases/download/test/rayhunter-daemon
chmod +x rayhunter-daemon
```

Copy two files to the device:
```
adb push config.toml.example /data/rayhunter/config.toml
adb push rayhunter-daemon /media/card/rayhunter-daemon
```

**Terminal 1 (adb)**:
```
/media/card/rayhunter-daemon /data/rayhunter/config.toml
```

On your computer open web browser and go to `http://192.168.0.1:8080`.

It is recommended to disconnect your computer from WiFi or Ethernet (the only connection should be with USB cable to the TP-Link M7350 device). You also need to insert SIM card to the TP-link device.

![Image](https://github.com/user-attachments/assets/ce6df40c-c87d-4adf-ac91-24082643bdeb)

### Autostart Rayhunter on the device

Since you want that Rayhunter is autostarted at each boot of the TP-Link device, you need to set up `init.d` script.

First copy [this script from the repository](https://github.com/m0veax/rayhunter-tplink-m7350/blob/installer/dist_tplink/lighttpd) to your computer.

Connect to the device:
```
adb shell 
```

Copy the content of the file to a new file (named `lighttpd2`): 
```
cd /etc/init.d/
vi lighttpd2
```
In `vi` editor press `esc`, `:`,  `i` and then paste the text of the script with right click of the mouse.  `esc`, `w`,  `q` - write and save the file.

Now remove the old file and replace it with the new one:
```
rm lighttpd
mv lighttpd2 lighttpd
reboot
```

After reboot, Rayhunter should be autostarted automatically. You can visit Rayhunter WebUI with your web browser at `http://192.168.0.1:8080`.

## Usage

Once installed, Rayhunter will run automatically whenever your device is running. It serves a web UI that provides some basic controls, such as being able to start/stop recordings, download captures, and view heuristic analyses of captures. You can access this UI in one of two ways:

<<<<<<< HEAD
1. **Over WiFi**: Connect your phone/laptop to the device WiFi network and visit `http://192.168.0.1:8080` (click past your browser warning you about the connection not being secure, Rayhunter doesn't have HTTPS yet!)

2. **Over USB**: Connect the Orbic device to your laptop via USB, then visit `http://192.168.0.1:8080`. Maybe you need to run `adb forward tcp:8080 tcp:8080`, then visit `http://localhost:8080`. For this you will need to install the Android Debug Bridge (ADB) on your computer, you can copy the version that was downloaded inside the releases/platform-tools/` folder to somewhere else in your path or you can install it manually.  You can find instructions for doing so on your platform [here](https://www.xda-developers.com/install-adb-windows-macos-linux/#how-to-set-up-adb-on-your-computer), (don't worry about instructions for installing it on a phone/device yet).
=======
1. Over wifi: Connect your phone/laptop to the Orbic's wifi network and visit `http://192.168.1.1:8080` (click past your browser warning you about the connection not being secure, Rayhunter doesn't have HTTPS yet!).
    * Note that you'll need the Orbic's wifi password for this, which can be retrieved by pressing the "MENU" button on the device and opening the 2.4 GHz menu.
2. Over usb: Connect the Orbic device to your laptop via usb. Run `adb forward tcp:8080 tcp:8080`, then visit `http://localhost:8080`. For this you will need to install the Android Debug Bridge (ADB) on your computer, you can copy the version that was downloaded inside the releases/platform-tools/` folder to somewhere else in your path or you can install it manually.  You can find instructions for doing so on your platform [here](https://www.xda-developers.com/install-adb-windows-macos-linux/#how-to-set-up-adb-on-your-computer), (don't worry about instructions for installing it on a phone/device yet).
>>>>>>> 3b80df76a5bcec4ebeed29d1c3e6d2fc40040aee

## Frequently Asked Questions

### Do I need an active SIM card to use Rayhunter?
**It Depends**. Operation of Rayhunter does require the insertion of a SIM card into the device, but whether that SIM card has to be currently active for our tests to work is still under investigation. If you want to use the device as a hotspot in addition to a research device an active plan would of course be necessary, however we have not done enough testing yet to know whether an active subscription is required for detection. If you want to test the device with an inactive SIM card, we would certainly be interested in seeing any data you collect, and especially any runs that trigger an alert!
 
### Help, Rayhunter's line is red! What should I do?
Unfortunately, the circumstances that might lead to a positive CSS signal are quite varied, so we don't have a universal recommendation for how to deal with the a positive signal. You might also want to turn off your phone until you are out of the area (or put it on airplane mode,) and tell your friends to do the same!

 Please feel free to contact an EFF technologist with more information & a copy of the QMDL in question at [info@eff.org](mailto:info@eff.org). Please note that this file may contain sensetive information such as your IMSI and the unique IDs of cell towers you were near which could be used to ascertain your location at the time. We encourage you to use PGP encryption when sending your message. You can find the [PGP public key for info@eff.org here](https://www.eff.org/about/contact#main-content).
### Does Rayhunter work outside of the US?
**Probably**. Some Rayhunter users have reported successfully using it in other countries with unlocked devices and SIM cards from local telcos. We can't guarantee whether or not it will work for you though.
### Should I get a locked or unlocked orbic device? What is the difference?
If you want to use a non verizon SIM card you will probably need an unlocked device. But it's not clear how locked the locked devices are nor how to unlock them, we welcome any experimentation and information regarding the use of unlocked devices.
### Does Rayhunter work on any other devices besides the Orbic RC400L?
**Maybe**. We have not tested Rayhunter on any other hardware but we would love to expand the supported platforms. We will consider giving official support to any hardware platform that can be bought for around $20-30USD. The Rayhunter daemon should theoretically work on any linux/android device that has a qualcomm chip with a /dev/diag interface and root access, though our installer script has only been tested with an Orbic. If you get it working on another device, please let us know!

## Development (compiling Rayhunter binary)

**Under development - work in progress.**

Install `Rust` and cross compiling dependences:
```
sudo apt install curl build-essential libc6-armhf-cross libc6-dev-armhf-cross gcc-arm-linux-gnueabihf rustup cargo
rustup default stable
rustup target add x86_64-unknown-linux-gnu
rustup target add armv7-unknown-linux-gnueabihf
```

Clone the repository:
```
git clone https://github.com/NAME_OF_REPOSITORY
cd NAME_OF_REPOSITORY
```

Compile binary:
```
cargo build --target armv7-unknown-linux-gnueabihf --release
```
Compiled binaries are then in `target/armv7-unknown-linux-gnueabihf/release/`:
```
cd target/armv7-unknown-linux-gnueabihf/release/
file rayhunter-daemon

rayhunter-daemon: ELF 32-bit LSB executable, ARM, EABI5 version 1 (SYSV), statically linked, BuildID[sha1]=be93b0df122db6d577a2dd2da231d01ddb351ba9, for GNU/Linux 3.2.0, stripped
```

Building documentation locally:
`RUSTDOCFLAGS="--cfg docsrs" cargo doc --no-deps --all-features  --open`

Documentation is then in `target/doc/rayhunter/index.html`.

* Write your code and write tests

* Build for arm using `cargo build`

* Run tests using `cargo test_pc`

* push to the device with `./make.sh`

## Support and Discussion

If you're having issues installing or using Rayhunter, please open an issue in this repo. Join us in the `#rayhunter` channel of [EFF's Mattermost](https://opensource.eff.org/signup_user_complete/?id=6iqur37ucfrctfswrs14iscobw&md=link&sbr=su) instance to chat!

## Documentation
* Build docs locally using `RUSTDOCFLAGS="--cfg docsrs" cargo doc --no-deps --all-features  --open`

**LEGAL DISCLAIMER:** Use this program at your own risk. We believe running this program does not currently violate any laws or regulations in the United States. However, we are not responsible for civil or criminal liability resulting from the use of this software. If you are located outside of the US please consult with an attorney in your country to help you assess the legal risks of running this program.

The reason for that is, that this software uses Qualcomm DIAG kernel driver (`DIAG_CHAR`) to analyze **your own network traffic** that is processed by baseband chip on your device. So, there is **no interception of traffic** of other mobile subscribers and **no unauthorized firmware modifications** to the baseband chip which would normally require a new certification. Rayhunter just enables you to see and analyse all network traffic from the mobile network which is usually hidden from you.

*Good Hunting!*
