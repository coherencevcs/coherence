pub mod ast;
pub mod chrpatch;
pub mod cli;
pub mod commands;
pub mod utils;

use crate::cli::{Cli, Commands};
use crate::commands::parse::main as parse;
use clap::Parser;
use colored::Colorize;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Parse { file }) => parse(file)?,
        None => {
            anyhow::bail!("Unknown command. Run {} for help.", "chr help".green());
        }
    };

    Ok(())
}
