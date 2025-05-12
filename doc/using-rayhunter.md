# Using Rayhunter

Once installed, Rayhunter will run automatically whenever your device is running. You'll see a green line on top of the device's display to indicate that it's running and recording. [The line will turn red](#red) once a potential IMSI catcher has been found, until the device is rebooted or a new recording is started through the web UI.

It also serves a web UI that provides some basic controls, such as being able to start/stop recordings, download captures, delete captures, and view heuristic analyses of captures.

You can access this UI in one of two ways:

1. **Connect over wifi:** Connect your phone/laptop to your device's wifi network and visit [http://192.168.1.1:8080](http://192.168.1.1:8080). (Click past your browser warning you about the connection not being secure, Rayhunter doesn't have HTTPS yet).
    * On the Orbic, you can find the wifi network password by going to the Orbic's menu > 2.4 GHz WIFI Info > Enter > find the 8-character password next to the lock 🔒 icon.
2. **Connect over USB:** Connect your device to your laptop via USB. Run `adb forward tcp:8080 tcp:8080`, then visit [http://localhost:8080](http://localhost:8080).
    * For this you will need to install the Android Debug Bridge (ADB) on your computer, you can copy the version that was downloaded inside the `releases/platform-tools/` folder to somewhere else in your path or you can install it manually.
    * You can find instructions for doing so on your platform [here](https://www.xda-developers.com/install-adb-windows-macos-linux/#how-to-set-up-adb-on-your-computer), (don't worry about instructions for installing it on a phone/device yet).
    * On macOS, the easiest way to install ADB is with Homebrew: First [install Homebrew](https://brew.sh/), then run `brew install android-platform-tools`.
