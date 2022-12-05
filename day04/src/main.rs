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

fn find_overlaps(bounds: Vec<i32>) -> () {
    let chunks = bounds.chunks(4);
    for (index, chunk) in chunks.enumerate() {
        println!(
            "Line: {}, Chunk 1: {:?}, Chunk 2: {:?}",
            index + 1,
            &chunk[0..2],
            &chunk[2..]
        );
        // No overlaps if first upper bound is lower than second lower bound
        if chunk[1] < chunk[2] {
            println!("No overlap!");
            continue;
        }
        // Full overlap if perfectly equal upper and lower bounds
        else if chunk[0] == chunk[2] && chunk[1] == chunk[3] {
            println!("Perfect overlap!");
        } // Overlap if lower bounds same and second upper higher than first upper
          else if chunk[0] == chunk[2] && chunk[1] < chunk[3] {
            println!("Overlap!");
        } // Partial overlap if first upper bound higher than second lower bound
          else if chunk[2] < chunk[3] {
            println!("Partial overlap!");
          }
    }
}

fn main() {
    let puz = read_puzzle_input();
    let bounds = get_bounds(&puz);
    // println!("{:?}", bounds);
    find_overlaps(bounds);
}
