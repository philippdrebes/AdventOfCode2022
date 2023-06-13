use std::{env, io};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq, Eq)]
enum Hand {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

const LOST: u8 = 0;
const DRAW: u8 = 3;
const WIN: u8 = 6;

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

        let opponent = map_hand(game[0]);
        let myself = map_hand(game[1]);
        let outcome = get_game_outcome(&myself, &opponent);

        total_score += myself as u32 + outcome as u32;
    }

    println!("Score: {}", total_score);

    Ok(())
}

fn map_hand(hand: &str) -> Hand {
    match hand {
        "A" | "X" => Hand::Rock,
        "B" | "Y" => Hand::Paper,
        "C" | "Z" => Hand::Scissors,
        _ => panic!("Invalid hand"),
    }
}

fn get_game_outcome(player1: &Hand, player2: &Hand) -> u8 {
    if player1 == &Hand::Rock && player2 == &Hand::Scissors {
        return WIN;
    }

    if player1 == &Hand::Scissors && player2 == &Hand::Paper {
        return WIN;
    }

    if player1 == &Hand::Paper && player2 == &Hand::Rock {
        return WIN;
    }

    if player1.eq(player2) {
        return DRAW;
    }

    return LOST;
}