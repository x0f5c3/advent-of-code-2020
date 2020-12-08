use reqwest::blocking::Client;
use thiserror::Error;
use crate::config::Config;

pub fn url_for_day(day: u8) -> String {
    format!("https://adventofcode.com/2020/day/{}", day)
}


pub fn url_for_input(day: u8) -> String {
    format!("{}/input", url_for_day(day))
}


pub fn get_input(config: &Config, day: u8) -> Result<(), Error> {
    let path = config.input_for(day);
    if path.exists() {
        return Ok(())
    }
    let session = &config.session;
    let client = Client::builder()
        .gzip(true)
        .timeout(std::time::Duration::from_secs(5))
        .build()
        .map_err(Error::ClientBuilder)?;
    let mut response = client
        .get(&url_for_input(day))
        .header(
            reqwest::header::COOKIE,
            format!("session={}", session),
        )
        .send()
        .map_err(Error::RequestingInput)?
        .error_for_status()
        .map_err(Error::ResponseStatus)?;
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent)?;
        }
    }
    let mut file = std::fs::File::create(path)?;
    response.copy_to(&mut file).map_err(Error::Downloading)?;
    Ok(())
    
}

#[derive(Debug,Error)]
pub enum Error {
    #[error("Failed to build reqwest client")]
    ClientBuilder(#[source] reqwest::Error),
    #[error("requesting input file")]
    RequestingInput(#[source] reqwest::Error),
    #[error("response status unsuccessful")]
    ResponseStatus(#[source] reqwest::Error),
    #[error("Failed IO")]
    IOError(#[from] std::io::Error),
    #[error("downloading to local file")]
    Downloading(#[source] reqwest::Error),
}