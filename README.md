![Rayhunter Logo - An Orca taking a bite out of a cellular signal bar](https://www.eff.org/files/styles/media_browser_preview/public/banner_library/rayhunter-banner.png)

# Rayhunter

![Tests](https://github.com/EFForg/rayhunter/actions/workflows/check-and-test.yml/badge.svg)

Rayhunter is an IMSI Catcher Catcher for the Orbic mobile hotspot.

**THIS CODE IS A PROOF OF CONCEPT AND SHOULD NOT BE RELIED UPON IN HIGH RISK SITUATIONS!**

## The Hardware

Rayhunter has been built and tested for the Orbic RC400L mobile hotspot. It may work on other Orbics and other
Linux/Qualcom devices, but this is the only one we have tested on.
You can buy the orbic [using bezos bucks](https://www.amazon.com/Orbic-Verizon-Hotspot-Connect-Enabled/dp/B08N3CHC4Y),
or on [eBay](https://www.ebay.com/sch/i.html?_nkw=orbic+rc400l).

## Setup (Silicon Mac, Linux)

1. Download the latest `release.tar` from the [Rayhunter releases page](https://github.com/EFForg/rayhunter/releases)
2. Unzip the `release.tar`. Open the terminal and navigate to the folder

    ```bash
    cd ~/Downloads/release
    ```

3. Turn on the Orbic device by holding the power button for 3 seconds. Plug it into your computer using a USB-C Cable.
4. Run the install script for your operating system:

    ```bash
    ./install.sh
    ```

    The device will restart multiple times over the next few minutes.

    You will know it is done when you see terminal output that says `checking for rayhunter server...success!`

5. Rayhunter should now be running! You can verify this by following the instructions below to [view the web UI](#usage-viewing-the-web-ui). You should also see a green line flash along the top of top the display on the device.

### Installation Notes

* Note: If you are installing from the cloned GitHub repository please see the development instructions below, running `install.sh` from the git tree will not work.
* The install script has only been tested for Linux on the latest version of Ubuntu. If it fails you will need to follow the install steps outlined in **Development** below.
* On macOS if you encounter an error that says "No Orbic device found," it may because you the "Allow accessories to connect" security setting set to "Ask for approval." You may need to temporarily change it to "Always" for the script to run. Make sure to change it back to a more secure setting when you're done.

## Setup (Intel Mac, Windows)

* **Intel Mac:** The install script also won't work on older Macs with Intel chips, for those Macs you will need to follow the [instructions for installing Rayhunter on Intel Macs](https://github.com/EFForg/rayhunter/wiki/Install-Rayhunter-on-Mac-Intel-devices)
* **Windows:** We don't currently support automated installs on Windows, you will have to follow the instructions in the **Development** section below.

## Updating

Great news: if you've successfully installed rayhunter, you already know how to update it! Our update process is identical to the setup process: simply download the latest release and follow the steps in the [setup section](#setup-silicon-mac-linux).

## Usage (viewing the web UI)

Once installed, Rayhunter will run automatically whenever your Orbic device is running. It serves a web UI that provides some basic controls, such as being able to start/stop recordings, download captures, and view heuristic analyses of captures. You can access this UI in one of two ways:

1. **Connect over wifi:** Connect your phone/laptop to the Orbic's 2.4GHz wifi network and visit [http://192.168.1.1:8080](http://192.168.1.1:8080). (Click past your browser warning you about the connection not being secure, Rayhunter doesn't have HTTPS yet).
    * You can find the wifi network password by going to the Orbic's menu > 2.4 GHz WIFI Info > Enter > find the 8-character password next to the lock 🔒 icon.
2. **Connect over USB:** Connect the Orbic device to your laptop via USB. Run `adb forward tcp:8080 tcp:8080`, then visit [http://localhost:8080](http://localhost:8080).
    * For this you will need to install the Android Debug Bridge (ADB) on your computer, you can copy the version that was downloaded inside the `releases/platform-tools/` folder to somewhere else in your path or you can install it manually.
    * You can find instructions for doing so on your platform [here](https://www.xda-developers.com/install-adb-windows-macos-linux/#how-to-set-up-adb-on-your-computer), (don't worry about instructions for installing it on a phone/device yet).
    * On macOS, the easiest way to install ADB is with Homebrew: First [install Homebrew](https://brew.sh/), then run `brew install android-platform-tools`.

## Frequently Asked Questions

### Do I need an active SIM card to use Rayhunter?

**It Depends**. Operation of Rayhunter does require the insertion of a SIM card into the device, but whether that SIM card has to be currently active for our tests to work is still under investigation. If you want to use the device as a hotspot in addition to a research device an active plan would of course be necessary, however we have not done enough testing yet to know whether an active subscription is required for detection. If you want to test the device with an inactive SIM card, we would certainly be interested in seeing any data you collect, and especially any runs that trigger an alert!

### Help, Rayhunter's line is red! What should I do?

Unfortunately, the circumstances that might lead to a positive cell site simulator (CSS) signal are quite varied, so we don't have a universal recommendation for how to deal with the a positive signal. Depending on your circumstances and threat model, you may want to turn off your phone until you are out of the area (or put it on airplane mode) and tell your friends to do the same!

If you've received a Rayhunter warning and would like to help us with our research, please send your Rayhunter data captures (QMDL and PCAP logs) to us at our [Signal](https://signal.org/) username [**ElectronicFrontierFoundation.90**](https://signal.me/#eu/HZbPPED5LyMkbTxJsG2PtWc2TXxPUR1OxBMcJGLOPeeCDGPuaTpOi5cfGRY6RrGf) with the following information: capture date, capture location, device, device model, and Rayhunter version. If you're unfamiliar with Signal, feel free to check out our [Security Self Defense guide on it](https://ssd.eff.org/module/how-to-use-signal).

Please note that this file may contain sensitive information such as your IMSI and the unique IDs of cell towers you were near which could be used to ascertain your location at the time.

### Does Rayhunter work outside of the US?

**Probably**. Some Rayhunter users have reported successfully using it in other countries with unlocked devices and SIM cards from local telcos. We can't guarantee whether or not it will work for you though.

### Should I get a locked or unlocked orbic device? What is the difference?

If you want to use a non-Verizon SIM card you will probably need an unlocked device. But it's not clear how locked the locked devices are nor how to unlock them, we welcome any experimentation and information regarding the use of unlocked devices.

### Does Rayhunter work on any other devices besides the Orbic RC400L?

**Maybe**. We have not tested Rayhunter on any other hardware but we would love to expand the supported platforms. We will consider giving official support to any hardware platform that can be bought for around $20-30USD. The Rayhunter daemon should theoretically work on any Linux/Android device that has a qualcomm chip with a `/dev/diag` interface and root access, though our installer script has only been tested with an Orbic. If you get it working on another device, please let us know!

### How do I delete capture files from the Rayhunter device?

You can get a shell on the device by inputting `adb shell` to a terminal with the device connected, you can check if it is detected with `adb devices`.
The capture files are located at */data/rayhunter/qmdl* but you will need root access to modify or delete them. From the adb shell run `/bin/rootshell` and you can now use commands like 'rm' as root to modify and delete entries in the */data/rayhunter/qmdl* directory. **Be careful not to delete important files in other directories as you may seriously damage the device**

## Development

Follow these instructions if you need to build Rayhunter from source rather than using our [compiled builds](https://github.com/EFForg/rayhunter/releases).

* Install ADB on your computer using the instructions above, and make sure it's in your terminal's PATH
  * You can verify if ADB is in your PATH by running `which adb` in a terminal. If it prints the filepath to where ADB is installed, you're set! Otherwise, try following one of these guides:
    * [linux](https://askubuntu.com/questions/652936/adding-android-sdk-platform-tools-to-path-downloaded-from-umake)
    * [macOS](https://www.repeato.app/setting-up-adb-on-macos-a-step-by-step-guide/)
    * [Windows](https://medium.com/@yadav-ajay/a-step-by-step-guide-to-setting-up-adb-path-on-windows-0b833faebf18)

### If you're on x86 linux

Install Rust the usual way and then install cross compiling dependences:

```bash
sudo apt install curl build-essential libc6-armhf-cross libc6-dev-armhf-cross gcc-arm-linux-gnueabihf
rustup target add x86_64-unknown-linux-gnu
rustup target add armv7-unknown-linux-gnueabihf
```

Now you can root your device and install Rayhunter by running `./tools/install-dev.sh`

### If you're on Windows or can't run the install scripts

* Root your device on Windows using the instructions here: <https://xdaforums.com/t/resetting-verizon-orbic-speed-rc400l-firmware-flash-kajeet.4334899/#post-87855183>

* Build for arm using `cargo build`

* Run tests using `cargo test_pc`

* Push the scripts in `scripts/` to `/etc/init.d` on device and make a directory called `/data/rayhunter` using `adb shell` (and sshell for your root shell if you followed the steps above)

* you also need to copy `config.toml.example` to `/data/rayhunter/config.toml`

* Then run `./make.sh` this will build the binary and push it over adb. Restart your device or run `/etc/init.d/rayhunter_daemon start` on the device and you are good to go.

* Write your code and write tests

* Build for arm using `cargo build`

* Run tests using `cargo test_pc`

* push to the device with `./make.sh`

## Support and Discussion

If you're having issues installing or using Rayhunter, please open an issue in this repo. Join us in the `#rayhunter` channel of [EFF's Mattermost](https://opensource.eff.org/signup_user_complete/?id=6iqur37ucfrctfswrs14iscobw&md=link&sbr=su) instance to chat!

## Documentation

* Build docs locally using `RUSTDOCFLAGS="--cfg docsrs" cargo doc --no-deps --all-features  --open`

**LEGAL DISCLAIMER:** Use this program at your own risk. We believe running this program does not currently violate any laws or regulations in the United States. However, we are not responsible for civil or criminal liability resulting from the use of this software. If you are located outside of the US please consult with an attorney in your country to help you assess the legal risks of running this program.

*Good Hunting!*
