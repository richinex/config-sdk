// listener.rs

use crate::errors::ConfigError;
use crate::logger::configure_logging;
use crate::models::ServerConfig;
use futures::stream::StreamExt;
use reqwest::Client;
use serde_json::from_slice;
use slog::{info, warn};
use tokio::time::{sleep, Duration};


/// Starts listening for Server-Sent Events (SSE) from the specified URL and
/// handles updates using the provided update handler function.
///
/// This function establishes an HTTP connection to the given `url` to listen for
/// SSE. Upon receiving an event, it attempts to parse the event data as JSON into
/// a `ServerConfig` and passes the result to `update_handler`. The connection
/// attempts are made with exponential backoff based on the number of retries.
///
/// # Arguments
///
/// * `url` - A string slice that holds the URL of the SSE server to connect to.
/// * `update_handler` - A function or closure that takes a `ServerConfig` and handles
///   it. This handler is called each time a valid event is received and successfully
///   parsed.
/// * `max_retries` - The maximum number of connection attempts to make before giving up.
///
/// # Errors
///
/// Returns `Err(ConfigError)` if an error occurs while trying to establish a connection,
/// if there is an issue with the incoming data stream, or if the maximum number of retries
/// is reached without a successful connection.
///
/// # Examples
///
/// ```
/// async fn update_config(config: ServerConfig) {
///     // Handle the configuration update here
/// }
///
/// let url = "http://example.com/config_stream";
/// start_listening_for_updates(url, update_config, 5).await.unwrap();
/// ```
pub async fn start_listening_for_updates<F>(url: &str, mut update_handler: F, max_retries: u32) -> Result<(), ConfigError>
where
    F: FnMut(ServerConfig) + Send + 'static,
{
    let log = configure_logging();
    let client = Client::builder()
        .user_agent("RichieClient/1.0")
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
