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

diag helper binary for the Orbic mobile hotspot. Based on code from [QCSuper](https://github.com/P1sec/QCSuper)

Build for arm using `cargo build` or just use the makefile 

Run tests using `cargo test_pc`

## Setup
Root your device using the instructions here: https://xdaforums.com/t/resetting-verizon-orbic-speed-rc400l-firmware-flash-kajeet.4334899/#post-87855183

Push the scripts in `scripts/` to /etc/init.d  on device and make a directory called /data/wavehunter using `adb shell` (and sshell for your root shell if you followed the steps above) 

you also need to copy `config.toml.example` to /data/wavehunter/config.toml

Then run ./make.sh this will build the binary and push it over adb. Restart your device or run `/etc/init.d wavehunter_daemon start` on the device and you are good to go. 
