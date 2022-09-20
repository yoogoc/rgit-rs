mod commands;
mod constant;
mod error;
mod model;
pub mod utils;

use clap::{Parser, Subcommand};
use commands::{add, init};

/// A fictional versioning CLI
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
        /// The remote to clone
        paths: Vec<String>,
    },
}

fn main() {
    let result = match Cli::parse().command {
        Commands::Clone { remote: _ } => Ok(()),
        Commands::Init => init(),
        Commands::Add { paths } => add(paths),
    };
    match result {
        Ok(_) => {}
        Err(err) => {
            panic!("{}", err)
        }
    }
}
