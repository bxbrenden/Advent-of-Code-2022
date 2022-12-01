use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = match args.len() {
        1 => "puzzle_input.txt",
        2 => &args[1],
        _ => "puzzle_input.txt",
    };
    println!("The input file is {}", input_file);

    let puzzle_input = fs::read_to_string(input_file)
        .expect("Unable to read puzzle input file {input_file}");
}
