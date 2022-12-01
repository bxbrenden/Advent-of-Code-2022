use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = match args.len() {
        1 => "puzzle_input.txt",
        2 => &args[1],
        _ => "puzzle_input.txt",
    };
    println!("The input file is {}", input_file);

    let puzzle_input = match fs::read_to_string(input_file) {
        Ok(puz) => puz,
        Err(e) => {
            println!("Failed to read file with error:\n{e}");
            process::exit(1);
        }
    };

    println!("{}", puzzle_input);
}
