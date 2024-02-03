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
![Tests](https://github.com/EFForg/rayhunter/actions/workflows/rust.yml/badge.svg)

Rayhunter is an IMSI Catcher Catcher for the Orbic mobile hotspot. Based on code from [QCSuper](https://github.com/P1sec/QCSuper)

**THIS CODE IS PROOF OF CONCEPT AND SHOULD NOT BE RELIED UPON IN HIGH RISK SITUATIONS**

Code is built and tested for the Orbic RC400L mobile hotspot, it may work on other orbics and other 
linux/qualcom devices but this is the only one we have tested on. Buy the orbic [using bezos bucks](https://www.amazon.com/gp/product/B09CLS6Z7X/)




## Setup
### If your are on x86 linux
on your linux laptop install rust the usual way and then install cross compiling dependences. 
run `sudo apt install  build-essential libc6-armhf-cross libc6-dev-armhf-cross gcc-arm-linux-gnueabihf`

set up cross compliing for rust:
```
rustup target add x86_64-unknown-linux-gnu
rustup target add armv7-unknown-linux-gnueabihf
```

Now you can root your device and install rayhunter by running `./install.sh` - **Note:** You will have to install the cross compile tooling below before running this. 


### If you aren't on linux or can't run the install scripts 
Root your device on windows using the instructions here: https://xdaforums.com/t/resetting-verizon-orbic-speed-rc400l-firmware-flash-kajeet.4334899/#post-87855183

Build for arm using `cargo build` 

Run tests using `cargo test_pc`

Push the scripts in `scripts/` to /etc/init.d  on device and make a directory called /data/rayhunter using `adb shell` (and sshell for your root shell if you followed the steps above) 

you also need to copy `config.toml.example` to /data/rayhunter/config.toml

Then run `./make.sh` this will build the binary and push it over adb. Restart your device or run `/etc/init.d/rayhunter_daemon start` on the device and you are good to go. 

## Development
Write your code and write tests 

Build for arm using `cargo build` 

Run tests using `cargo test_pc`

push to the device with `./make.sh`
