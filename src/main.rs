mod app;
mod cli;
mod config;

use std::fs::create_dir_all;
use std::io::Write;
use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

use self::app::App;
use self::cli::Cli;
use self::config::{Config, ConfigData};

fn setup(args: &Cli) -> Result<Config> {
    let config_dir = args
        .config_dir
        .clone()
        .or_else(|| dirs::config_dir().map(|path| path.join("paper-tasks")))
        .unwrap_or_else(|| PathBuf::from("./"));
    if !config_dir.exists() {
        create_dir_all(&config_dir)?;
    }

    let config_path = config_dir.join("config.toml");
    if !config_path.exists() {
        let mut config_file = std::fs::File::create_new(&config_path)?;
        config_file.write_all(toml::to_string_pretty(&ConfigData::default())?.as_bytes())?;
    }
    let config_data = std::fs::read_to_string(&config_path)?;
    let config_data: ConfigData = toml::from_str(&config_data)?;

    let db_path = config_dir.join(
        config_data
            .db_path
            .unwrap_or_else(|| PathBuf::from("database.db")),
    );

    Ok(Config {
        config_dir,
        config_path,
        db_path,
    })
}

#[allow(clippy::unnecessary_wraps)]
fn main() -> Result<()> {
    let args = cli::Cli::parse();

    let config = setup(&args)?;

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
