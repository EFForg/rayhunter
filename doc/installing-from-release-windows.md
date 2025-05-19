# Installing from the latest release (Windows)

Windows support in Rayhunter's installer is a work-in-progress. Depending on the device, the installation instructions differ.

## TP-Link

1. Connect the device via WiFi or USB Tethering -- you should be able to view the TP-Link admin page on [http://192.168.0.1](http://192.168.0.1).
2. Download the latest release (must be at least 0.3.0), and unpack the zipfile.
3. Open PowerShell or CMD in that extracted folder, and run the binary appropriate for your operating system: `./installer-windows-x86_64/installer tplink`
4. Follow the instructions on the screen, if there are any.

## Orbic

1. Install the [Zadig WinUSB driver](https://zadig.akeo.ie/).
2. Download the latest `rayhunter-vX.X.X.zip` from the [Rayhunter releases page](https://github.com/EFForg/rayhunter/releases). The version you download will have numbers instead of X
3. Unzip `rayhunter-vX.X.X` .
4. Save the [`install.ps1` file here](https://github.com/EFForg/rayhunter/blob/powershell/installer/install.ps1) in top of the folder that was unzipped from release.zip.
5. Run the install script by double clicking on `install.ps1`. A powershell window will launch.
    The device will restart multiple times over the next few minutes.
    You will know it is done when you see terminal output that says `checking for rayhunter server...success!`
6. Rayhunter should now be running! You can verify this by following the instructions below to [view the web UI](#usage-viewing-the-web-ui). You should also see a green line flash along the top of top the display on the device.
