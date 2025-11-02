mod app;
mod cli;

use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

use self::app::App;

#[allow(clippy::unnecessary_wraps)]
fn main() -> Result<()> {
    let args = cli::Cli::parse();

    let config_dir = args
        .config_dir
        .or_else(|| {
            dirs::config_dir().map(|mut path| {
                path.push("paper-tasks");
                path
            })
        })
        .unwrap_or_else(|| PathBuf::from("./"));

    match args.command.unwrap_or_default() {
        cli::Commands::Pipe => todo!(),
        cli::Commands::Print => todo!(),
        cli::Commands::Show => {
            let mut terminal = ratatui::init();
            let result = App::default().run(&mut terminal);
            ratatui::restore();
            result
        }
    }
}
