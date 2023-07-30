use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{char, env, io};

fn main() -> io::Result<()> {
    println!("Puzzle 03");

    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut total_sum: u32 = 0;

    let lines: Vec<String> = reader.lines().map(|x| x.expect("asdf")).collect();

    let mut step = 3;
    for (i, line) in lines.iter().step_by(step).enumerate() {
        let idx = i * 3;
        let values = vec![line, &lines[idx + 1], &lines[idx + 2]];
        total_sum += process_lines(values).expect("could not process lines");
    }

    println!("total sum: {}", total_sum);

    Ok(())
}

fn process_lines(input: Vec<&String>) -> Result<u32, Box<dyn std::error::Error>> {
    // let mut bags: Vec<Vec<char>> = Vec::new();
    // for i in input {
    //     bags.push(i.chars().collect());
    // }

    for c0 in input[0].chars().into_iter() {
        let found_match = input[1..]
            .iter()
            .map(|b| b.contains(c0))
            .reduce(|c, p| c && p);

        if found_match.expect("process_lines: could not find match in bags") {
            return Ok(get_char_value(c0.clone()));
        }
    }

    return Err(Box::from(
        "process_lines: did not find any matches. this should not happen.",
    ));
}

fn process_line(input: String) -> u32 {
    let m = find_match(input).expect("no match found");
    return get_char_value(m);
}

fn get_char_value(chr: char) -> u32 {
    let val = chr as u32;

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
