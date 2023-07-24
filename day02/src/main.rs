use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{env, io};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Hand {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Debug, PartialEq, Eq)]
enum Outcome {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

fn main() -> io::Result<()> {
    println!("Puzzle 02");

    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut total_score: u32 = 0;

    for line in reader.lines() {
        let val = line.expect("oops not Ok");
        let game = val.split(" ").collect::<Vec<&str>>();

        let opponent_hand = match game[0] {
            "A" => Hand::Rock,
            "B" => Hand::Paper,
            "C" => Hand::Scissors,
            _ => panic!("Invalid hand"),
        };

        let desired_outcome = match game[1] {
            "X" => Outcome::Lose,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => panic!("Invalid outcome"),
        };

        let my_hand = get_my_hand(&desired_outcome, &opponent_hand);

        total_score += my_hand as u32 + desired_outcome as u32;
    }

    println!("Score: {}", total_score);

    Ok(())
}

fn get_my_hand(desired_outcome: &Outcome, opponent_hand: &Hand) -> u8 {
    return match desired_outcome {
        &Outcome::Win => match opponent_hand {
            Hand::Rock => Hand::Paper as u8,
            Hand::Paper => Hand::Scissors as u8,
            Hand::Scissors => Hand::Rock as u8,
        },
        &Outcome::Lose => match opponent_hand {
            Hand::Rock => Hand::Scissors as u8,
            Hand::Paper => Hand::Rock as u8,
            Hand::Scissors => Hand::Paper as u8,
        },
        &Outcome::Draw => *opponent_hand as u8,
    };
}

