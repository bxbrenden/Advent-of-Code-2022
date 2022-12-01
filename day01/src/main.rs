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

    // Break into groups separated by blank lines.
    // Each group is one elf's calories
    let spl1: Vec<&str> = puzzle_input.trim().split("\n\n").collect();

    let mut calorie_sums: Vec<i32> = Vec::new();
    for s in spl1 {
        let spl: Vec<&str> = s.trim().split("\n").collect();
        let mut sum: i32 = 0;
        for sp in spl {
            let num: i32 = sp.parse().unwrap();
            sum += num
        }
        calorie_sums.push(sum);
    }

    let answer: i32 = *calorie_sums.iter().max().unwrap();
    println!("{}", answer);
}
