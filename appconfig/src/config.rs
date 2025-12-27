use std::{
    fs::File,
    path::{Path, PathBuf},
};

use directories::ProjectDirs;
use error_stack::{IntoReport, Report, ResultExt};
use serde::Deserialize;
use tracing::info;

#[derive(Deserialize)]
pub struct Config {
    pub database_url: String,
}

pub fn init_data_dir() -> Result<PathBuf, Report<Error>> {
    let Some(user_dirs) = ProjectDirs::from("com", "elferdo", "reverse_markets") else {
        return Err(Error::HomeDirError.into_report());
    };

    let d = user_dirs.data_local_dir().to_owned();

    if let Ok(exists) = d.try_exists() {
        if !exists {
            Err(Error::HomeDirError.into_report())
        } else {
            Ok(d)
        }
    } else {
        let dir_path = format!("{}", d.display());

        std::fs::create_dir(&d)
            .change_context(Error::ConfigDirCreationError)
            .attach(dir_path)?;

        info!("Created data directory {}", d.display());

        Ok(d)
    }
}

pub fn config_from_yaml(name: &Path) -> Result<Config, Report<Error>> {
    let printable_path = format!("{}", name.display());

    let file = File::open(name)
        .change_context(Error::FileOpenError)
        .attach(printable_path)?;

    let contents: Config =
        serde_yaml::from_reader(file).map_err(|e| Error::ParseError.into_report().attach(e))?;

    Ok(contents)
}

pub fn read() -> Result<Config, Report<Error>> {
    let config_dir = init_data_dir()?;

    let config_file_path = config_dir.join("config.yaml");
    let config = config_from_yaml(&config_file_path)?;

    Ok(config)
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("could not retrieve a valid home")]
    HomeDirError,

    #[error("could not open config file")]
    FileOpenError,

    #[error("could not create config dir")]
    ConfigDirCreationError,

    #[error("reading configuration file")]
    IoError,

    #[error("parsing configuration")]
    ParseError,
}
