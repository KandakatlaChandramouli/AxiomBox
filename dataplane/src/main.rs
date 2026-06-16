use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Parser)]
#[command(name = "axiombox")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Create { id: String },
    State { id: String },
}

#[derive(Serialize, Deserialize)]
struct ContainerState {
    id: String,
    status: String,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Create { id } => {
            let state = ContainerState {
                id: id.clone(),
                status: "created".to_string(),
            };

            let path = format!(
                "runtime/containers/{}.json",
                id
            );

            fs::write(
                path,
                serde_json::to_string_pretty(&state).unwrap(),
            )
            .unwrap();

            println!("container created");
        }

        Commands::State { id } => {
            let path = format!(
                "runtime/containers/{}.json",
                id
            );

            let data =
                fs::read_to_string(path).unwrap();

            println!("{}", data);
        }
    }
}
