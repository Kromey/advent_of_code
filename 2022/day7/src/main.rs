use std::collections::HashMap;

use utils::*;

#[derive(Debug, PartialEq, Eq)]
enum Command<'a> {
    Cd(&'a str),
    CdRoot,
    CdUp,
    List,
}

#[derive(Debug)]
enum CommandError {
    UnknownCommand(String)
}

impl<'a> Command<'a> {
    fn from_str(s: &'a str) -> Result<Self, CommandError> {
        match s {
            "ls" => Ok(Command::List),
            "cd /" => Ok(Command::CdRoot),
            "cd .." => Ok(Command::CdUp),
            _ if &s[..3] == "cd " => Ok(Command::Cd(&s[3..])),
            _ => Err(CommandError::UnknownCommand(s.to_string())),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Listing<'a> {
    File(&'a str, usize),
    Directory(&'a str),
}

#[derive(Debug)]
enum ListingError {
    UnrecognizedFormat(String)
}

impl<'a> Listing<'a> {
    fn from_str(s: &'a str) -> Result<Self, ListingError> {
        if let Some((data, name)) = s.split_once(' ') {
            if let Ok(size) = data.parse::<usize>() {
                Ok(Listing::File(name, size))
            } else {
                Ok(Listing::Directory(name))
            }
        } else {
            Err(ListingError::UnrecognizedFormat(s.to_string()))
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Terminal<'a> {
    Command(Command<'a>),
    Listing(Listing<'a>),
}

#[derive(Debug)]
enum ParserError {
    CommandError(CommandError),
    ListingError(ListingError),
    UnexpectedInput(String),
}

impl From<CommandError> for ParserError {
    fn from(e: CommandError) -> Self {
        Self::CommandError(e)
    }
}

impl From<ListingError> for ParserError {
    fn from(e: ListingError) -> Self {
        Self::ListingError(e)
    }
}

fn parse_line(line: &str) -> Result<Terminal, ParserError> {
    match line.chars().next().unwrap() {
        '$' => Ok(Terminal::Command(Command::from_str(&line[2..])?)),
        'd' | '0'..='9' => Ok(Terminal::Listing(Listing::from_str(line)?)),
        _ => Err(ParserError::UnexpectedInput(format!("Bad Input: {line}"))),
    }
}

fn dir_size(dir: &String, filesystem: &HashMap<String, Vec<Listing>>) -> usize {
    if let Some(list) = filesystem.get(dir) {
        let mut size = 0;

        for l in list.iter() {
            match l {
                Listing::File(_, file_size) => size += *file_size,
                Listing::Directory(name) => {
                    let path = vec![dir.as_str(), name].join("/");
                    size += dir_size(&path, filesystem);
                }
            }
        }

        size
    } else {
        0
    }
}

fn main() -> Result<(), ParserError> {
    // let input = "$ cd /
    // $ ls
    // dir a
    // 14848514 b.txt
    // 8504156 c.dat
    // dir d
    // $ cd a
    // $ ls
    // dir e
    // 29116 f
    // 2557 g
    // 62596 h.lst
    // $ cd e
    // $ ls
    // 584 i
    // $ cd ..
    // $ cd ..
    // $ cd d
    // $ ls
    // 4060174 j
    // 8033020 d.log
    // 5626152 d.ext
    // 7214296 k";
    let input = read_puzzle_input!().unwrap();

    let mut pwd = vec![""];
    let mut filesystem: HashMap<_, Vec<_>> = HashMap::new();
    let mut dirs = vec![("".to_string(), 0)];

    for line in input.lines() {
        match parse_line(line.trim())? {
            Terminal::Command(command) => {
                match command {
                    Command::Cd(dir) => { pwd.push(dir); },
                    Command::CdUp => { pwd.pop(); },
                    Command::CdRoot => { pwd.truncate(1); },
                    Command::List => {},
                }
            }
            Terminal::Listing(list) => {
                if let Listing::Directory(name) = list {
                    let path = vec![pwd.join("/").as_str(), name].join("/");
                    dirs.push((path, 0));
                }
                filesystem.entry(pwd.join("/")).or_default().push(list);
            }
        }
    }

    for (dir, size) in dirs.iter_mut() {
        *size = dir_size(&dir.to_string(), &filesystem);
    }

    // Part 1
    let sum: usize = dirs.iter().filter_map(|(_, size)| {
        if *size <= 100_000 {
            Some(*size)
        } else {
            None
        }
    })
    .sum();
    println!("Sum of all directories up to 100KB: {sum}");

    // Part 2
    println!();

    let used_space = dirs[0].1;

    let total_space = 70_000_000;
    let free_space = total_space - used_space;
    let needed_space = 30_000_000;
    let to_free = needed_space - free_space;

    println!("Total disk space: {total_space}");
    println!("Available disk space: {free_space}");
    println!("Disk space needed for update: {needed_space}");
    println!("Need to free {to_free} to allow update to proceed");

    println!("\nAnalyzing disk space...\n");

    if let Some((name, size)) = dirs.iter()
        .filter(|(_, size)| *size >= to_free)
        .min_by_key(|(_, size)| size)
    {
        println!("Deleting directory {name} would free {size} and allow the update to proceed");
    } else {
        println!("ERR: No suitable directory found that would free enough space");
    }

    Ok(())
}
