//! `gaia-validate` binary — thin CLI wrapper around the library.
//!
//! # Usage
//!
//! ```text
//! gaia-validate validate [--mode <MODE>] [--attempt <N>] [--prior <FILE>]
//! gaia-validate result   [--path <FILE>]
//! gaia-validate history  [--dir <DIR>]
//! ```

use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::process;

#[derive(Parser)]
#[command(
    name = "gaia-validate",
    about = "MCP service layer for the GAIA-2.0 human-gated correction loop",
    version
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run agent-validate.sh and print the structured result as JSON.
    Validate {
        /// Validation mode: full, changed, or targeted.
        #[arg(long, default_value = "changed")]
        mode: String,

        /// Correction loop attempt number (1-indexed).
        #[arg(long, default_value_t = 1)]
        attempt: u32,

        /// Path to the previous agent-validation.json for no-progress detection.
        #[arg(long)]
        prior: Option<PathBuf>,
    },

    /// Deserialise and print an existing agent-validation.json as JSON.
    Result {
        /// Path to agent-validation.json.
        #[arg(long, default_value = "agent-validation.json")]
        path: PathBuf,
    },

    /// Load and print all attempt results from a directory as a JSON array.
    History {
        /// Directory containing agent-validation-N.json files.
        #[arg(long, default_value = ".")]
        dir: PathBuf,
    },

    /// Boot the ontology chain and print the running status.
    OntologyBoot,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Validate { mode, attempt, prior } => {
            match gaia_validate::validate(mode, *attempt, prior.as_deref()) {
                Ok(result) => {
                    let json = serde_json::to_string_pretty(&result)
                        .expect("serialisation cannot fail");
                    println!("{json}");
                    // Mirror the shell script exit codes so the binary can be
                    // used in the same pipelines:
                    //   0 = passed, 1 = failed, 3 = no_progress
                    if result.is_no_progress() {
                        process::exit(3);
                    } else if !result.is_passing() {
                        process::exit(1);
                    }
                }
                Err(e) => {
                    eprintln!("error: {e}");
                    process::exit(2);
                }
            }
        }

        Commands::Result { path } => {
            match gaia_validate::get_validation_result(path) {
                Ok(result) => {
                    let json = serde_json::to_string_pretty(&result)
                        .expect("serialisation cannot fail");
                    println!("{json}");
                }
                Err(e) => {
                    eprintln!("error: {e}");
                    process::exit(2);
                }
            }
        }

        Commands::OntologyBoot => {
            let boot = gaia_validate::OntologyBoot::boot();
            println!("{}", gaia_validate::boot_line());
            let claim = gaia_validate::PowerClaim {
                name: "super autonomy".into(),
                state: gaia_validate::PowerState::Operational,
                has_mechanism: false,
                has_evidence: false,
            };
            match boot.submit(&claim) {
                Err(_) => println!("operational claim without evidence: rejected"),
                Ok(()) => {
                    eprintln!("error: operational claim was accepted");
                    process::exit(1);
                }
            }
        }

        Commands::History { dir } => {
            match gaia_validate::get_attempt_history(dir) {
                Ok(history) => {
                    let json = serde_json::to_string_pretty(&history.entries)
                        .expect("serialisation cannot fail");
                    println!("{json}");
                    if history.is_stuck() {
                        eprintln!("warning: loop is stuck — last two attempts share identical failure fingerprints");
                        process::exit(3);
                    }
                }
                Err(e) => {
                    eprintln!("error: {e}");
                    process::exit(2);
                }
            }
        }
    }
}
