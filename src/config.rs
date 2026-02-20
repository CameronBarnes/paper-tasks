use std::path::PathBuf;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ConfigData {
    pub db_path: Option<PathBuf>,
}

pub struct Config {
    pub config_dir: PathBuf,
    pub config_path: PathBuf,
    pub db_path: PathBuf,
}
