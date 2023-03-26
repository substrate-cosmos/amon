//! Amon Subcommands
//!
//! This is where you specify the subcommands of your application.
//!
//! The default application comes with two subcommands:
//!
//! - `start`: launches the application
//! - `--version`: print application version
//!
//! See the `impl Configurable` below for how to specify the path to the
//! application's configuration file.

mod keys;
mod start;

use self::keys::KeysCmd;
use self::start::StartCmd;
use crate::config::AmonConfig;
use abscissa_core::{config::Override, Command, Configurable, FrameworkError, Runnable};
use std::path::PathBuf;
use tracing::{error, info};

/// Amon Configuration Filename
pub const DEFAULT_CONFIG_PATH: &str = ".amon/config.toml";

/// Default configuration file path
pub fn default_config_file() -> Option<PathBuf> {
    dirs_next::home_dir().map(|home| home.join(DEFAULT_CONFIG_PATH))
}

/// Amon Subcommands
/// Subcommands need to be listed in an enum.
#[derive(clap::Parser, Command, Debug, Runnable)]
pub enum AmonCmd {
    /// The `start` subcommand
    Start(StartCmd),
    #[command(subcommand)]
    /// The `keys` subcommand
    Keys(KeysCmd),
}

/// Entry point for the application. It needs to be a struct to allow using subcommands!
#[derive(clap::Parser, Command, Debug)]
#[command(author, about, version)]
pub struct EntryPoint {
    #[command(subcommand)]
    cmd: AmonCmd,

    /// Enable verbose logging
    #[arg(short, long)]
    pub verbose: bool,

    /// Use the specified config file
    #[arg(short, long)]
    pub config: Option<String>,
}

impl Runnable for EntryPoint {
    fn run(&self) {
        self.cmd.run()
    }
}

/// This trait allows you to define how application configuration is loaded.
impl Configurable<AmonConfig> for EntryPoint {
    /// Location of the configuration file
    fn config_path(&self) -> Option<PathBuf> {
        let path = default_config_file();

        match path {
            Some(path) if path.exists() => {
                info!("using default configuration from '{}'", path.display());
                Some(path)
            }
            Some(path) => {
                // No file exists at the config path
                error!("could not find configuration file at '{}'", path.display());
                error!("for an example, please see https://hermes.informal.systems/config.html#example-configuration-file");
                None
            }
            None => {
                // The path to the default config file could not be found
                error!("could not find default configuration file");
                error!(
                    "please create one at '~/{}' or specify it with the '-c'/'--config' flag",
                    DEFAULT_CONFIG_PATH
                );
                error!("for an example, please see https://hermes.informal.systems/config.html#example-configuration-file");
                None
            }
        }
    }

    /// Apply changes to the config after it's been loaded, e.g. overriding
    /// values in a config file using command-line options.
    ///
    /// This can be safely deleted if you don't want to override config
    /// settings from command-line options.
    fn process_config(&self, config: AmonConfig) -> Result<AmonConfig, FrameworkError> {
        match &self.cmd {
            AmonCmd::Start(cmd) => cmd.override_config(config),
            AmonCmd::Keys(cmd) => cmd.override_config(config),
            //
            // If you don't need special overrides for some
            // subcommands, you can just use a catch all
            // _ => Ok(config),
        }
    }
}
