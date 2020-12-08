use serde::{Deserialize, Serialize};
use thiserror::Error;
use std::path::PathBuf;

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub session: String,
    pub input_files: Option<PathBuf>,
}

pub fn path() -> PathBuf {
    dirs::config_dir()
        .expect("Run with a user with a home dir")
        .join("AoC-rs")
        .join("2020.toml")
}
impl Config {
    pub fn save(&self) -> Result<(), Error> {
        let path = path();
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent)?;
            }
        }
        let serialized = toml::ser::to_string_pretty(self)?;
        std::fs::write(path, serialized.as_bytes()).map_err(Into::into)
    }
    pub fn load() -> Result<Self, Error> {
        let data = std::fs::read(path())?;
        toml::de::from_slice(&data).map_err(Into::into)
    }
    pub fn input_files(&self) -> PathBuf {
        match self.input_files {
            Some(ref input_files) => input_files.to_owned(),
            None => match std::env::current_dir() {
                Ok(current) => current.join("inputs"),
                Err(_) => dirs::data_dir()
                    .expect("Run with a user with a home dir")
                    .join("adventofcode")
                    .join("2020"),
            },
        }
    }
    pub fn input_for(&self, day: u8) -> PathBuf {
        self.input_files().join(format!("input-{:02}.txt", day))
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to deserialize config")]
    Malformed(#[from] toml::de::Error ),
    #[error("Failed to load config")]
    CouldNotLoad(#[from] std::io::Error),
    #[error("Failed to serialize")]
    CouldNotSerialize(#[from] toml::ser::Error),
}


// impl Config {
//     pub fn init(session: String) ->
// }