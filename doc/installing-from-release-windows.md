# Installing from the latest release (Windows)

1. Install the [Zadig WinUSB driver](https://zadig.akeo.ie/).
2. Download the latest `rayhunter-vX.X.X.zip` from the [Rayhunter releases page](https://github.com/EFForg/rayhunter/releases). The version you download will have numbers instead of X
3. Unzip `rayhunter-vX.X.X` .
4. Save the [`install.ps1` file here](https://github.com/EFForg/rayhunter/blob/powershell/installer/install.ps1) in top of the folder that was unzipped from release.zip.
5. Run the following powershell command `Set-ExecutionPolicy remotesigned`
5. Run the install script by double clicking on `install.ps1`. A powershell window will launch.
    The device will restart multiple times over the next few minutes.
    You will know it is done when you see terminal output that says `checking for rayhunter server...success!`
6. Rayhunter should now be running! You can verify this by following the instructions below to [view the web UI](#usage-viewing-the-web-ui). You should also see a green line flash along the top of top the display on the device.
