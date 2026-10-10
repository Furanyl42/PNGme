#![allow(unused)]
mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

use anyhow::Result;
use args::Cli;
use clap::Parser;

fn main() -> Result<()> {
    let cli = Cli::parse();
    commands::run(cli)?;
    Ok(())
}
