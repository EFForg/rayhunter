# Rayhunter GUI Installer

This directory contains experimental work on a Rayhunter GUI installer based on [Tauri](https://tauri.app/).

## Dependencies

Before building the GUI installer, you'll first need to install its dependencies.

### Tauri Dependencies

You'll need to install [Tauri's dependencies](https://tauri.app/start/prerequisites/). In addition to Rust, you'll need [Node.js/npm](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm). If you're on Linux, also be sure to install the necessary [system dependencies](https://tauri.app/start/prerequisites/#linux) from your package manager.

### Rayhunter CLI Installer

The Rayhunter GUI installer currently just bundles and wraps the CLI Rayhunter installer. When building the GUI installer, the CLI installer needs to be built and available for bundling. By default it assumed the installer is present in the repo's `target` directory at either `debug/installer` or `release/installer` depending on whether you're doing a debug or release build of the GUI installer.

You can use a different path by setting the environment variable INSTALLER_PATH when the GUI installer being built. You can also use the environment variable SKIP_INSTALLER_COPY which leaves any previously bundled CLI installer unmodified or if one does not exist bundles a dummy installer file allowing the GUI installer to be successfully built.

## Building

After preparing dependencies, the GUI installer can be built by:

1. Running `npm install` in this directory.
2. Setting INSTALLER_PATH or SKIP_INSTALLER_COPY if desired and running `npm run tauri dev`.

This will build the GUI installer in development mode. While this command is running, any changes to either the frontend or backend code will cause the installer to be reloaded or rebuilt.

You can also run `npm run tauri build` to create the final GUI installer artifacts for your OS as is done in CI.
