# Orca

```
                                               
                                               
       u.      .u    .                         
 ...ue888b   .d88B :@8c        .         u     
 888R Y888r ="8888f8888r  .udR88N     us888u.  
 888R I888>   4888>'88"  <888'888k .@88 "8888" 
 888R I888>   4888> '    9888 'Y"  9888  9888  
 888R I888>   4888>      9888      9888  9888  
u8888cJ888   .d888L .+   9888      9888  9888  
 "*888*P"    ^"8888*"    ?8888u../ 9888  9888  
   'Y"          "Y"       "8888P'  "888*""888" 
                            "P'     ^Y"   ^Y'  
                                               
                                               
Orca Realtime Cellular Analysis 
                                                                                                 
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

IMSI Catcher Catcher for the Orbic mobile hotspot. Based on code from [QCSuper](https://github.com/P1sec/QCSuper)

**THIS CODE IS PROOF OF CONCEPT AND SHOULD NOT BE RELIED UPON IN HIGH RISK SITUATIONS**

Code is built and tested for the Orbic RC400L mobile hotspot, it may work on other orbics and other 
linux/qualcom devices but this is the only one we have tested on. Buy the orbic [using bezos bucks](https://www.amazon.com/gp/product/B09CLS6Z7X/)

Root your device on windows using the instructions here: https://xdaforums.com/t/resetting-verizon-orbic-speed-rc400l-firmware-flash-kajeet.4334899/#post-87855183
(script to root on linux coming soon) 



## Setup
on your linux laptop install rust the usual way and then install cross compiling dependences. 
run `sudo apt install  build-essential libc6-armhf-cross libc6-dev-armhf-cross gcc-arm-linux-gnueabihf`

set up cross compliing for rust:
```
rustup target add x86_64-unknown-linux-gnu
rustup target add armv7-unknown-linux-gnueabihf
```

Build for arm using `cargo build` 

Run tests using `cargo test_pc`


Push the scripts in `scripts/` to /etc/init.d  on device and make a directory called /data/wavehunter using `adb shell` (and sshell for your root shell if you followed the steps above) 

you also need to copy `config.toml.example` to /data/wavehunter/config.toml

Then run ./make.sh this will build the binary and push it over adb. Restart your device or run `/etc/init.d wavehunter_daemon start` on the device and you are good to go. 
