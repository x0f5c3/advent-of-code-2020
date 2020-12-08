#[macro_use]
extern crate clap;

use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::fmt;


#[derive(Debug)]
enum Day1Errors {
    P1NotFound,
    P2NotFound,
}

impl fmt::Display for Day1Errors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::P1NotFound => write!(f, "Couldn't find 2 numbers that sum up to 2020"),
            Self::P2NotFound => write!(f, "Couldn't find 3 numbers that sum up to 2020"),
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
        (@subcommand p2 =>
            (about: "Show solution to part 2")
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
        Some("p2") => {
            let (first, second, third, multi) = part_2(&data)?;
            println!("{} * {} * {} = {}", first, second, third, multi);
        }
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

fn part_2(data: &str) -> Result<(u32,u32,u32,u32), Box<dyn Error>> {
    let mut arr = data.lines().map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
    arr.sort_unstable();
    for i in 0..=(arr.len() - 2) {
        let mut l = i + 1;
        let mut r = arr.len() - 1;
        while l < r {
            if (arr[i] + arr[l] + arr[r]) == 2020 {
                let multi = arr[i] * arr[l] * arr[r];
                return Ok((arr[i], arr[l], arr[r], multi));
            }
            else if (arr[i] + arr[l] + arr[r]) < 2020 {
                l += 1;
            }
            else {
                r -= 1;
            }
        }
    }
    Err(Box::new(Day1Errors::P2NotFound))

}