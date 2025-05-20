use clap::{Parser, Subcommand};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: simple-cli <name>");

        std::process::exit(1);
    }

    let name: &String = &args[1];
    println!("Hello {}!", { name });
}
