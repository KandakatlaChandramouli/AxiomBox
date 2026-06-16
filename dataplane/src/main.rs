use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "axiombox")]
#[command(version = "0.1.0")]
#[command(about = "AxiomBox OCI Runtime")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Create {
        id: String,
    },

    Start {
        id: String,
    },

    State {
        id: String,
    },

    Delete {
        id: String,
    },

    Run {
        id: String,
        bundle: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Create { id } => {
            println!("CREATE {}", id);
        }

        Commands::Start { id } => {
            println!("START {}", id);
        }

        Commands::State { id } => {
            println!("STATE {}", id);
        }

        Commands::Delete { id } => {
            println!("DELETE {}", id);
        }

        Commands::Run { id, bundle } => {
            println!("RUN {} {}", id, bundle);
        }
    }
}
