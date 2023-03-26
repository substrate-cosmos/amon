//! `keys` subcommand - Manage your application's keys

/// App-local prelude includes `app_reader()`/`app_writer()`/`app_config()`
/// accessors along with logging macros. Customize as you see fit.
use crate::prelude::*;

use crate::config::AmonConfig;
use abscissa_core::{config, Command, FrameworkError, Runnable};

mod add;
mod delete;
mod export;
mod import;
mod list;
mod migrate;
mod mnemonic;
mod parse;
mod rename;
mod show;

/// `keys` subcommand
///
/// Manage your application's keys
///
/// Keyring management commands. These keys may be in any format supported by the
/// CometBFT crypto library and can be used by light-clients, full nodes, or any other application
/// that needs to sign with a private key.
///
/// The keyring supports the following backends:
///
///     os          Uses the operating system's default credentials store.
///
///     file        Uses encrypted file-based keystore within the app's configuration directory.
///                 This keyring will request a password each time it is accessed, which may occur
///                 multiple times in a single command resulting in repeated password prompts.
///
///     kwallet     Uses KDE Wallet Manager as a credentials management application.
///
///     pass        Uses the pass command line utility to store and retrieve keys.
///
///     test        Stores keys insecurely to disk. It does not prompt for a password to be unlocked
///                 and it should be use only for testing purposes.
///
/// kwallet and pass backends depend on external tools. Refer to their respective documentation for more
/// information:
///
///     KWallet     https://github.com/KDE/kwallet
///
///     pass        https://www.passwordstore.org/
///
/// The pass backend requires GnuPG: https://gnupg.org/
#[derive(clap::Subcommand, Command, Debug)]
pub enum KeysCmd {
    Add(add::AddCmd),
    Delete(delete::DeleteCmd),
    Export(export::ExportCmd),
    Import(import::ImportCmd),
    List(list::ListCmd),
    Migrate(migrate::MigrateCmd),
    Mnemonic(mnemonic::MnemonicCmd),
    Parse(parse::ParseCmd),
    Rename(rename::RenameCmd),
    Show(show::ShowCmd),
}

impl Runnable for KeysCmd {
    /// Start the application.
    fn run(&self) {
        println!("Hello, amon wallet!");
        match self {
            KeysCmd::Add(c) => c.run(),
            KeysCmd::Delete(c) => c.run(),
            KeysCmd::Export(c) => c.run(),
            KeysCmd::Import(c) => c.run(),
            KeysCmd::List(c) => c.run(),
            KeysCmd::Migrate(c) => c.run(),
            KeysCmd::Mnemonic(c) => c.run(),
            KeysCmd::Parse(c) => c.run(),
            KeysCmd::Rename(c) => c.run(),
            KeysCmd::Show(c) => c.run(),
        }
    }
}

impl config::Override<AmonConfig> for KeysCmd {
    // Process the given command line options, overriding settings from
    // a configuration file using explicit flags taken from command-line
    // arguments.
    fn override_config(&self, config: AmonConfig) -> Result<AmonConfig, FrameworkError> {
        Ok(config)
    }
}
