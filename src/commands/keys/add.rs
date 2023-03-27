//! `add` subcommand - example of how to write a subcommand

/// App-local prelude includes `app_reader()`/`app_writer()`/`app_config()`
/// accessors along with logging macros. Customize as you see fit.
use crate::prelude::*;

use crate::config::AmonConfig;
use abscissa_core::{config, Command, FrameworkError, Runnable};

/// add subcommand - Add an encrypted private key (either newly generated or recovered), encrypt it, and save to <name> file
///
/// Derive a new private key and encrypt to disk.
///
/// Optionally specify a BIP39 mnemonic, a BIP39 passphrase to further secure the mnemonic,
/// and a bip32 HD path to derive a specific account. The key will be stored under the given name
/// and encrypted with the given password. The only input that is required is the encryption password.
///
/// If run with -i, it will prompt the user for BIP44 path, BIP39 mnemonic, and passphrase.
/// The flag --recover allows one to recover a key from a seed passphrase.
/// If run with --dry-run, a key would be generated (or recovered) but not stored to the
/// local keystore.
///
/// Use the --pubkey flag to add arbitrary public keys to the keystore for constructing
/// multisig transactions.
///
/// You can create and store a multisig key by passing the list of key names stored in a keyring
/// and the minimum number of signatures required through --multisig-threshold. The keys are
/// sorted by address, unless the flag --nosort is set.
///
/// Example:
///     keys add mymultisig --multisig "keyname1,keyname2,keyname3" --multisig-threshold 2
///
#[derive(clap::Parser, Command, Debug)]
pub struct AddCmd {
    /// Account name
    pub name: String,
    /// Account number for HD derivation (less than equal 2147483647)
    #[arg(long, default_value_t = 0)]
    pub account: u32,
    /// Key signing algorithm to generate keys for
    #[arg(long, default_value_t = String::from("secp256k1"))]
    pub algo: String, // todo
    /// coin type number for HD derivation
    #[arg(short, long, default_value_t = 118)]
    pub coin_type: u32, // todo
    /// Perform action, but don't add key to local keystore
    #[arg(long, default_value_t = false)]
    pub dry_run: bool,
    /// Manual HD Path derivation (overrides BIP44 config)
    #[arg(long)]
    pub hd_path: Option<String>,
    /// Address index number for HD derivation (less than equal 2147483647)
    #[arg(short, long, default_value_t = 0)]
    pub index: u32,
    /// Interactively prompt user for BIP39 passphrase and mnemonic
    #[arg(long, default_value_t = false)]
    pub interactive: bool,
    /// Store a local reference to a private key on a Ledger device
    #[arg(long, default_value_t = false)]
    pub ledger: bool,
    /// List of key names stored in keyring to construct a public legacy multisig key
    #[arg(long)]
    pub multisig: Option<String>,
    /// K out of N required signatures. For use in conjunction with --multisig
    #[arg(long, default_value_t = 1)]
    pub multisig_threshold: i32,
    /// Don't print out seed phrase (if others are watching the terminal)
    #[arg(long, default_value_t = false)]
    pub no_backup: bool,
    /// Keys passed to --multisig are taken in the order they're supplied
    #[arg(long, default_value_t = false)]
    pub nosort: bool,
    /// Parse a public key in JSON format and saves key info to <name> file.
    #[arg(short, long)]
    pub pubkey: Option<String>,
    /// Provide seed phrase to recover existing key instead of creating
    #[arg(long, default_value_t = false)]
    pub recover: bool,
}

impl Runnable for AddCmd {
    /*
    input
      - bip39 mnemonic
      - bip39 passphrase
      - bip44 path
      - local encryption password
    output
      - armor encrypted private key (saved to file)
    */
    /// Start the application.
    fn run(&self) {
        println!("add subcommand!");
        //        var err error

        // name := args[0]
        let name = self.name.clone();
        // interactive, _ := cmd.Flags().GetBool(flagInteractive)
        let interactive = self.interactive;
        // noBackup, _ := cmd.Flags().GetBool(flagNoBackup)
        let no_backup = self.no_backup;
        // showMnemonic := !noBackup
        let show_mnemonic = !no_backup;
        // kb := ctx.Keyring
        // outputFormat := ctx.OutputFormat

        // keyringAlgos, _ := kb.SupportedAlgorithms()
        // algoStr, _ := cmd.Flags().GetString(flags.FlagKeyType)
        // algo, err := keyring.NewSigningAlgoFromString(algoStr, keyringAlgos)
        // if err != nil {
        // 	return err
        // }

        // if dryRun, _ := cmd.Flags().GetBool(flags.FlagDryRun); dryRun {
        // 	// use in memory keybase
        // 	kb = keyring.NewInMemory(ctx.Codec)
        // } else {
        // 	_, err = kb.Key(name)
        // 	if err == nil {
        // 		// account exists, ask for user confirmation
        // 		response, err2 := input.GetConfirmation(fmt.Sprintf("override the existing name %s", name), inBuf, cmd.ErrOrStderr())
        // 		if err2 != nil {
        // 			return err2
        // 		}

        // 		if !response {
        // 			return errors.New("aborted")
        // 		}

        // 		err2 = kb.Delete(name)
        // 		if err2 != nil {
        // 			return err2
        // 		}
        // 	}

        // 	multisigKeys, _ := cmd.Flags().GetStringSlice(flagMultisig)
        // 	if len(multisigKeys) != 0 {
        // 		pks := make([]cryptotypes.PubKey, len(multisigKeys))
        // 		multisigThreshold, _ := cmd.Flags().GetInt(flagMultiSigThreshold)
        // 		if err := validateMultisigThreshold(multisigThreshold, len(multisigKeys)); err != nil {
        // 			return err
        // 		}

        // 		for i, keyname := range multisigKeys {
        // 			k, err := kb.Key(keyname)
        // 			if err != nil {
        // 				return err
        // 			}

        // 			key, err := k.GetPubKey()
        // 			if err != nil {
        // 				return err
        // 			}
        // 			pks[i] = key
        // 		}

        // 		if noSort, _ := cmd.Flags().GetBool(flagNoSort); !noSort {
        // 			sort.Slice(pks, func(i, j int) bool {
        // 				return bytes.Compare(pks[i].Address(), pks[j].Address()) < 0
        // 			})
        // 		}

        // 		pk := multisig.NewLegacyAminoPubKey(multisigThreshold, pks)
        // 		k, err := kb.SaveMultisig(name, pk)
        // 		if err != nil {
        // 			return err
        // 		}

        // 		return printCreate(cmd, k, false, "", outputFormat)
        // 	}
        // }

        // pubKey, _ := cmd.Flags().GetString(FlagPublicKey)
        // if pubKey != "" {
        // 	var pk cryptotypes.PubKey
        // 	if err = ctx.Codec.UnmarshalInterfaceJSON([]byte(pubKey), &pk); err != nil {
        // 		return err
        // 	}

        // 	k, err := kb.SaveOfflineKey(name, pk)
        // 	if err != nil {
        // 		return err
        // 	}

        // 	return printCreate(cmd, k, false, "", outputFormat)
        // }

        // coinType, _ := cmd.Flags().GetUint32(flagCoinType)
        // account, _ := cmd.Flags().GetUint32(flagAccount)
        // index, _ := cmd.Flags().GetUint32(flagIndex)
        // hdPath, _ := cmd.Flags().GetString(flagHDPath)
        // useLedger, _ := cmd.Flags().GetBool(flags.FlagUseLedger)

        // if len(hdPath) == 0 {
        // 	hdPath = hd.CreateHDPath(coinType, account, index).String()
        // } else if useLedger {
        // 	return errors.New("cannot set custom bip32 path with ledger")
        // }

        // // If we're using ledger, only thing we need is the path and the bech32 prefix.
        // if useLedger {
        // 	bech32PrefixAccAddr := sdk.GetConfig().GetBech32AccountAddrPrefix()
        // 	k, err := kb.SaveLedgerKey(name, hd.Secp256k1, bech32PrefixAccAddr, coinType, account, index)
        // 	if err != nil {
        // 		return err
        // 	}

        // 	return printCreate(cmd, k, false, "", outputFormat)
        // }

        // // Get bip39 mnemonic
        // var mnemonic, bip39Passphrase string

        // recover, _ := cmd.Flags().GetBool(flagRecover)
        // if recover {
        // 	mnemonic, err = input.GetString("Enter your bip39 mnemonic", inBuf)
        // 	if err != nil {
        // 		return err
        // 	}

        // 	if !bip39.IsMnemonicValid(mnemonic) {
        // 		return errors.New("invalid mnemonic")
        // 	}
        // } else if interactive {
        // 	mnemonic, err = input.GetString("Enter your bip39 mnemonic, or hit enter to generate one.", inBuf)
        // 	if err != nil {
        // 		return err
        // 	}

        // 	if !bip39.IsMnemonicValid(mnemonic) && mnemonic != "" {
        // 		return errors.New("invalid mnemonic")
        // 	}
        // }

        // if len(mnemonic) == 0 {
        // 	// read entropy seed straight from cmtcrypto.Rand and convert to mnemonic
        // 	entropySeed, err := bip39.NewEntropy(mnemonicEntropySize)
        // 	if err != nil {
        // 		return err
        // 	}

        // 	mnemonic, err = bip39.NewMnemonic(entropySeed)
        // 	if err != nil {
        // 		return err
        // 	}
        // }

        // // override bip39 passphrase
        // if interactive {
        // 	bip39Passphrase, err = input.GetString(
        // 		"Enter your bip39 passphrase. This is combined with the mnemonic to derive the seed. "+
        // 			"Most users should just hit enter to use the default, \"\"", inBuf)
        // 	if err != nil {
        // 		return err
        // 	}

        // 	// if they use one, make them re-enter it
        // 	if len(bip39Passphrase) != 0 {
        // 		p2, err := input.GetString("Repeat the passphrase:", inBuf)
        // 		if err != nil {
        // 			return err
        // 		}

        // 		if bip39Passphrase != p2 {
        // 			return errors.New("passphrases don't match")
        // 		}
        // 	}
        // }

        // k, err := kb.NewAccount(name, mnemonic, bip39Passphrase, hdPath, algo)
        // if err != nil {
        // 	return err
        // }

        // // Recover key from seed passphrase
        // if recover {
        // 	// Hide mnemonic from output
        // 	showMnemonic = false
        // 	mnemonic = ""
        // }

        // return printCreate(cmd, k, showMnemonic, mnemonic, outputFormat)
    }
}

impl config::Override<AmonConfig> for AddCmd {
    // Process the given command line options, overriding settings from
    // a configuration file using explicit flags taken from command-line
    // arguments.
    fn override_config(&self, config: AmonConfig) -> Result<AmonConfig, FrameworkError> {
        Ok(config)
    }
}

use serde_json::json;
use std::fmt;
use std::io::{Error, Write};

enum OutputFormat {
    Text,
    JSON,
}

fn print_create(
    k: String,
    show_mnemonic: bool,
    mnemonic: &str,
    output_format: OutputFormat,
) -> Result<(), Error> {
    match output_format {
        OutputFormat::Text => {
            // if let Err(e) =
            //     print_keyring_record(cmd.out_or_stdout(), k, mk_acc_key_output, &output_format)
            // {
            //     return Err(e);
            // }

            // print mnemonic unless requested not to.
            if show_mnemonic {
                println!(
                    "\n**Important** write this mnemonic phrase in a safe place.\n\
                    It is the only way to recover your account if you ever forget your password.\n\n{}\n",
                    mnemonic
                );
            }
        }
        OutputFormat::JSON => {
            // let mut out = mk_acc_key_output(k)?;

            // if show_mnemonic {
            //     out.mnemonic = mnemonic.to_string();
            // }

            // let json_string = json!(&out).to_string();
            // println!("{}", json_string);
        }
    }
    Ok(())
}
