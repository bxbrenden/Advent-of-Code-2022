use std::env;
use std::fs;

fn read_puzzle_input() -> String {
    let args: Vec<String> = env::args().collect();
    let file_path = match args.len() {
        1 => "sample_input.txt",
        2 => &args[1],
        _ => "sample_input.txt",
    };

    let puzzle_input =
        fs::read_to_string(file_path).expect("Failed to read input file \"{file_path}\"");

    puzzle_input
}

fn create_crates(puz: &String) -> () {
    let spl = puz.trim().split("\n\n");
    // println!("{:?}", spl);

    let chunks: Vec<&str> = spl.collect();
    let crate_chunk = chunks[0];
    println!("Crate Chunk:\n{crate_chunk}");

    let cspl: Vec<&str> = crate_chunk.trim().split("\n").collect();
    let crate_nums: Vec<&str> = cspl[cspl.len() - 1].trim().split_whitespace().collect();
    println!("Crate nums: {:?}", crate_nums);
    let num_columns = crate_nums.len();
    println!("Number of columns: {num_columns}");

    let crates = &cspl[0..cspl.len() -1];
    for (index, crate_row) in crates.iter().enumerate() {
        println!("{:?}", crate_row.replace("[", " ").replace("]"," "));
    }
}

fn main() {
    let puz = read_puzzle_input();
    // println!("{}", puz);
    create_crates(&puz);
}
