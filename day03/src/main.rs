use std::env;
use std::fs;
use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;

fn read_puzzle_input() -> String {
    let args: Vec<String> = env::args().collect();
    let file_path = match args.len() {
        1 => "sample_input.txt",
        2 => &args[1],
        _ => "sample_input.txt",
    };
    let contents = fs::read_to_string(file_path)
        .expect("Failed to read input file {file_path}");
    contents
}

fn index_of_newline(s: String) -> u32 {
    let chars = s.chars();
    let mut index: u32 = 0;
    for c in chars {
        index += 1;
        if c == '\n' {
            break
        }
    }
    index
}

fn find_badges(puzzle_input: String) {
    let mut priorities: HashMap<char, u32> = HashMap::new();
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .collect();

    for (mut index, letter) in alphabet.iter().enumerate() {
        index += 1;
        priorities.insert(*letter, index.try_into().unwrap());
    }

    let mut stream: Vec<char> = Vec::new();
    let mut count = 0;
    let chars = puzzle_input.trim().chars();
    for c in chars {
        match c {
            '\n' => {
                match count {
                    0|1 => {
                        count += 1;
                        continue
                    },
                    2 => break,
                    _ => (),
                }
            },
            _ => stream.push(c),
        }
    }
    println!("{:?}", stream);
}

fn find_common_elements(puzzle_input: String) -> Vec<u32> {
    let mut priorities: HashMap<char, u32> = HashMap::new();
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .collect();

    for (mut index, letter) in alphabet.iter().enumerate() {
        index += 1;
        priorities.insert(*letter, index.try_into().unwrap());
    }

    // println!("{:?}", priorities);

    let mut errors: Vec<u32> = Vec::new();
    let lines = puzzle_input.trim().split("\n");
    for line in lines {
        let line_length_half = line.len() / 2;
        let first_half: Vec<char> = line[..line_length_half]
            .chars()
            .collect();
        let second_half: Vec<char> = line[line_length_half..]
            .chars()
            .collect();
        let mut hash1: HashSet<char> = HashSet::new();
        let mut hash2: HashSet<char> = HashSet::new();
        for ch in first_half {
            hash1.insert(ch);
        }
        for ch in second_half {
            hash2.insert(ch);
        }
        let items = hash1.intersection(&hash2);
        // println!("{:?}", items);
        for item in items {
            // println!("{}", priorities[item]);
            errors.push(priorities[item]);
        }
    }

    errors
}

fn main() {
    let contents = read_puzzle_input();
    println!("{}", contents);

    // let errors = find_common_elements(contents);
    // println!("{:?}", errors.iter().sum::<u32>());
    find_badges(contents);
}
