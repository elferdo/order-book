use std::{
    fs::File,
    path::{Path, PathBuf},
};

use directories::ProjectDirs;
use serde::Deserialize;
use tracing::info;

#[derive(Deserialize)]
pub struct Config {
    pub database_url: String,
}

pub fn init_data_dir() -> Result<PathBuf, Error> {
    let Some(user_dirs) = ProjectDirs::from("com", "elferdo", "reverse_markets") else {
        return Err(Error::DirError);
    };

    let d = user_dirs.data_local_dir().to_owned();

    if let Ok(exists) = d.try_exists() {
        if !exists {
            std::fs::create_dir(&d)?;

            info!("Created data directory {}", d.display());
        }

        Ok(d)
    } else {
        Err(Error::DirError)
    }
}

pub fn config_from_yaml(name: &Path) -> Result<Config, Error> {
    let file = File::open(name)?;

    let contents: Config = serde_yaml::from_reader(file)?;

    Ok(contents)
}

pub fn read() -> Result<Config, Error> {
    let config_dir = init_data_dir()?;

    let config_file_path = config_dir.join("config.yaml");
    let config = config_from_yaml(&config_file_path)?;

    Ok(config)
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("directories")]
    DirError,

    #[error("reading configuration file")]
    DirIoError(#[from] std::io::Error),

    #[error("reading configuration")]
    ParseError(#[from] serde_yaml::Error),
}
