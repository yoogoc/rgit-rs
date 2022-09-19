mod commands;
mod constant;
mod error;

use clap::{Args, Parser, Subcommand};
use commands::init::init;

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
}

fn main() {
    let result = match Cli::parse().command {
        Commands::Clone { remote: _ } => Ok(()),
        Commands::Init => init(),
    };
    match result {
        Ok(_) => {}
        Err(err) => {
            panic!("{}", err)
        }
    }
}
