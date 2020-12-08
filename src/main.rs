
#[macro_use]
extern crate clap;
use std::path::PathBuf;

pub mod website;
pub mod config;


fn main() -> Result<(), config::Error>{
    let matches = clap_app!(day_1 =>
        (version: "0.1")
        (author: "x0f5c3 <x0f5c3@tutanota.com>")
        (about: "Solutions to day 1 of AoC")
        (@subcommand cookie =>
            (about: "Set cookie")
            (@arg cookie: +required -c --cookie "Add session key")
        )
        (@subcommand set_dir =>
            (about: "Init configuration")
            (@arg cookie: +required -c --cookie "Add session key")
            (@arg dir: +required -d --dir "Add input dir")
        )
    ).get_matches();
    match matches.subcommand_name() {
        Some("cookie") => {
            if let Some(session) = matches.subcommand_matches("cookie") {
                if let Some(val) = session.value_of("cookie") {
                    let config = config::Config {
                        session: val.to_string(),
                        input_files: None,
                    };
                    config.save()?;
                }
            }
        },
        Some("set_dir") => {
            if let Some(setit) = matches.subcommand_matches("set_dir") {
                if let Some(cookie) = setit.value_of("cookie") {
                    if let Some(dire) = setit.value_of("dir") {
                        let config = config::Config {
                            session: cookie.to_string(),
                            input_files: Some(PathBuf::from(dire))
                        };
                        config.save()?;
                    }
                }
            }
        },
        _ => {},
    }
    Ok(())
}