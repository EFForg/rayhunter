# Installing from the latest release

Make sure you've got one of Rayhunter's [supported devices](./supported-devices.md). These instructions have only been tested on macOS and Ubuntu 24.04. If they fail, you will need to [install Rayhunter from source](./installing-from-source.md).

1. For the TP-Link only, insert a FAT-formatted SD card. This will be used to store all recordings.
2. Download the latest `rayhunter-vX.X.X-PLATFORM.zip` from the [Rayhunter releases page](https://github.com/EFForg/rayhunter/releases) for your platform:
    - for Linux on x64 architecture: `linux-x64`
    - for Linux on ARM64 architecture: `linux-aarch64`
    - for Linux on armv7/v8 (32-bit) architecture: `linux-armv7`
    - for MacOS on Intel (old macbooks) architecture: `macos-intel`
    - for MacOS on ARM (M1/M2 etc.) architecture: `macos-arm`
    - for Windows: `windows-x86_64`

3. Decompress the `rayhunter-vX.X.X-PLATFORM.zip` archive. Open the terminal and navigate to the folder. (Be sure to replace X.X.X with the correct version number!)

    ```bash
    unzip ~/Downloads/rayhunter-vX.X.X-PLATFORM.zip
    cd ~/Downloads/rayhunter-vX.X.X-PLATFORM
    ```

4. Turn on your device by holding the power button on the front.

   * For the Orbic, connect the device using a USB-C cable.
     * Or connect to the network if using the network based installer, this is especially recommended on Windows.
   * For TP-Link, connect to its network using either WiFi or USB Tethering.

5. Run the installer:

    ```bash
    # On MacOS, you must first remove the quarantine bit
    xattr -d com.apple.quarantine installer
    ```
    Then run the installer:
    ```bash
    ./installer orbic
    # or: ./installer [orbic-network|tplink|tmobile|uz801|pinephone|wingtech]
    ```

    The device will restart multiple times over the next few minutes.

    You will know it is done when you see terminal output that says `Testing Rayhunter... done`

6. Rayhunter should now be running! You can verify this by [viewing Rayhunter's web UI](./using-rayhunter.md). You should also see a green line flash along the top of top the display on the device.

## Troubleshooting

* You can test your device by enabling the test heuristic. This will be very noisy and fire an alert every time you see a new tower. Be sure to turn it off when you are done testing.  

* On MacOS if you encounter an error that says "No Orbic device found," it may because you have the "Allow accessories to connect" security setting set to "Ask for approval." You may need to temporarily change it to "Always" for the script to run. Make sure to change it back to a more secure setting when you're done.

```bash
./installer --help
./installer util --help
```
