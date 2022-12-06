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
    let len = &puz.len();

    for (i, c) in puz.chars().enumerate() {
        let mut seen: HashSet<char> = HashSet::new();

        if i < len - 3 {
            let slice = &puz[i..i+4];
            println!("{}", slice);
            for c in slice.chars() {
                seen.insert(c);
            }
        }
        if seen.len() == 4 {
            println!("Index: {}", i + 4);
            break;
        }
    }
}

fn main() {
    let puz = read_puzzle_input();
    // println!("{}", puz);

    find_sop_marker(&puz);
}
