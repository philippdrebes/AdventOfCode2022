use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{env, io};

struct Bounds {
    lower: u8,
    upper: u8,
}

struct Pair {
    first: Bounds,
    second: Bounds,
}

impl Pair {
    fn one_fully_contains_other(&self) -> bool {
        let first_in_second =
            self.first.lower >= self.second.lower && self.first.upper <= self.second.upper;
        let second_in_first =
            self.second.lower >= self.first.lower && self.second.upper <= self.first.upper;

        return first_in_second || second_in_first;
    }
}

fn main() -> io::Result<()> {
    println!("Puzzle 04");

    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let count = reader
        .lines()
        .map(|x| x.expect("wrong line input"))
        .map(|x| x.split(",").map(str::to_owned).collect::<Vec<String>>())
        .map(|x| Pair {
            first: parse_bounds(&x[0]),
            second: parse_bounds(&x[1]),
        })
        .filter(|x| x.one_fully_contains_other())
        .count();

    println!("Found {} contained sections", count);

    Ok(())
}

fn parse_bounds(input: &String) -> Bounds {
    let pair = input.split("-").collect::<Vec<&str>>();
    return Bounds {
        lower: pair[0].parse::<u8>().unwrap(),
        upper: pair[1].parse::<u8>().unwrap(),
    };
}
