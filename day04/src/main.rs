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

    fn overlapping(&self) -> bool {
        let overlap1 =
            self.first.upper >= self.second.lower && self.first.lower <= self.second.lower;
        let overlap2 =
            self.second.upper >= self.first.lower && self.second.lower <= self.first.lower;
        return overlap1 || overlap2;
    }
}

#[cfg(test)]
mod tests {
    use crate::Bounds;
    use crate::Pair;

    #[test]
    fn fully_contained() {
        let p = Pair {
            first: Bounds { lower: 5, upper: 7 },
            second: Bounds { lower: 2, upper: 9 },
        };
        assert_eq!(p.one_fully_contains_other(), true);
    }

    #[test]
    fn not_fully_contained() {
        let p = Pair {
            first: Bounds { lower: 1, upper: 7 },
            second: Bounds { lower: 5, upper: 9 },
        };
        assert_eq!(p.one_fully_contains_other(), false);
    }

    #[test]
    fn first_overlapping_second_by_one() {
        let p = Pair {
            first: Bounds { lower: 7, upper: 9 },
            second: Bounds { lower: 5, upper: 7 },
        };
        assert_eq!(p.overlapping(), true);
    }

    #[test]
    fn second_overlapping_first_by_one() {
        let p = Pair {
            first: Bounds { lower: 5, upper: 7 },
            second: Bounds { lower: 7, upper: 9 },
        };
        assert_eq!(p.overlapping(), true);
    }

    #[test]
    fn overlap_in_single_section() {
        let p = Pair {
            first: Bounds { lower: 6, upper: 6 },
            second: Bounds { lower: 4, upper: 6 },
        };
        assert_eq!(p.overlapping(), true);
    }

    #[test]
    fn overlap_in_multiple_sections() {
        let p = Pair {
            first: Bounds { lower: 2, upper: 6 },
            second: Bounds { lower: 4, upper: 8 },
        };
        assert_eq!(p.overlapping(), true);
    }

    #[test]
    fn no_overlap() {
        let p = Pair {
            first: Bounds { lower: 2, upper: 6 },
            second: Bounds { lower: 60, upper: 82 },
        };
        assert_eq!(p.overlapping(), false);
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
        .filter(|x| x.one_fully_contains_other() || x.overlapping())
        .count();

    println!("Found {} contained and overlapping sections", count);

    Ok(())
}

fn parse_bounds(input: &String) -> Bounds {
    let pair = input.split("-").collect::<Vec<&str>>();
    return Bounds {
        lower: pair[0].parse::<u8>().unwrap(),
        upper: pair[1].parse::<u8>().unwrap(),
    };
}
