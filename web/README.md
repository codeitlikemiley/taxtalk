# TaxTalk Web Interface

Web interface for the TaxTalk natural language bookkeeping system built with Leptos.

## Configuration

The web interface supports configurable ports via environment variables:

- `SERVER_PORT` - Backend API server port (default: 3000)
- `CLIENT_PORT` - Frontend dev server port (default: 8080)

## Prerequisites

Install required tools:
```bash
# Install trunk for building and serving
cargo install trunk

# Add wasm target
rustup target add wasm32-unknown-unknown
```

## Development

### Quick Start with Scripts

```bash
# Default ports (client: 8080, server: 3000)
./serve.sh

# Custom ports
CLIENT_PORT=3001 SERVER_PORT=4000 ./serve.sh
```

### Manual Commands

```bash
# Build with custom server port
SERVER_PORT=4000 trunk build

# Serve with custom ports  
SERVER_PORT=4000 trunk serve --port 3001

# Open in browser
SERVER_PORT=4000 trunk serve --port 3001 --open
```

### Running the Backend Server

From the root directory:
```bash
# Default port 3000
cargo run --bin server

# Custom port
PORT=4000 cargo run --bin server
```

## Building for Production

```bash
# Using the build script
SERVER_PORT=4000 ./build.sh

# Or manually
SERVER_PORT=4000 trunk build --release
```

The production build will be output to the `dist` folder.

## Architecture

The web interface provides:

- **Guided Chat Interface** - Natural language command input with step-by-step guidance
- **Smart Autocomplete** - Entity selection with type-ahead search
- **Plugin Management** - Enable/disable and configure WASM plugins
- **Command Palette** - Quick access to common commands
- **Real-time Validation** - Immediate feedback on command structure
- **Progress Tracking** - Visual indicators for multi-step transactions

## Project Structure

```
web/
├── src/
│   ├── components/       # UI components
│   │   ├── guided_chat.rs
│   │   ├── plugins.rs
│   │   └── ...
│   ├── pages/            # Page components
│   ├── config.rs         # Configuration (API URLs)
│   └── lib.rs           # App entry point
├── public/              # Static assets
├── Trunk.toml          # Build configuration
├── serve.sh            # Dev server script
└── build.sh            # Production build script
```

## Styling

The project uses Tailwind CSS for styling. The Tailwind configuration is in `tailwind.css` and is automatically processed by Trunk during the build.

## API Integration

All API calls use the configurable `SERVER_PORT` environment variable. The base URL is constructed at compile time using the `option_env!` macro, ensuring the correct backend port is used.

Example:
```rust
// Automatically uses SERVER_PORT env var
let url = api_url("api/validate");
```