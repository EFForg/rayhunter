# Installing from the latest release

Make sure you've got one of Rayhunter's [supported devices](./supported-devices.md). These instructions have only been tested on macOS and Ubuntu 24.04. If they fail, you will need to [install Rayhunter from source](./installing-from-source.md).

1. Download the latest `rayhunter-vX.X.X.zip` from the [Rayhunter releases page](https://github.com/EFForg/rayhunter/releases)
2. Decompress the `rayhunter-vX.X.X.zip` archive. Open the terminal and navigate to the folder. (Be sure to replace X.X.X with the correct version number!)

    ```bash
    unzip ~/Downloads/rayhunter-vX.X.X.zip
    cd ~/Downloads/rayhunter-vX.X.X
    ```

3. Turn on your device by holding the power button on the front.

   * For the Orbic, connect the device using a USB-C cable.
   * For TP-Link, connect to its network using either WiFi or USB Tethering.

4. Run the install script for your operating system:

    First, enter the correct subfolder for your operating system:
    - for Ubuntu on x64 arhitecture: `cd installer-ubuntu-24`
    - for Ubuntu on ARM64 arhitecture: `cd installer-ubuntu-24-aarch64`
    - for MacOS on Intel (old macbooks) architecture: `cd installer-macos-intel`
    - for MacOS on ARM (M1/M2 etc.) achitecture: `cd installer-macos-arm`
    - for Windows: `cd installer-windows-x86_64`

    ```bash
    # On MacOS, you must first remove the quarantine bit
    xattr -d com.apple.quarantine installer
    ```
    Then run the installer:
    ```bash
    ./installer orbic
    # or: ./installer tplink
    # or: ./installer wingtech
    ```

    The device will restart multiple times over the next few minutes.

    You will know it is done when you see terminal output that says `Testing Rayhunter... done`

5. Rayhunter should now be running! You can verify this by [viewing Rayhunter's web UI](./using-rayhunter.md). You should also see a green line flash along the top of top the display on the device.

## Troubleshooting

* On MacOS if you encounter an error that says "No Orbic device found," it may because you have the "Allow accessories to connect" security setting set to "Ask for approval." You may need to temporarily change it to "Always" for the script to run. Make sure to change it back to a more secure setting when you're done.

./installer --help
./installer util --help
