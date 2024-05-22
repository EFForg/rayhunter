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

Rayhunter is an IMSI Catcher Catcher for the Orbic mobile hotspot. Based on code from [QCSuper](https://github.com/P1sec/QCSuper)

**THIS CODE IS PROOF OF CONCEPT AND SHOULD NOT BE RELIED UPON IN HIGH RISK SITUATIONS**

Code is built and tested for the Orbic RC400L mobile hotspot, it may work on other orbics and other 
linux/qualcom devices but this is the only one we have tested on. Buy the orbic [using bezos bucks](https://www.amazon.com/gp/product/B09CLS6Z7X/)

## Setup

1. Install the Android Debug Bridge (ADB) on your computer (don't worry about instructions for installing it on a phone/device yet). You can find instructions for doing so on your platform [here](https://www.xda-developers.com/install-adb-windows-macos-linux/#how-to-set-up-adb-on-your-computer).
2. Download the latest rayhunter release bundle and unzip it.
3. Run the install script inside the bundle corresponding to your platform (`install-linux.sh`, `install-mac.sh`, or `install-windows.bat`).
4. Once finished, rayhunter should be running! You can verify this by visiting the web UI as described below.

## Usage

Once installed, rayhunter will run automatically whenever your Orbic device is running. It serves a web UI that provides some basic controls, such as being able to start/stop recordings, download captures, and view heuristic analyses of captures. You can access this UI in one of two ways:

1. Over wifi: Connect your phone/laptop to the Orbic's wifi network and visit `http://192.168.1.1:8080` (click past your browser warning you about the connection not being secure, rayhunter doesn't have HTTPS yet!)
    * Note that you'll need the Orbic's wifi password for this, which can be retrieved by pressing the "MENU" button on the device and opening the 2.4 GHz menu.
2. Over usb: Connect the Orbic device to your laptop via usb. Run `adb forward tcp:8080 tcp:8080`, then visit `http://localhost:8080`.
