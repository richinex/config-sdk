// listener.rs

use crate::errors::ConfigError;
use crate::logger::configure_logging;
use crate::models::ServerConfig;
use futures::stream::StreamExt;
use reqwest::Client;
use serde_json::from_slice;
use slog::{info, warn};
use tokio::time::{sleep, Duration};

pub async fn start_listening_for_updates<F>(url: &str, mut update_handler: F, max_retries: u32) -> Result<(), ConfigError>
where
    F: FnMut(ServerConfig) + Send + 'static,
{
    let log = configure_logging();
    let client = Client::builder()
        .user_agent("MyCustomClient/1.0")
        .build()?;
    let mut attempt = 0;
    const BASE_DELAY: u64 = 2; // Base delay in seconds for the exponential backoff

    loop {
        attempt += 1;

        match client.get(url)
            .header("Accept", "text/event-stream")
            .send()
            .await {
            Ok(response) => {
                if response.status().is_success() {
                    info!(log, "Connected to SSE server"; "url" => url, "attempt" => format!("{}", attempt));
                    let mut stream = response.bytes_stream();

                    while let Some(item) = stream.next().await {
                        match item {
                            Ok(bytes) => {
                                let text = String::from_utf8(bytes.to_vec()).unwrap_or_else(|_| "".to_string());
                                info!(log, "Received SSE data"; "data" => &text);

                                if text.starts_with("data: ") {
                                    let json_part = text.trim_start_matches("data: ").trim();
                                    match from_slice::<ServerConfig>(json_part.as_bytes()) {
                                        Ok(config) => {
                                            update_handler(config);
                                            info!(log, "Configuration updated"; "config" => json_part);
                                        },
                                        Err(e) => {
                                            warn!(log, "Failed to parse configuration data"; "error" => %e);
                                        },
                                    }
                                }
                            },
                            Err(e) => {
                                warn!(log, "Error processing SSE data"; "error" => %e);
                                return Err(ConfigError::Request(e));
                            },
                        }
                    }

                    // Exit the loop successfully after processing the stream
                    break;
                } else {
                    warn!(log, "Received non-success status from SSE server"; "status" => %response.status(), "url" => %url);
                    // Instead of breaking, continue to apply retry logic
                }
            },
            Err(e) => {
                warn!(log, "Failed to connect to SSE server"; "error" => %e, "attempt" => format!("{}", attempt));
            },
        }

        if attempt >= max_retries {
            // Give up after reaching the maximum number of retries
            return Err(ConfigError::GenericError("Maximum retries reached, giving up.".to_string()));
        }

        // Calculate the delay for the exponential backoff
        let delay = BASE_DELAY.pow(attempt) as u64;
        warn!(log, "Retrying in {} seconds...", delay);
        sleep(Duration::from_secs(delay)).await;
    }

    Ok(())
}
