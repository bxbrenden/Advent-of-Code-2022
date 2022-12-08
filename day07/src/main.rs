use std::env;
use std::fs;

fn read_puzzle_input() -> String {
    let args: Vec<String> = env::args().collect();

    let file_path = match args.len() {
        1 => "sample_input.txt",
        2 => &args[1],
        _ => "sample_input.txt",
    };

    let mut puz = fs::read_to_string(file_path).expect("Failed to read input file {file_path}");

    puz = puz.trim().to_string();
    puz
}

fn main() {
    let puz = read_puzzle_input();

    let lines: Vec<&str> = puz.split("\n").collect();
    for line in &lines {
        println!("{}", line);
    }
}
