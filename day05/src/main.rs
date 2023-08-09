use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{char, env, io};

struct Movement {
    count: i32,
    source: i32,
    target: i32,
}

fn main() -> io::Result<()> {
    println!("Puzzle 05");

    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader
        .lines()
        .map(|x| x.expect("valid line string"))
        .collect();

    let stack_lines = lines
        .clone()
        .into_iter()
        .take_while(|x| !x.is_empty())
        .collect();

    let mut stacks: Vec<Vec<char>> = parse_stacks(stack_lines);

    let movements: Vec<Movement> = lines
        .into_iter()
        .skip_while(|x| !x.is_empty())
        .skip(1)
        .map(parse_movement)
        .collect();

    for m in movements {
        // print_stacks(stacks.clone());
        // println!(
        //     "source: {}, target: {}, count: {}",
        //     m.source, m.target, m.count
        // );
        // println!("-------------------------");

        for _ in 0..m.count {
            let mut item: char = ' ';
            let src_idx: usize = m.source as usize - 1;
            let trgt_idx: usize = m.target as usize - 1;
            if let Some(src) = stacks.get_mut(src_idx) {
                item = src.pop().expect("element in stack");
            }

            if let Some(trgt) = stacks.get_mut(trgt_idx) {
                trgt.push(item);
            }
        }
    }

    print_stacks(stacks);

    Ok(())
}

fn parse_stacks(input: Vec<String>) -> Vec<Vec<char>> {
    let mut stacks: Vec<Vec<char>> = Vec::new();
    for line in input {
        for (i, c) in line.chars().skip(1).step_by(4).enumerate() {
            if let None = stacks.get(i) {
                stacks.push(Vec::new());
            }

            if c.is_numeric() {
                break;
            }

            if c.is_alphabetic() {
                stacks[i].insert(0, c);
            }
        }
    }
    return stacks;
}

fn parse_movement(input: String) -> Movement {
    let commands = input.split(" ").map(str::to_owned).collect::<Vec<String>>();
    return Movement {
        count: commands[1].parse::<i32>().unwrap(),
        source: commands[3].parse::<i32>().unwrap(),
        target: commands[5].parse::<i32>().unwrap(),
    };
}

fn print_stacks(stacks: Vec<Vec<char>>) {
    println!("    BOTTOM ------> TOP");
    for (i, s) in stacks.iter().enumerate() {
        println!(
            "{}: {}",
            i,
            s.iter()
                .map(|x| x.to_string())
                .fold("".to_string(), |acc, item| format!("{} {}", acc, item))
        );
    }

    let top_of_each = stacks
        .iter()
        .map(|x| x.last().expect("each stack has at least one item"))
        .map(|x| x.to_string())
        .fold("".to_string(), |acc, item| acc + &item);

    println!("Top of each stack: {}", top_of_each);
}

#[cfg(test)]
mod tests {
    use crate::parse_movement;

    #[test]
    fn parse_movement_returns_correct_values() {
        let input = "move 12 from 8 to 7";
        let act = parse_movement(input.to_string());
        assert_eq!(act.count, 12);
        assert_eq!(act.source, 8);
        assert_eq!(act.target, 7);
    }
}
