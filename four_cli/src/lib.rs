use clap::Parser;
use cli::{Cli, Commands};
use command::Command;

mod cli;
mod command;
mod template;

pub async fn run() -> color_eyre::eyre::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Sfn(cmd) => cmd.run().await,
    }
}
