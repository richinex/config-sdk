// src/lib.rs

//! # My SSE Client Library
//!
//! This library provides functionality for connecting to and listening for Server-Sent Events (SSE)
//! from a specified URL. It handles continuous listening, reconnections, and parsing of SSE data
//! into strongly typed Rust structures. The library is designed with resilience in mind, featuring
//! error handling and retry strategies.
//!
//! ## Features
//!
//! - **Listening for SSE**: Connect to an SSE endpoint and listen for real-time events.
//! - **Automatic Reconnection**: Implements exponential backoff strategy for reconnections.
//! - **Configuration Update Handling**: Parse incoming SSE data into custom `ServerConfig` structures.
//! - **Logging**: Utilize built-in logging for monitoring connection status and errors.
//!
//! ## Usage
//!
//! To use this library, you will need to:
//!
//! 1. Define a handler function for processing `ServerConfig` updates.
//! 2. Call `start_listening_for_updates` with the URL of your SSE source, the handler function,
//!    and the maximum number of retries.
//!
//! ```no_run
//! use my_sse_client_library::{ServerConfig, start_listening_for_updates};
//!
//! async fn my_update_handler(config: ServerConfig) {
//!     // Process the incoming configuration update here
//! }
//!
//! #[tokio::main]
//! async fn main() {
//!     let sse_url = "http://example.com/sse";
//!     start_listening_for_updates(sse_url, my_update_handler, 3).await.unwrap();
//! }
//! ```
//!
//! Please refer to the `start_listening_for_updates` function documentation for more details
//! on its parameters and error handling.

mod models;
mod listener;
mod errors;
mod logger;

pub use models::ServerConfig;
pub use listener::start_listening_for_updates;
