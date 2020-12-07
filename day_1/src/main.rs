#[macro_use]
extern crate clap;

use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::fmt;


#[derive(Debug)]
enum Day1Errors {
    P1NotFound,
}

impl fmt::Display for Day1Errors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::P1NotFound => write!(f, "Couldn't find 2 numbers that sum up to 2020"),
        }
    }
}
impl Error for Day1Errors {}



fn main() -> Result<(),Box<dyn Error>>  {
    let matches = clap_app!(day_1 =>
        (version: "0.1")
        (author: "x0f5c3 <x0f5c3@tutanota.com>")
        (about: "Solutions to day 1 of AoC")
        (@subcommand p1 =>
            (about: "Show solution to part 1")
        )
    ).get_matches();
    let mut input = File::open("./input")?;
    let mut data = String::new();
    input.read_to_string(&mut data)?;
    match matches.subcommand_name() {
        Some("p1") => {
            let (num, wanted, multi) = part_1(&data)?;
            println!("{} * {} = {}",num, wanted, multi);
        },
        _ => {
            println!("No subcommand given or not supported");
        },
    }

    Ok(())

}


fn part_1(data: &str) -> Result<(u32,u32,u32), Box<dyn Error>> {
    let mut arr = data.lines().map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
    arr.sort_unstable();
    for num in &arr {
        let wanted = 2020 - num;
        if let Ok(_found_index) = arr.binary_search(&wanted) {
            println!("Found {} {}", num, wanted);
            let multiply = num * wanted;
            println!("Result of the multiplication: {}", multiply);
            return Ok((*num, wanted, multiply))
        }
        else {
            continue;
        }
    }
    Err(Box::new(Day1Errors::P1NotFound))
}
