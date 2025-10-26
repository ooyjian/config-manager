use clap::{Parser, Args, Subcommand};
use std::io;

mod helpers;
mod commands;
use commands::{apply, add, delete};

#[derive(Parser)]
struct App {
    #[command(subcommand)]
    mode: Command,
}

#[derive(Args)]
struct Opts {
    #[arg(required=true, index=1)]
    app: String,
    #[arg(required=true, index=2)]
    config: String,
}

#[derive(Subcommand)]
enum Command {
    Apply(Opts),
    Add(Opts),
    Del(Opts),
}

fn main() -> io::Result<()> {
    // the path we are looking for is $HOME/.config/{app_name}/possible-configs/{config_name}
    let subcmd = App::parse();

    let home_dir = match dirs::home_dir() {
        Some(path) => path,
        None => {
            eprintln!("Error: Could not find home directory.");
            std::process::exit(1);
        }
    };

    let _ = match subcmd.mode {
        Command::Apply(args) => {
            return apply(args.app, args.config, home_dir);
        },
        Command::Add(args) => {
            return add(args.app, args.config, home_dir);
        },
        Command::Del(args) => {
            return delete(args.app, args.config, home_dir);
        },
    };
}
