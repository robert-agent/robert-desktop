use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "robert")]
#[command(version = "0.1.0")]
#[command(about = "Robert CLI for headless management", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Download a model
    Download {
        /// Model ID (e.g., microsoft/Phi-3-mini-4k-instruct)
        model_id: String,

        /// Revision (optional)
        #[arg(long)]
        revision: Option<String>,
    },

    /// Ingest a file into memory
    Ingest {
        /// Path to file
        file: PathBuf,
    },

    /// Query the agent
    Query {
        /// Query text
        text: String,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Download { model_id, revision } => {
            println!("Downloading model: {}", model_id);
            // TODO: Connect to robert-downloader
            println!("Download feature coming soon via robert-downloader");
        }
        Commands::Ingest { file } => {
            println!("Ingesting file: {:?}", file);
            // TODO: Connect to robert-core memory
            println!("Ingest feature coming soon via robert-core");
        }
        Commands::Query { text } => {
            println!("Querying: {}", text);
            // TODO: Connect to robert-core inference
            println!("Query feature coming soon via robert-core");
        }
    }

    Ok(())
}
