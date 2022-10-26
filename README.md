# Orderbook Aggregator UI

## Building

Tested on Ubuntu 22.04.

Requires orderbook-aggregator crate to be present in the parent directory.

Requires Rust and npm already installed.

Install system dependencies:

```bash
sudo apt install \
    build-essential \
    curl \
    file \
    libayatana-appindicator3-dev \
    libgtk-3-dev \
    librsvg2-dev \
    libssl-dev \
    libwebkit2gtk-4.0-dev \
    wget
```

Install npm dependencies:

```bash
npm install
```

Build the app:

```bash
npm run tauri build
```

## Running

The app requires the server to be already running. It only attempts to connect to the server at startup.

```bash
./src-tauri/target/release/bundle/deb/orderbook-aggregator-ui_0.0.0_amd64/data/usr/bin/orderbook-aggregator-ui --url http://localhost:50051
```

## References

- https://tauri.app/v1/guides/getting-started/prerequisites
