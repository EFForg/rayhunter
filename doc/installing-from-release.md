# Installing from the latest release

Make sure you've got one of Rayhunter's [supported devices](./supported-devices.md). These instructions have only been tested on macOS and Ubuntu 24.04. If they fail, you will need to [install Rayhunter from source](./installing-from-source.md).

1. **For the TP-Link only,** insert a FAT-formatted SD card. This will be used to store all recordings.
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

   On Windows you can decompress using the file browser, then navigate to the
   folder that contains `installer.exe`, **hold Shift**, Right-Click inside the
   folder, then click "Open in PowerShell".

4. **Connect to your device.**

   First turn on your device by holding the power button on the front.

   Then connect to the device using either WiFi or USB tethering.

   You know you are in the right network when you can access
   <http://192.168.1.1> (Orbic) or <http://192.168.0.1> (TP-Link) and see the
   hardware's own admin menu.

5. **On MacOS only**, you have to run `xattr -d
   com.apple.quarantine installer` to allow execution of
   the binary.

6. **Run the installer.**

   ```bash
   # For Orbic:
   ./installer orbic --admin-password 'mypassword'
   # Note: the arguments --admin-username 'myusername' and --admin-ip 'mydeviceip'
   #       may be required if different from the default.

   # Or install over USB if you want ADB and a root shell (not recommended for most users)
   ./installer orbic-usb

   # For TP-Link:
   ./installer tplink
   ```

   * On Verizon Orbic, the password is the one used to login to the device's admin menu, and the default is the WiFi password.
     * ***Note:*** If you have changed the device username, password, or IP address from their default values, these must be provided as arguments to the installer command above.
   * On Kajeet/Smartspot devices, the default password is `$m@rt$p0tc0nf!g`
   * On Moxee-brand devices, check under the battery for the password.
   * You can reset the password by pressing the button under the back case until the unit restarts.

   TP-Link does not require an `--admin-password` parameter.

   For other devices, check `./installer --help` or the
   respective page in the sidebar under "Supported
   Devices."

7. The installer will eventually tell you it's done, and the device will reboot.

8. Rayhunter should now be running! You can verify this by [viewing Rayhunter's web UI](./using-rayhunter.md). You should also see a green line flash along the top of top the display on the device.

## Troubleshooting

* If you are having trouble installing Rayhunter and you're connecting to your device over USB, try using a different USB cable to connect the device to your computer. If you are using a USB hub, try using a different one or directly connecting the device to a USB port on your computer. A faulty USB connection can cause the Rayhunter installer to fail.

* You can test your device by enabling the test heuristic. This will be very noisy and fire an alert every time you see a new tower. Be sure to turn it off when you are done testing.  

* On MacOS if you encounter an error that says "No Orbic device found," it may because you have the "Allow accessories to connect" security setting set to "Ask for approval." You may need to temporarily change it to "Always" for the script to run. Make sure to change it back to a more secure setting when you're done.

```bash
./installer --help
./installer util --help
```
