# Rayhunter

```
 @@@@@@@   @@@@@@  @@@ @@@ @@@  @@@ @@@  @@@ @@@  @@@ @@@@@@@ @@@@@@@@ @@@@@@@ 
 @@!  @@@ @@!  @@@ @@! !@@ @@!  @@@ @@!  @@@ @@!@!@@@   @@!   @@!      @@!  @@@
 @!@!!@!  @!@!@!@!  !@!@!  @!@!@!@! @!@  !@! @!@@!!@!   @!!   @!!!:!   @!@!!@! 
 !!: :!!  !!:  !!!   !!:   !!:  !!! !!:  !!! !!:  !!!   !!:   !!:      !!: :!! 
  :   : :  :   : :   .:     :   : :  :.:: :  ::    :     :    : :: :::  :   : :
                                                                                    
                                               
_      _      _      _      _      _      _      _
)`'-.,_)`'-.,_)`'-.,_)`'-.,_)`'-.,_)`'-.,_)`'-.,_)`'-.,_

                O          .
             O            ' '
               o         '   .
             o         .'
          __________.-'       '...___
       .-'                      ###  '''...__
      /   a###                 ##            ''--.._ ______
      '.                      #     ########        '   .-'
        '-._          ..**********####  ___...---'''\   '
            '-._     __________...---'''             \   l
                \   |                         apc     '._|
                 \__;
```
![Tests](https://github.com/EFForg/rayhunter/actions/workflows/check-and-test.yml/badge.svg)

Rayhunter is an IMSI Catcher Catcher for the Orbic mobile hotspot.

**THIS CODE IS PROOF OF CONCEPT AND SHOULD NOT BE RELIED UPON IN HIGH RISK SITUATIONS**

Code is built and tested for the Orbic RC400L mobile hotspot, it may work on other orbics and other 
linux/qualcom devices but this is the only one we have tested on. Buy the orbic [using bezos bucks](https://www.amazon.com/gp/product/B09CLS6Z7X/)

## Setup

*NOTE: We don't currently support automated installs on windows, you will have to follow the manual install instructions below*

1. Download the latest [rayhunter release bundle](https://github.com/EFForg/rayhunter/releases) and extract it.
2. Run the install script inside the bundle corresponding to your platform (`install-linux.sh`, `install-mac.sh`).
3. Once finished, rayhunter should be running! You can verify this by visiting the web UI as described below.

## Usage

Once installed, rayhunter will run automatically whenever your Orbic device is running. It serves a web UI that provides some basic controls, such as being able to start/stop recordings, download captures, and view heuristic analyses of captures. You can access this UI in one of two ways:

1. Over wifi: Connect your phone/laptop to the Orbic's wifi network and visit `http://192.168.1.1:8080` (click past your browser warning you about the connection not being secure, rayhunter doesn't have HTTPS yet!)
    * Note that you'll need the Orbic's wifi password for this, which can be retrieved by pressing the "MENU" button on the device and opening the 2.4 GHz menu.
2. Over usb: Connect the Orbic device to your laptop via usb. Run `adb forward tcp:8080 tcp:8080`, then visit `http://localhost:8080`. For this you will need to install the Android Debug Bridge (ADB) on your computer, you can copy the version that was downloaded inside the releases/platform-tools/` folder to somewhere else in your path or you can install it manually.  You can find instructions for doing so on your platform [here](https://www.xda-developers.com/install-adb-windows-macos-linux/#how-to-set-up-adb-on-your-computer), (don't worry about instructions for installing it on a phone/device yet).

## Development
* Install ADB  on your computer using the instructions above. 

### If your are on x86 linux
* on your linux laptop install rust the usual way and then install cross compiling dependences. 
* run `sudo apt install  build-essential libc6-armhf-cross libc6-dev-armhf-cross gcc-arm-linux-gnueabihf`

* set up cross compliing for rust:
```
rustup target add x86_64-unknown-linux-gnu
rustup target add armv7-unknown-linux-gnueabihf
```

Now you can root your device and install rayhunter by running `./tools/install-dev.sh` 

### If you are on windows or can't run the install scripts
* Root your device on windows using the instructions here: https://xdaforums.com/t/resetting-verizon-orbic-speed-rc400l-firmware-flash-kajeet.4334899/#post-87855183

* Build for arm using `cargo build` 

* Run tests using `cargo test_pc`

* Push the scripts in `scripts/` to /etc/init.d  on device and make a directory called /data/rayhunter using `adb shell` (and sshell for your root shell if you followed the steps above) 

* you also need to copy `config.toml.example` to /data/rayhunter/config.toml

* Then run `./make.sh` this will build the binary and push it over adb. Restart your device or run `/etc/init.d/rayhunter_daemon start` on the device and you are good to go. 

* Write your code and write tests 

* Build for arm using `cargo build` 

* Run tests using `cargo test_pc`

* push to the device with `./make.sh`

## Documentation 
* Build docs locallly using `RUSTDOCFLAGS="--cfg docsrs" cargo doc --no-deps --all-features  --open`

**LEGAL DISCLAIMER:** Use this program at your own risk. We beilieve running this program does not currently violate any laws or regulations in the United States. However, we are not responsible for civil or criminal liability resulting from the use of this software. If you are located outside of the US please consult with an attorney in your country to help you assess the legal risks of running this program. 

*Good Hunting!*
