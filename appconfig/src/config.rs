use std::{
    fs::File,
    path::{Path, PathBuf},
};

use directories::ProjectDirs;
use error_stack::{IntoReport, Report, ResultExt};
use serde::Deserialize;
use tracing::info;

#[derive(Default, Deserialize)]
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

pub fn config_from_env() -> Result<Config, Report<Error>> {
    let database_url = std::env::var("DATABASE_URL")
        .change_context(Error::EnvironmentVariableError("DATABASE_URL".to_string()))?;

    let config = Config { database_url };

    Ok(config)
}

pub fn read() -> Result<Config, Report<Error>> {
    let config = match init_data_dir() {
        Ok(config_dir) => {
            let config_file_path = config_dir.join("config.yaml");

            config_from_yaml(&config_file_path)?
        }
        Err(mut e) => config_from_env().map_err(|err| {
            e.change_context(err.into_error())
                .change_context(Error::NoValidConfigError)
        })?,
    };

    Ok(config)
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("could not retrieve a valid home")]
    HomeDirError,

    #[error("environment variable {0} not found")]
    EnvironmentVariableError(String),

    #[error("could not open config file")]
    FileOpenError,

    #[error("could not create config dir")]
    ConfigDirCreationError,

    #[error("reading configuration file")]
    IoError,

    #[error("parsing configuration")]
    ParseError,

    #[error("could not get a valid config")]
    NoValidConfigError,
}
