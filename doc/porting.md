# Porting to new devices

## When will we consider new devices?

Rayhunter is already officially supported on [several devices](./supported-devices.md), and people are often interested in adding support for hardware they already own. Here's a non-exhaustive list of situations where we'd consider adding a new Tier 2 device:

* The device is significantly cheaper or more available in a specific region than any device we already support.
* The device supports 5G and costs less than 100 USD.
* You're willing to commit to supporting this device and handling bug reports.

We want to avoid a situation where the list of supported devices keeps growing but the number of recurring contributors and maintainers stays the same.

That said, you can always maintain a fork, or install Rayhunter manually without writing an installer. You can promote this work in the [GitHub discussions](https://github.com/EFForg/rayhunter/discussions) area, where most new hardware investigations happen.

Please don't open issues about supporting a new device, use GitHub discussions instead. Most hardware investigations end up being abandoned, and the amount of issues we'd have to triage would be too much.

## Prerequisites: root shell, and /dev/diag

Rayhunter is a Linux binary that reads traffic from `/dev/diag`, which requires root. If either of those isn't available, Rayhunter can't work. Everything else (displays, buttons) is secondary, and we can deal with it later.

You can check ahead of purchase whether `/dev/diag` is available by ensuring the device has a Qualcomm MDM* chip. Other Qualcomm LTE chips might work but we haven't encountered one yet. Typically you will be able to get this information from [fcc.report](https://fcc.report), where either the chip is written down in some PDF or at least plainly visible in one of the teardown photos. Sometimes this information can also be found through teardown videos on YouTube. If you find that chip, there's a good chance (but no guarantee) `/dev/diag` is available.

Any vendor other than Qualcomm (Mediatek, Rockchip, ...) is unlikely to work. Quectel sometimes repackages Qualcomm chips into larger systems and might work. Huawei devices won't work, as they use their own chips.

Getting a root shell varies from device to device. Check the [GitHub discussions](https://github.com/EFForg/rayhunter/discussions) for prior art, and look through the installer source in `installer/src/` for inspiration. These approaches are common:

* Connecting with `adb shell`.
* If `adb shell` doesn't work, sending a special USB serial command might enable it.
* Sometimes there's an unpatched CVE that can be used to launch `telnetd` as root (search "device name CVE", the website [opencve.io](https://opencve.io) is particularly easy to use).

Once you have a root shell, check that `/dev/diag` exists.

## Installing Rayhunter manually

The Rayhunter installation consists of just two components: the `rayhunter-daemon` binary, and the config file (`config.toml`).

Typically the layout on the filesystem will look like this:

```text
/data/rayhunter/rayhunter-daemon
/data/rayhunter/config.toml
/data/rayhunter/qmdl/
```

Then, `./rayhunter-daemon config.toml` can be started manually.

You can refer to [Installing from source](./installing-from-source.md) for how to obtain the `rayhunter-daemon` binary.

We're assuming that your device is ARMv7, i.e. 32-bit ARM. If that's not the case, you can still build the daemon but you'll need to figure out the correct target triple on your own.

You can copy the daemon and config files to the device using `netcat` or `adb push`. They don't have to be in `/data/rayhunter/`, this is just convention. If you use a different path, be sure to update the `qmdl_store_path` setting in `config.toml`.

## Display support

Rayhunter has a `device` setting in `config.toml` (see [`Device` enum in `lib/src/lib.rs`](https://github.com/EFForg/rayhunter/blob/main/lib/src/lib.rs)), which conditionally enables and disables specific behavior such as how to render the display. Unless your device is a variant of an existing device, you'll want to add a new variant to the `Device` enum and write a corresponding display module in `daemon/src/display/`.

You can play around with the existing values of the `device` setting to see which one ends up rendering on your device's display. Most likely your device has a display similar enough to an existing one, and the display module for that device (e.g. `daemon/src/display/orbic.rs`, `daemon/src/display/tplink.rs`) can be used as a starting point.

If your device has LEDs instead of a display, take a look at `daemon/src/display/uz801.rs` which controls LEDs via sysfs.

## Button support

Rayhunter can use the power button to restart recordings via a double-tap gesture. The implementation is in [`daemon/src/key_input.rs`](https://github.com/EFForg/rayhunter/blob/main/daemon/src/key_input.rs). It currently has no structure for device-specific implementations, as all devices we support expose the same input event interface.

The `key_input_mode` setting in `config.toml` controls this feature (`0` = disabled, `1` = double-tap power button to start/stop recordings).

## Writing the installer, and contributing official support

At this point you'll want to have figured out how to automate the entire installation in principle, and how to make it as repeatable as possible. A proof-of-concept of this in bash or another language is also a welcome contribution (to be posted on [GitHub discussions](https://github.com/EFForg/rayhunter/discussions), not as a PR).

Writing the installer means adding a new variant to the `Command` enum in [`installer/src/lib.rs`](https://github.com/EFForg/rayhunter/blob/main/installer/src/lib.rs) and implementing the install logic in a new module under `installer/src/`. Each subcommand maps to a device-specific entry point function (e.g. `tplink::main_tplink`, `orbic_network::install`).

You should also add a corresponding shell/telnet utility subcommand under `installer util` (the `UtilSubCommand` enum in `installer/src/lib.rs`). These utilities (e.g. `installer util tplink-shell`, `installer util orbic-shell`) give users and developers interactive shell access to the device, which is essential for debugging. See the existing `UtilSubCommand` variants for examples.

Please reuse existing utilities wherever possible. Take a look at [`installer/src/tplink.rs`](https://github.com/EFForg/rayhunter/blob/main/installer/src/tplink.rs) and [`installer/src/orbic_network.rs`](https://github.com/EFForg/rayhunter/blob/main/installer/src/orbic_network.rs) for inspiration. But the structures there are still evolving, and we'll happily guide you during code review.
