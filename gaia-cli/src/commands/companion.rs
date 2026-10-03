//! Local companion. Memory stays on disk. The model must be localhost.

use anyhow::{anyhow, Result};
use clap::{Args, Subcommand};
use std::path::PathBuf;
use std::time::Duration;

#[derive(Args)]
pub struct CompanionArgs {
    #[command(subcommand)]
    pub action: Action,
}

#[derive(Subcommand)]
pub enum Action {
    /// Write one note.
    Remember { note: String },
    /// Print the notes.
    Recall,
    /// Ask the local model. A miss is a miss.
    Model { prompt: String },
}

pub async fn run(args: CompanionArgs) -> Result<()> {
    let path = memory_path();
    match args.action {
        Action::Remember { note } => {
            gaia_kernel::companion::remember(&path, &note).map_err(anyhow::Error::msg)?;
            println!("remembered");
            Ok(())
        }
        Action::Recall => {
            let notes = gaia_kernel::companion::recall(&path).map_err(anyhow::Error::msg)?;
            println!("notes={}", notes.len());
            for note in notes {
                println!("{note}");
            }
            Ok(())
        }
        Action::Model { prompt } => {
            let url = "http://127.0.0.1:11434/api/generate";
            gaia_kernel::companion::local_model_url(url).map_err(anyhow::Error::msg)?;
            match ask_local(&prompt) {
                Ok(text) => {
                    println!("model=local");
                    println!("{text}");
                    Ok(())
                }
                Err(reason) => Err(anyhow!(reason)),
            }
        }
    }
}

fn memory_path() -> PathBuf {
    let home = std::env::var("GAIA_HOME").unwrap_or_else(|_| ".".into());
    PathBuf::from(home).join("companion").join("memory.txt")
}

fn ask_local(prompt: &str) -> Result<String, String> {
    let body = format!("{{\"model\":\"llama3.2\",\"prompt\":{},\"stream\":false}}", serde_json::to_string(prompt).map_err(|err| err.to_string())?);
    let agent = ureq::AgentBuilder::new().timeout(Duration::from_secs(2)).build();
    let response = agent
        .post("http://127.0.0.1:11434/api/generate")
        .set("content-type", "application/json")
        .send_string(&body)
        .map_err(|_| "local model did not answer".to_string())?;
    response.into_string().map_err(|err| err.to_string())
}
