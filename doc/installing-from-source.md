# Installing from source

Building Rayhunter from source, either for development or because the install script doesn't work on your system, involves a number of external dependencies. Unless you need to do this, we recommend you use our [compiled builds](https://github.com/EFForg/rayhunter/releases).

* Install [nodejs/npm](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm), which is required to build Rayhunter's web UI
  * Make sure to build the site with `pushd daemon/web && npm install && npm run build && popd` before building Rayhunter. If you're working directly on the frontend, `npm run dev` will allow you to test a local frontend with hot-reloading (use `http://localhost:5173` instead of `http://localhost:8080`).
* Install ADB on your computer using the instructions above, and make sure it's in your terminal's PATH
  * You can verify if ADB is in your PATH by running `which adb` in a terminal. If it prints the filepath to where ADB is installed, you're set! Otherwise, try following one of these guides:
    * [linux](https://askubuntu.com/questions/652936/adding-android-sdk-platform-tools-to-path-downloaded-from-umake)
    * [macOS](https://www.repeato.app/setting-up-adb-on-macos-a-step-by-step-guide/)
    * [Windows](https://medium.com/@yadav-ajay/a-step-by-step-guide-to-setting-up-adb-path-on-windows-0b833faebf18)
* Install `curl` on your computer to run the install scripts. It is not needed to build binaries.

### Install Rust targets

[Install Rust the usual way](https://www.rust-lang.org/tools/install). Then,

- install the cross-compilation target for the device Rayhunter will run on:
```sh
rustup target add armv7-unknown-linux-musleabihf
```

- install the statically compiled target for your host machine to build the binary installer `serial`.
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

Now you can root your device and install Rayhunter by running:

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

# Replace 'orbic' with your device type if different.
# A list of possible values can be found with 'cargo run --bin installer help'.
FIRMWARE_PROFILE=firmware-devel cargo run -p installer --bin installer orbic
```

### If you're on Windows or can't run the install scripts

* Root your device on Windows using the instructions here: <https://xdaforums.com/t/resetting-verizon-orbic-speed-rc400l-firmware-flash-kajeet.4334899/#post-87855183>
* Build the web UI using `cd daemon/web && npm install && npm run build`
* Push the scripts in `scripts/` to `/etc/init.d` on device and make a directory called `/data/rayhunter` using `adb shell` (and sshell for your root shell if you followed the steps above)
* You also need to copy `config.toml.in` to `/data/rayhunter/config.toml`. Uncomment the `device` line and set the value to your device type if necessary.
* Then run `./make.sh`, which will build the binary, push it over adb, and restart the device. Once it's restarted, Rayhunter should be running!
