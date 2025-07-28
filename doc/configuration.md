# Configuration

Rayhunter can be configured through web user interface or by editing `/data/rayhunter/config.toml` on the device.

![rayhunter_config](./rayhunter_config.png)

Through web UI you can set:
- **Device UI Level**, which defines what Rayhunter shows on device's built-in screen. *Device UI Level* could be:
  - *Invisible mode*: Rayhunter does not show anything on the built-in screen
  - *Subtle mode (colored line)*: Rayhunter shows green line if there are no warnings, red line if there are warnings (warnings could be checked through web UI) and white line if Rayhunter is not recording
  - *Demo mode (orca gif)*, which shows image of orca fish *and* colored line
  - *EFF logo*, which shows EFF logo and *and* colored line.
- **Device Input Mode**, which defines behaviour of built-in power button of the device. *Device Input Mode* could be:
  - *Disable button control*: built-in power button of the device is not used by Rayhunter;
  - *Double-tap power button to start/stop recording*: double clicking on a built-in power button of the device stops and immediatelly restarts the recording. This could be useful if Rayhunter's heuristichs is triggered and you get the red line, and you want to "reset" the past warnings. Normally you can do that through web UI, but sometimes it is easier to double tap on power button.
- **Colorblind Mode** enables color blind mode (blue line is shown instead of green line, red line remains red). Please note that this does not cover all types of color blindness, but switching green to blue should be about enough to differentiate the color change for most types of color blindness.
- With **Analyzer Heuristic Settings** you can switch on or off built-in [Rayhunter heuristics](heuristics.md). Some heuristics are experimental or can trigger a lot of false positive warnings in some networks (our tests have shown that some heuristics have different behaviour in US or European networks). In that case you can decide whether you would like to have the heuristics that trigger a lot of false positives on or off. Please note that we are constantly improving and adding new heuristics, so new release may reduce false positives in existing heuristics as well.

If you prefer editing `config.toml` file, you need to obtain a shell on your [Orbic](./orbic.md#obtaining-a-shell) or [TP-Link](./tplink-m7350.md#obtaining-a-shell) device and edit the file manually. You can view the [default configuration file on a GitHub](https://github.com/EFForg/rayhunter/blob/main/dist/config.toml.in).
