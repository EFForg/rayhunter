# Rayhunter GUI Installer

This directory contains experimental work on a Rayhunter GUI installer based on [Tauri](https://tauri.app/).

## Dependencies

Before building the GUI installer, you'll first need to install its dependencies.

### Tauri Dependencies

You'll need to install [Tauri's dependencies](https://tauri.app/start/prerequisites/). In addition to Rust, you'll need [Node.js/npm](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm). If you're on Linux, also be sure to install the necessary [system dependencies](https://tauri.app/start/prerequisites/#linux) from your package manager.

### Rayhunter CLI Installer

The GUI installer pulls in the CLI installer as a library. Like with the CLI installer, the firmware binary needs to be present and can be overridden with the same envvars. See `../installer/build.rs` for options.

For example, to build the firmware in development mode:

```bash
cargo build-daemon-firmware-devel
cargo build-rootshell-firmware-devel

(cd installer-gui && FIRMWARE_PROFILE=firmware-devel npm run tauri android build)
```

## Building

After preparing dependencies, the GUI installer can be built by:

1. Running `npm install` in this directory.
2. Running `npm run tauri dev`.

This will build the GUI installer in development mode. While this command is running, any changes to either the frontend or backend code will cause the installer to be reloaded or rebuilt.

You can also run `npm run tauri build` to create the final GUI installer artifacts for your OS as is done in CI.
