# Installing from the latest release

Make sure you've got one of Rayhunter's [supported devices](./supported-devices.md). These instructions have only been tested on macOS and Ubuntu 24.04. If they fail, you will need to [install Rayhunter from source](./installing-from-source.md).

1. **For the TP-Link only,** insert a FAT-formatted SD card. This will be used to store all recordings.
2. Download the latest `rayhunter-cli-vX.X.X-PLATFORM.zip` from the [Rayhunter releases page](https://github.com/EFForg/rayhunter/releases) for your platform:
    - for Linux on x64 architecture: `linux-x64`
    - for Linux on ARM64 architecture: `linux-aarch64`
    - for Linux on armv7/v8 (32-bit) architecture: `linux-armv7`
    - for MacOS on Intel (old macbooks) architecture: `macos-intel`
    - for MacOS on ARM (M1/M2 etc.) architecture: `macos-arm`
    - for Windows: `windows-x86_64`

    > **Note:** Download the `rayhunter-cli-vX.X.X-PLATFORM.zip` asset for your platform, **not** the automatically generated "Source code (zip)" archive at the bottom of the release. The source archive does not contain the prebuilt `installer` binary, so the commands below will fail with an error like `./installer: command not found`.

3. Decompress the `rayhunter-cli-vX.X.X-PLATFORM.zip` archive. Open the terminal and navigate to the folder. (Be sure to replace X.X.X with the correct version number!)

    ```bash
    unzip ~/Downloads/rayhunter-cli-vX.X.X-PLATFORM.zip
    cd ~/Downloads/rayhunter-cli-vX.X.X-PLATFORM
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

   # Important notes:
   # - the arguments --admin-username 'myusername' and --admin-ip 'mydeviceip'
   #   may be required if different from the default.
   # - on Windows this command should be run in PowerShell and not the Command
   #   Prompt (CMD) where characters like quotes are treated differently.

   # For TP-Link:
   ./installer tplink
   ```

   * On Verizon Orbic, the default password is the WiFi password. You can reset the password by pressing the button under the back case until the unit restarts.
      * ***Note:*** If you have changed the device username, password, or IP address from their default values, these must be provided as arguments to the installer command above.
   * TP-Link does not require an `--admin-password` parameter.

   For other devices, check `./installer --help` or the
   respective page in the sidebar under "Supported
   Devices" for the installation command.

{{#include common-post-install.md}}

```bash
./installer --help
./installer util --help
```
