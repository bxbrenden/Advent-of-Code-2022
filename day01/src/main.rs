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

    let spl: Vec<&str> = puzzle_input
        .trim()
        .split("\n")
        .collect();

    let mut calories_list: Vec<i32> = Vec::new();
    for s in spl {
        if s != "" {
            let calories: i32 = s.parse().unwrap();
            calories_list.push(calories);
        }
    }

    for c in calories_list {
        println!("Parsed {c} into i32");
    }
}
