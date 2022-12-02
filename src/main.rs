
use clap::{Parser, Subcommand};

mod helpers;
mod day1;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Day1 {
        #[arg(long)]
        inputfile: String
    }
}

fn main() {
    let cli = Cli::parse();
    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Day1 { inputfile }) => {
            day1::main::run(inputfile.to_string());
        }
        None => {}
    }
}