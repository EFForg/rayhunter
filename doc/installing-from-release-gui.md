# Installing from the release GUI

Make sure you've got one of Rayhunter's [supported devices](./supported-devices.md).

1. **For the TP-Link only,** insert a FAT-formatted SD card. This will be used to store all recordings.

2. Download the latest `Rayhunter-Installer` from the [releases page](https://github.com/EFForg/rayhunter/releases) for your platform:
    - for Windows: `Rayhunter-Installer_VERSION_x64-setup.exe`
    - for MacOS: `Rayhunter-Installer.app.zip`
    - for Linux on x86_64: `Rayhunter-Installer_VERSION_amd64.AppImage`
    - for Linux on AMR64: `Rayhunter-Installer_VERSION_aarch64.AppImage`

3. Decompress the zip archive if applicable and run the installer.

4. On Windows and MacOS, you may need to take additional steps due to our application not containing a signature trusted by Microsoft or Apple.
    - On Windows, this involves clicking "More Info" on the Microsoft Defender screen and then clicking "Run anyway".
    - On MacOS, you should follow the instructions at <https://support.apple.com/en-is/guide/mac-help/mh40616/mac>.

5. **On Windows only,** the application will walk you through installing the Rayhunter Installer which must then be run separately. This can be done by leaving the "Run Rayhunter Installer" box checked at the end of the setup process.

6. Follow the prompts in the program to install Rayhunter on your device.

{{#include common-post-install.md}}