//! Amon Config
//!
//! See instructions in `commands.rs` to specify the path to your
//! application's configuration file and/or command-line options
//! for specifying it.

use serde::{Deserialize, Serialize};

/// Amon Configuration
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AmonConfig {
    /// An example configuration section
    pub account: Account,
}

/// Default configuration settings.
///
/// Note: if your needs are as simple as below, you can
/// use `#[derive(Default)]` on AmonConfig instead.
impl Default for AmonConfig {
    fn default() -> Self {
        Self {
            account: Account::default(),
        }
    }
}

/// Example configuration section.
///
/// Delete this and replace it with your actual configuration structs.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Account {
    /// Example configuration value
    pub public_key: String,
    pub private_key: String,
}

impl Default for Account {
    fn default() -> Self {
        Self {
            public_key: "alice public key".to_string(),
            private_key: "alice private key".to_string(),
        }
    }
}
