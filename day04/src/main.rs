use std::env;
use std::fs;

const RADIX: u32 = 10;

fn read_puzzle_input() -> String {
    let args: Vec<String> = env::args().collect();
    let file_path = match args.len() {
        1 => "sample_input.txt",
        2 => &args[1],
        _ => "sample_input.txt",
    };
    let contents = fs::read_to_string(file_path).expect("Failed to read input file {file_path}");
    contents
}

fn get_bounds(puz: &String) -> Vec<i32> {
    let mut bounds: Vec<i32> = Vec::new();
    for line in puz.trim().split("\n") {
        let spl = line.split(",");
        for sp in spl {
            let mut s = sp.split("-");
            for sx in s {
                let n: i32 = sx.parse::<i32>().unwrap();
                bounds.push(n);
            }
        }
    }
    bounds
}

fn main() {
    let puz = read_puzzle_input();
    let bounds = get_bounds(&puz);
    println!("{:?}", bounds);
}
