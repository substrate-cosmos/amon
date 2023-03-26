//! `export` subcommand - example of how to write a subcommand

/// App-local prelude includes `app_reader()`/`app_writer()`/`app_config()`
/// accessors along with logging macros. Customize as you see fit.
use crate::prelude::*;

use crate::config::AmonConfig;
use abscissa_core::{config, Command, FrameworkError, Runnable};

/// export subcommand
///
/// The `Parser` proc macro generates an option parser based on the struct
/// definition, and is defined in the `clap` crate. See their documentation
/// for a more comprehensive example:
///
/// <https://docs.rs/clap/>
#[derive(clap::Parser, Command, Debug)]
pub struct ExportCmd {}

impl Runnable for ExportCmd {
    /// Start the application.
    fn run(&self) {
        println!("export subcommand!");
    }
}

impl config::Override<AmonConfig> for ExportCmd {
    // Process the given command line options, overriding settings from
    // a configuration file using explicit flags taken from command-line
    // arguments.
    fn override_config(&self, config: AmonConfig) -> Result<AmonConfig, FrameworkError> {
        Ok(config)
    }
}