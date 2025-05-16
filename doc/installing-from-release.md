# Installing from the latest release

Make sure you've got one of Rayhunter's [supported devices](./supported-devices.md). These instructions have only been tested on macOS and Ubuntu 24.04. If they fail, you will need to [install Rayhunter from source](./installing-from-source.md).

1. Download the latest `release.tar` from the [Rayhunter releases page](https://github.com/EFForg/rayhunter/releases)
2. Decompress the `release.tar` archive. Open the terminal and navigate to the folder

    ```bash
    mkdir ~/Downloads/release
    tar -xvf ~/Downloads/release.tar -C ~/Downloads/release
    cd ~/Downloads/release
    ```

3. Turn on your device by holding the power button on the front.

  * For the Orbic, connect the device using a USB-C cable.
  * For TP-Link, connect to its network using either WiFi or USB Tethering.

4. Run the install script for your operating system:

    ```bash
    ./install orbic
    # or: ./install tplink
    ```

    The device will restart multiple times over the next few minutes.

    You will know it is done when you see terminal output that says `Testing rayhunter... done`

5. Rayhunter should now be running! You can verify this by [viewing Rayhunter's web UI](./using-rayhunter). You should also see a green line flash along the top of top the display on the device.

## Troubleshooting

* On macOS if you encounter an error that says "No Orbic device found," it may because you the "Allow accessories to connect" security setting set to "Ask for approval." You may need to temporarily change it to "Always" for the script to run. Make sure to change it back to a more secure setting when you're done.
