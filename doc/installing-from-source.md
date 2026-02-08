# Installing from source

Building Rayhunter from source, either for development or otherwise, involves a
number of external dependencies. Unless you need to do this, we recommend you
use our [compiled builds](https://github.com/EFForg/rayhunter/releases).

At a high level, we have:

* A JS frontend written in SvelteKit (`./daemon/web/`)
* A Rust binary `rayhunter-daemon` (`./daemon/`) that runs on the device, and bundles the frontend.
* A Rust binary `installer` (`./installer`) that runs on the computer and bundles `rayhunter-daemon`.

It's recommended to work either on Mac/Linux, or WSL on Windows.

## Building frontend and backend

First, install dependencies:

- [Rust](https://www.rust-lang.org/tools/install)
- [Node.js/npm](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm)
- C compiler tools (`apt install build-essential` or `brew install gcc`)

Then you can build everything with:

```sh
./scripts/build-dev.sh
./scripts/install-dev.sh orbic  # replace 'orbic' with your device type
```

## Hot-reloading the frontend

If you are working on the frontend, you normally have to repeat all of the above steps everytime to see a change.

You can instead run the frontend separately on your PC while the Rust parts
continue running on your target device:

```sh
cd daemon/web

# Assumes rayhunter-daemon is listening on localhost:8080
npm run dev

# Use a custom target IP:port where the backend runs
API_TARGET=http://192.168.1.1:8080 npm run dev
```

The UI will listen on `localhost:5173` and instantly show any frontend changes
you make. Backend changes require building everything from the top (daemon and installer).

## Installer utils, getting a shell

Check `./scripts/install-dev.sh util --help`
for useful utilities for transferring files, opening shells. The exact tools
available wildly depend on the device you're working on, and they are
usually documented the relevant device's page under [Supported
Devices](./supported-devices.md).

A lot of devices run a trimmed down version of Android and have ADB (Android
Debug Bridge) support. The USB-based installers (`orbic-usb`, `pinephone`,
`uz801`) use ADB to perform the installation.

You might want to install and use actual ADB to connect to the device, push
files and generally poke around. The installer contains some tools to enable ADB:

```sh
adb kill-server

# Enables ADB on either of these devices
./scripts/install-dev.sh util tmobile-start-adb
./scripts/install-dev.sh orbic-usb

adb shell
```

Note though that we can't assist with any issues setting ADB up, _especially
not_ on Windows. There have been too many driver issues to make this the
"golden path" for most users or contributors. There have been instances where
people managed to brick their orbic devices using ADB on Windows.
