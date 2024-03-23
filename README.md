
# Rust Configuration SDK

This Rust-based SDK is designed to facilitate the integration with config-server-sse endpoints, focusing on enabling applications to receive real-time configuration updates. It's built with resilience in mind, featuring exponential backoff retry strategies and comprehensive error handling.

## Features

- **Real-Time Updates**: Leverage SSE for receiving live configuration changes.
- **Robust Error Handling**: Includes mechanisms to gracefully handle connectivity issues and data parsing errors.
- **Exponential Backoff**: Implements an exponential backoff strategy for reconnections, enhancing the reliability of the SDK.
- **Customizable Logging**: Utilizes `slog` for flexible and powerful logging capabilities.

## Getting Started

### Prerequisites

Ensure you have the latest version of Rust and Cargo installed on your system.

### Installation

Add the SDK to your `Cargo.toml`:

```toml
[dependencies]
config-sdk = { git = "https://github.com/richinex/config-sdk", branch = "main" }
```

### Usage

To start listening for configuration updates from your SSE server:

```rust
#[tokio::main]
async fn main() {
    let sse_url = "http://localhost:8080/sse/dev"; // Your SSE endpoint
    let max_retries = 5; // Maximum number of retries for connecting to the SSE server

    // Now calling start_listening_for_updates with the max_retries parameter
    if let Err(e) = start_listening_for_updates(sse_url, |config: ServerConfig| {
        println!("Received config update: {:?}", config);
    }, max_retries).await {
        eprintln!("Failed to listen for updates: {}", e);
    }
}
```

## Contributing

Contributions are welcome! Please feel free to submit pull requests, report bugs, and suggest features.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.
