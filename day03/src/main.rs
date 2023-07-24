use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{char, env, io};

fn main() -> io::Result<()> {
    println!("Puzzle 03");

    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let total_sum: u32 = reader
        .lines()
        .into_iter()
        .map(|x| x.expect("encountered unexpected value"))
        .map(|x| process_line(x))
        .sum();

    println!("total sum: {}", total_sum);

    Ok(())
}

fn process_line(input: String) -> u32 {
    let m = find_match(input).expect("no match found");
    let val = m as u32;

    if val > 97 {
        return val - 96;
    }
    return val - 38;
}

fn find_match(input: String) -> Result<char, Box<dyn std::error::Error>> {
    let content: Vec<char> = input.chars().collect();
    let mut pack1_idx = 0;
    let pack2_idx_lower = content.len() / 2;
    let mut pack2_idx = pack2_idx_lower;

    while pack1_idx < pack2_idx_lower {
        while pack2_idx < content.len() {
            if content[pack1_idx] == content[pack2_idx] {
                return Ok(content[pack1_idx]);
            }
            pack2_idx += 1;
        }
        pack2_idx = pack2_idx_lower;
        pack1_idx += 1;
    }

    return Err(Box::from("could not find match"));
}

// fn alphabet_position(text: &str) -> String {
//     let s = text
//         .chars()
//         .into_iter()
//         .filter(|&c| c.is_alphabetic())
//         .map(|c| c.to_ascii_uppercase())
//         .map(|c| c as u8)
//         .map(|c| (c - 64u8).to_string())
//         .collect();
//     s
// }
