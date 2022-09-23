mod commands;
mod constant;
mod error;
mod file;
mod index;
mod model;
mod tree;
pub mod utils;

use std::process::exit;

use clap::{Parser, Subcommand};
use commands::{add, commit, init};

#[derive(Debug, Parser)] // requires `derive` feature
#[clap(name = "rgit")]
#[clap(about = "rgit", long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Init,
    #[clap(arg_required_else_help = true)]
    Clone {
        /// The remote to clone
        remote: String,
    },
    #[clap(arg_required_else_help = true)]
    Add {
        /// pathsepc
        paths: Vec<String>,
    },
    Commit {
        /// message
        #[clap(short, long, value_parser)]
        message: Option<String>,
    },
}

fn main() {
    let result = match Cli::parse().command {
        Commands::Clone { remote: _ } => Ok(()),
        Commands::Init => init(),
        Commands::Add { paths } => add(paths),
        Commands::Commit { message } => commit(message),
    };
    match result {
        Ok(_) => {}
        Err(err) => {
            eprintln!("{}", err.to_string());
            exit(1);
        }
    }
}
