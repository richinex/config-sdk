//listener.rs

use crate::errors::ConfigError;
use crate::logger::configure_logging;
use crate::models::ServerConfig;
use futures::stream::StreamExt;
use reqwest::Client;
use serde_json::from_slice;
use slog::{info, warn};
use tokio::time::{sleep, Duration};

pub async fn start_listening_for_updates<F>(url: &str, mut update_handler: F) -> Result<(), ConfigError>
where
    F: FnMut(ServerConfig) + Send + 'static,
{
    let log = configure_logging(); // Ensure you've correctly initialized your logger
    let mut attempt = 0;

    loop {
        attempt += 1;
        let client = Client::new();

        match client.get(url)
            .header("Accept", "text/event-stream")
            .send()
            .await {
            Ok(response) => {
                // info!(log, "Connected to SSE server"; "url" => url, "attempt" => attempt);
                info!(log, "Connected to SSE server"; "url" => url, "attempt" => format!("{}", attempt));

                let mut stream = response.bytes_stream();

                while let Some(item) = stream.next().await {
                    match item {
                        Ok(bytes) => {
                            // Handle potential UTF-8 conversion errors by defaulting to an empty string
                            let text = String::from_utf8(bytes.to_vec()).unwrap_or_else(|_| "".to_string());
                            // Log the received SSE data
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

                break; // Successfully connected and processed the stream, exit loop
            },
            Err(e) => {
                warn!(log, "Failed to connect to SSE server"; "error" => %e, "attempt" => format!("{}", attempt));


                if attempt >= 5 {
                    return Err(ConfigError::Request(e)); // Give up after 5 attempts
                }

                // Wait for 2 seconds before retrying
                sleep(Duration::from_secs(2)).await;
            },
        }
    }

    Ok(())
}
