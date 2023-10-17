use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{char, env, io};

struct Parser {
    position: i32,
    buffer: Vec<char>,
}

impl Parser {
    fn new() -> Self {
        Parser {
            position: 0,
            buffer: Vec::with_capacity(3),
        }
    }

    pub fn find_start_position(&mut self, stream: &String) -> i32 {
        for c in stream.chars() {
            self.buffer[0] = c;
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use crate::Parser;

    #[test]
    fn finds_correct_position() {
        let parser = &mut Parser::new();
        assert_eq!(
            parser.find_start_position(&String::from("bvwbjplbgvbhsrlpgdmjqwftvncz")),
            5
        );
    }
}

fn main() -> io::Result<()> {
    println!("Puzzle 06");

    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let file = File::open(file_path).expect("File not found");
    let mut reader = BufReader::new(file);

    let parser = &mut Parser::new();

    let mut buf = Vec::<u8>::new();
    while reader
        .read_until(b'\n', &mut buf)
        .expect("read_until failed")
        != 0
    {
        // this moves the ownership of the read data to s
        // there is no allocation
        let s = String::from_utf8(buf).expect("from_utf8 failed");

        parser.find_start_position(&s);

        // this returns the ownership of the read data to buf
        // there is no allocation
        buf = s.into_bytes();
        buf.clear();
    }

    return Ok(());
}
