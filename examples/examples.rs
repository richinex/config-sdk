// examples/example.rs
use config_sdk::{start_listening_for_updates, ServerConfig};
use tokio;

#[tokio::main]
async fn main() {
    let sse_url = "http://localhost:8080/sse/dev"; // Your SSE endpoint

    if let Err(e) = start_listening_for_updates(sse_url, |config: ServerConfig| {
        println!("Received config update: {:?}", config);
    }).await {
        eprintln!("Failed to listen for updates: {}", e);
    }
}
