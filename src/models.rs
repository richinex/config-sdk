use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;

/// Represents the configuration of a server, dynamically structured as a map.
///
/// The `ServerConfig` struct is designed to hold a configuration where keys are
/// represented by strings, and values are JSON values, allowing for a flexible
/// and dynamic configuration structure. This can accommodate various types of
/// configurations that might include numbers, strings, arrays, or even nested
/// objects.
///
/// # Usage
///
/// The `ServerConfig` can be used to deserialize server configuration data received
/// in the form of JSON. It can handle a wide variety of configuration styles and
/// structures, making it highly adaptable for different server setups.
///
/// # Example
///
/// ```
/// use my_crate::ServerConfig;
/// use serde_json::Value;
/// use std::collections::BTreeMap;
///
/// let mut settings = BTreeMap::new();
/// settings.insert("timeout".to_string(), Value::Number(30.into()));
/// settings.insert("hostname".to_string(), Value::String("example.com".to_string()));
///
/// let config = ServerConfig {
///     settings,
/// };
///
/// // Use `config` for accessing the server settings...
/// ```
///
/// In this example, a `ServerConfig` instance is created with a timeout and a hostname
/// setting. This shows how the struct can be manually constructed, but in practice, it
/// is more likely to be deserialized directly from JSON data received from a server or
/// configuration file.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ServerConfig {
    /// A map holding the server settings where each key is a setting name (a `String`)
    /// and each value is a `serde_json::Value`, allowing for flexible configuration data
    /// types including numbers, strings, arrays, and objects.
    pub settings: BTreeMap<String, Value>,
}
