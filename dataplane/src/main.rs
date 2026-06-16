mod oci;

use clap::{Parser, Subcommand};
use oci::spec::OciSpec;
use std::fs;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Run {
        id: String,
        bundle: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { id, bundle } => {
            let path =
                format!("{}/config.json", bundle);

            let data =
                fs::read_to_string(path).unwrap();

            let spec: OciSpec =
                serde_json::from_str(&data).unwrap();

            println!("Container: {}", id);

            println!(
                "OCI Version: {}",
                spec.oci_version
            );

            println!(
                "Command: {:?}",
                spec.process.args
            );

            println!(
                "Working Dir: {}",
                spec.process.cwd
            );
        }
    }
}
