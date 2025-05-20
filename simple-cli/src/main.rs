use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "hello cli")]
#[command(about = "A simple CLI with subcommands", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Greet { name: String },

    Bye { name: String },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Greet { name } => {
            println!("Hello {}!", name);
        }
        Commands::Bye { name } => {
            println!("Bye {}!", name);
        }
    }
}
