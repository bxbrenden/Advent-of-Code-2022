use std::env;
use std::fs;
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

fn find_triples(s: &String) -> Vec<usize> {
    let chars = s.chars();
    let mut index: usize = 0;
    let mut count: usize = 0;
    let mut triples: Vec<usize> = Vec::new();
    for c in chars {
        if c == '\n' {
            count += 1;
            if count % 3 == 0 && count != 0 {
                triples.push(index)
            }
        }
        index += 1;
    }
    triples
}

fn part2_answer(puzzle_input: &String, indices: Vec<usize>) -> u32 {
    let mut final_count: u32 = 0;
    let mut priorities: HashMap<char, u32> = HashMap::new();
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .collect();

    for (mut index, letter) in alphabet.iter().enumerate() {
        index += 1;
        priorities.insert(*letter, index.try_into().unwrap());
    }

    let mut chunks: Vec<&str> = Vec::new();
    for (i, _) in indices.iter().enumerate() {
        match i {
            0 => {
                chunks.push(&puzzle_input[..indices[i]]);
            }
            _ => {
                chunks.push(&puzzle_input[indices[i-1]..indices[i]]);
            }
        }
    }
    // println!("{:?}", chunks);

    for chunk in chunks {
        // println!("Starting chunk: {chunk}");
        let mut lines: Vec<&str> = Vec::new();
        for line in chunk.trim().split("\n") {
            lines.push(line);
        }
        let mut h1: HashSet<char> = HashSet::new();
        let mut h2: HashSet<char> = HashSet::new();
        let mut h3: HashSet<char> = HashSet::new();
        // Have to make a 4th HashSet for the result of
        // intersection h1 and h2. Rust's HashSets suck:
        // https://github.com/rust-lang/rfcs/issues/2023
        let mut h4: HashSet<char> = HashSet::new();

        for c in lines[0].chars() {
            h1.insert(c);
        }
        // println!("h1 contains: {:?}", h1);

        for c in lines[1].chars() {
            h2.insert(c);
        }
        // println!("h2 contains: {:?}", h2);

        for c in lines[2].chars() {
            h3.insert(c);
        }
        // println!("h3 contains: {:?}", h3);

        let h1h2 = h1.intersection(&h2);
        for c in h1h2 {
            h4.insert(*c);
        }
        // println!("h4 contains: {:?}", h4);

        let h3h4 = h3.intersection(&h4);
        for entry in h3h4 {
            // println!("{}", entry);
            final_count += priorities[entry];
        }
    }
    final_count
}

fn find_common_elements(puzzle_input: &String) -> Vec<u32> {
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
    // println!("{}", contents);

    let _errors = find_common_elements(&contents);
    // println!("{:?}", errors.iter().sum::<u32>());
    let triples = find_triples(&contents);
    // println!("{:?}", triples);
    let answer2 = part2_answer(&contents, triples);
    println!("{}", answer2);
}
