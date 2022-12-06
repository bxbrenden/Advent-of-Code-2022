use std::collections::HashSet;
use std::env;
use std::fs;

fn read_puzzle_input() -> String{
    let args: Vec<String> = env::args().collect();
    let file_path = match args.len() {
        1 => "sample_input.txt",
        2 => &args[1],
        _ => "sample_input.txt",
    };

    let mut puz = fs::read_to_string(file_path)
        .expect("Failed to open input file {file_path}");

    puz = puz.trim().to_string();
    puz
}

fn find_sop_marker(puz: &String) -> () {
    let seen: HashSet<&str> = HashSet::new();

    let len = &puz.len();

    for (i, c) in puz.chars().enumerate() {
        if i < len - 3 {
            let slice = &puz[i..i+4];
            println!("{}", slice);
        }
    }
}

fn main() {
    let puz = read_puzzle_input();
    // println!("{}", puz);

    find_sop_marker(&puz);
}
