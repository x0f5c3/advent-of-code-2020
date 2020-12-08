
use std::path::PathBuf;
use structopt::StructOpt;
use thiserror::Error;
pub mod website;
pub mod config;
pub mod input;
#[derive(Debug, StructOpt)]
enum Opt {
    Init {
        #[structopt(short, long)]
        cookie: String,
        #[structopt(short, long, parse(from_os_str))]
        input_dir: Option<PathBuf>
    },
    Download {
        #[structopt(short, long)]
        day: u8
    }
}


fn main() -> Result<(), Error>{
    let opt = Opt::from_args();
    match opt {
        Opt::Init{cookie, input_dir} => {
            let config = config::Config {
                session: cookie,
                input_files: input_dir
            };
            config.save()?;
        },
        Opt::Download{day} => {
            let config = config::Config::load()?;
            website::get_input(&config, day)?;
        } 
    }
    Ok(())
}

#[derive(Debug, Error)]
enum Error {
    #[error(transparent)]
    Config(#[from] config::Error),
    #[error(transparent)]
    Website(#[from] website::Error),
}