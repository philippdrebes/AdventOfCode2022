use core::iter::Peekable;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{env, io};

struct DirectoryNode {
    children: Vec<Node>,
    size: i32,
}

struct FileNode {
    size: i32,
    name: String,
}

enum Node {
    Directory(DirectoryNode),
    File(FileNode),
}

struct ChangeDirectoryCommand {
    target: String,
}

struct ListCommand {}

trait Command {
    fn execute<I>(&self, iter: &mut Peekable<I>, current_node: &mut Node) -> Node
    where
        I: Iterator<Item = String>,
        Self: Sized;
}

impl Command for ListCommand {
    fn execute<I>(&self, iter: &mut Peekable<I>, current_node: &mut Node) -> Node
    where
        I: Iterator<Item = String>,
    {
        


        Node::File(FileNode {
            size: 0,
            name: "".to_string(),
        })
    }
}

impl Command for ChangeDirectoryCommand {
    fn execute<I>(&self, iter: &mut Peekable<I>, current_node: &mut Node) -> Node
    where
        I: Iterator<Item = String>,
    {
        Node::File(FileNode {
            size: 0,
            name: "".to_string(),
        })
    }
}

fn parse_input<I>(iter: &mut Peekable<I>) -> Node
where
    I: Iterator<Item = String>,
{
    iter.next();
    let mut node = Node::Directory(DirectoryNode {
        children: vec![],
        size: 0,
    });

    let mut current_node = node;

    'outer: loop {
        let next: Option<String> = iter.next();
        if next == None {
            break 'outer;
        }

        let line = next.unwrap();
        let args = line.split(" ").collect::<Vec<&str>>();
        let command = args[1];

        let cmd: Box<dyn Command> = match command {
            "cd" => Box::new(ChangeDirectoryCommand {
                target: args[2].to_string(),
            }),
            "ls" => Box::new(ListCommand {}),
            _ => panic!("not implemented"),
        };

        cmd.execute(iter, &mut current_node)
    }

    node
}

#[cfg(test)]
mod tests {
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

        // assert_eq!(func, 95437);
    }
}

fn main() -> io::Result<()> {
    println!("Puzzle 07");

    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut iter = reader
        .lines()
        .map(|x| x.expect("wrong line input"))
        .peekable();

    parse_input(&mut iter);

    Ok(())
}
