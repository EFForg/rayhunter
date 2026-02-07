# Installing from source

Building Rayhunter from source, either for development or otherwise, involves a
number of external dependencies. Unless you need to do this, we recommend you
use our [compiled builds](https://github.com/EFForg/rayhunter/releases).

At a high level, we have:

* A JS frontend written in SvelteKit (`./daemon/web/`)
* A Rust binary `rayhunter-daemon` (`./daemon/`) that runs on the device, and bundles the frontend.
* A Rust binary `installer` (`./installer`) that runs on the computer and bundles `rayhunter-daemon`.

It's recommended to work either on Mac/Linux, or WSL on Windows.

## Quick start

If you have [Rust](https://www.rust-lang.org/tools/install) and
[Node.js/npm](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm)
installed, you can build everything with:

```sh
./scripts/build-dev.sh
./scripts/install-dev.sh orbic  # replace 'orbic' with your device type
```

## Step 1: Building the frontend

Install [nodejs/npm](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm), which is required to build Rayhunter's web UI.

Run:

```sh
pushd daemon/web && npm install && npm run build && popd
```

## Step 2: Building the daemon

[Install Rust the usual way](https://www.rust-lang.org/tools/install). Then,

- install a C compiler (`apt install build-essential` would be the command under Ubuntu)

- install the cross-compilation target for the device Rayhunter will run on:

  ```sh
  rustup target add armv7-unknown-linux-musleabihf
  ```

- install the statically compiled target for your host machine:

  ```sh
  # check which toolchain you have installed by default with
  rustup show
  # now install the correct variant for your host platform, one of:
  rustup target add aarch64-unknown-linux-musl
  rustup target add armv7-unknown-linux-musleabi
  rustup target add x86_64-unknown-linux-musl
  rustup target add aarch64-apple-darwin
  rustup target add x86_64-apple-darwin
  rustup target add x86_64-pc-windows-gnu
  ```

Now to build the daemon:

```sh
# Build the daemon binary for local development (rustcrypto TLS backend, fast compilation)
# WARNING: The rustcrypto library, though not known to be insecure, is less well
# tested than its counterpart and could potentially have severe issues in
# its cryptographic implementation. We therefore recommend using ring-tls in
# production builds (see below)
cargo build-daemon-firmware-devel

# To build it exactly like in CI (more mature ring TLS backend, slower compilation)
# CC_armv7_unknown_linux_musleabihf=arm-linux-gnueabihf-gcc cargo build-daemon-firmware

# Build rootshell
cargo build-rootshell-firmware-devel
```

## Step 3: Running the installer

Now that all dependencies of the installer have been built, you can run the installer like so (with the device connected according to the regular installation instructions):

```sh
# Replace 'orbic' with your device type if different.
# A list of possible values can be found with 'cargo run --bin installer help'.
FIRMWARE_PROFILE=firmware-devel cargo run -p installer --bin installer orbic
```

## Optional: Hot-reloading the frontend

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

## Optional: Installer utils, getting a shell

Check `FIRMWARE_PROFILE=firmware-devel cargo run -p installer -- util --help`
for useful utilities for transferring files, opening shells. The exact tools
available wildly depend on the device you're working on, and they are
usually documented the relevant device's page under [Supported
Devices](./supported-devices.md).

A lot of devices run a trimmed down version of Android and have ADB (Android
Debug Bridge) support. The USB-based installers (`orbic-usb`, `pinephone`,
`uz801`) use ADB to perform the installation.

You might want to install and use actual ADB to connect to the device, push
files and generally poke around. `installer util --help` contains commands to enable
ADB for some devices.

Note though that we can't assist with any issues setting ADB up, _especially
not_ on Windows. There have been too many driver issues to make this the
"golden path" for most users or contributors. There have been instances where
people managed to brick their orbic devices using ADB on Windows.

The installers `orbic` and `tplink` use network connections exclusively to
perform the installation. They end up not enabling ADB at all, and as such
cannot run into permission issues, driver issues. The downside is that the
development tooling (getting a shell, transferring a file) is currently all
over the place, and mostly consists of random subcommands in `installer util`.
