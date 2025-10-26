use clap::{Parser, Subcommand};
use std::io;

mod helpers;
mod commands;
use commands::{apply, add, delete};

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    mode: Command,
    #[arg(required=true, index=1)]
    app: String,
    #[arg(required=true, index=2)]
    config: String,
}

#[derive(Subcommand)]
enum Command {
    Apply,
    Add,
    Del,
}

fn main() -> io::Result<()> {
    // the path we are looking for is $HOME/.config/{app_name}/possible-configs/{config_name}
    let args = Args::parse();

    let home_dir = match dirs::home_dir() {
        Some(path) => path,
        None => {
            eprintln!("Error: Could not find home directory.");
            std::process::exit(1);
        }
    };

    let _ = match args.mode {
        Command::Apply => {
            return apply(args.app, args.config, home_dir);
        },
        Command::Add => {
            return add(args.app, args.config, home_dir);
        },
        Command::Del => {
            return delete(args.app, args.config, home_dir);
        },
    };
}
