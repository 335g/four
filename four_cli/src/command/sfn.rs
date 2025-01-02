use std::path::PathBuf;

use async_trait::async_trait;
use clap::Parser;
use crossterm::event;
use ratatui::{DefaultTerminal, Frame};

use super::Command;

#[derive(Parser)]
pub struct StepFunctionCommand {
    #[arg(short, long)]
    template: PathBuf,
}

#[async_trait]
impl Command for StepFunctionCommand {
    async fn run(self) -> color_eyre::eyre::Result<()> {
        let terminal = ratatui::init();
        let result = run(terminal);
        ratatui::restore();
        Ok(())
    }
}

fn run(mut terminal: DefaultTerminal) -> color_eyre::eyre::Result<()> {
    loop {
        terminal.draw(render)?;
        if matches!(event::read()?, event::Event::Key(_)) {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame) {
    frame.render_widget("hello world", frame.area());
}
