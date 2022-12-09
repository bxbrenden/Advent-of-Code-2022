use std::env;
use std::fs;

fn read_puzzle_input() -> String {
    let args: Vec<String> = env::args().collect();
    let file_path = match args.len() {
        1 => "sample_input.txt",
        2 => &args[1],
        _ => "sample_input.txt",
    };

    let puz = fs::read_to_string(file_path)
        .expect("Failed to read file \"{file_path}\"");
    puz
}

fn main() {
    let puz: String = read_puzzle_input();
    println!("{}", puz);
}
