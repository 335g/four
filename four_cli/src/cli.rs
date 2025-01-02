use clap::{Parser, Subcommand};

use crate::command::sfn::StepFunctionCommand;

#[derive(Parser)]
#[command(version, about, long_about=None)]
pub struct Cli {
    #[command(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Subcommand)]
pub(crate) enum Commands {
    Sfn(StepFunctionCommand),
}
