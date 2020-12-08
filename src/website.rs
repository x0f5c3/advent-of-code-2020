use config::Config;
use std::path::PathBuf;
use anyhow::Result;
use reqwest::blocking::Client;


pub fn url_for_day(day: u8) -> String {
    format!("https://adventofcode.com/2020/day/{}", day)
}


pub fn url_for_input(day: u8) -> String {
    format!("{}/input", url_for_day(day))
}


pub fn get_input(config: &Config, day: u8) -> Result<()> {
    let path = PathBuf::from(config.get_str(&format!("input_path_{}", day))?);
    if path.exists() {
        return Ok(())
    }
    let session = config.get_str("session")?;
    let client = Client::builder()
        .gzip(true)
        .timeout(std::time::Duration::from_secs(5))
        .build()?;
    let mut response = client
        .get(&url_for_input(day))
        .header(
            reqwest::header::COOKIE,
            format!("session={}", session),
        )
        .send()?;
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent)?;
        }
    }
    let mut file = std::fs::File::create(path)?;
    response.copy_to(&mut file)?;
    Ok(())
    
}