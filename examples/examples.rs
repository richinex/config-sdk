// examples/example.rs
use config_sdk::{start_listening_for_updates, ServerConfig};
use tokio;

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
