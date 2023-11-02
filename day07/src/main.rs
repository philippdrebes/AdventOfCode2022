use core::iter::Peekable;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{env, io};

fn main() -> io::Result<()> {
    println!("Puzzle 07");

    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let file = File::open(file_path).expect("File not found");
    let reader = BufReader::new(file);

    let (mut lines, mut sum) = (
        reader
            .lines()
            .map(|x| x.expect("valid line string"))
            .peekable(),
        0,
    );
    sh(&mut lines, &mut sum);
    println!("{}", sum);

    Ok(())
}

pub fn sh(lines: &mut Peekable<impl Iterator<Item = String>>, sum: &mut usize) -> usize {
    let mut size = 0;
    while let Some(i) = lines.next() {
        match i.as_str() {
            "$ cd .." => break,
            _ if i.starts_with("$ l") => {
                size = std::iter::from_fn(|| lines.next_if(|i| !i.starts_with('$')))
                    .filter(|i| !i.starts_with('d'))
                    .map(|i| i.split(' ').next().unwrap().parse::<usize>().unwrap())
                    .sum()
            }
            _ => size += sh(lines, sum),
        }
    }
    if size <= 100_000 {
        *sum += size;
    }
    size
}

#[cfg(test)]
mod tests {
    use crate::sh;

    #[test]
    fn calculate_correct_size() {
        let test_data = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

        let (mut lines, mut sum) = (test_data.lines().map(|x| x.to_string()).peekable(), 0);
        sh(&mut lines, &mut sum);
        assert_eq!(sum, 95437);
    }
}
