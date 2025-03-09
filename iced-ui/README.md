# Rayhunter Native UI

A native user interface for the Rayhunter IMSI Catcher Catcher built with [iced.rs](https://iced.rs/).

## Overview

This is a native replacement for the web-based UI of Rayhunter. It provides:

1. A dashboard view for monitoring system status and current recordings
2. A recordings view for managing and analyzing captured data
3. A settings view for configuring the application

## Development

### Prerequisites

- Rust (stable channel)
- Cargo

### Building

To build the UI, run:

```bash
cargo build --release
```

### Running

You can run the UI directly:

```bash
cargo run --release
```

Or use the integration script that starts both the daemon and UI:

```bash
./scripts/rayhunter-with-native-ui.sh
```

### Integration with Rayhunter Daemon

The native UI communicates with the Rayhunter daemon through its HTTP API. By default, it connects to `http://localhost:8080`. The server address can be changed in the settings.

## Architecture

The UI follows a Model-View-Update architecture based on the Elm architecture, as implemented by the iced.rs framework:

- **Model**: Application state defined in the various view structs
- **View**: UI layout and rendering defined in the `view()` methods
- **Update**: State update logic in the `update()` methods
- **Messages**: Events that trigger state updates

### Code Structure

- `src/main.rs` - Application entry point and main application loop
- `src/api.rs` - API client for communicating with the Rayhunter daemon
- `src/config.rs` - Configuration management
- `src/style.rs` - Styling constants and themes
- `src/views/` - UI view implementations
  - `dashboard.rs` - Main dashboard view
  - `recordings.rs` - Recording management view
  - `settings.rs` - Application settings view
- `src/widgets/` - Custom widget implementations

## Roadmap

This native UI implementation is being developed incrementally:

1. **Phase 1**: Basic functionality with API integration *(current)*
   - Dashboard view
   - Recordings management
   - Settings management
   
2. **Phase 2**: Enhanced visualization and direct daemon integration
   - Real-time visualization of signal data
   - Direct integration with daemon (without requiring the HTTP API)
   - Advanced filtering and analysis tools
   
3. **Phase 3**: Full replacement of web UI
   - Completely replace the web interface
   - Add additional features not available in the web UI
   - Optimize for performance and resource usage

## Contributing

Contributions are welcome! Please see the main Rayhunter repository's contribution guidelines.