use std::env;
use std::fs::File;
use std::io::{self, BufReader, prelude::*};

// Find the top three Elves carrying the most Calories. How many Calories are those Elves carrying in total?
fn main() -> io::Result<()> {
    println!("Puzzle 01");

    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut calorie_counter: u32 = 0;
    let mut top3: [u32; 3] = [0, 0, 0];

    for line in reader.lines() {
        let val = line.expect("oops not Ok");
        if val.eq("") {
            top3.sort();
            if calorie_counter > top3[0] {
                top3[0] = calorie_counter;
            }

            calorie_counter = 0;
            continue;
        }

        let calories: u32 = val.parse().expect("not an integer");
        calorie_counter += calories;
    }

    let sum: u32 = top3.iter().sum();

    println!("Top 3 calories combined: {}", sum);

    Ok(())
}

