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
        .expect("Failed to read input file {input_file}");

    puz
}

fn puz_to_vec(puz: String) -> Vec<Vec<char>> {
    let mut main_vec: Vec<Vec<char>> = Vec::new();
    let spl = puz.trim().split("\n");
    for s in spl {
        let mut inner_v: Vec<char> = Vec::new();
        for c in s.chars() {
            inner_v.push(c);
        }
        main_vec.push(inner_v);
    }

    main_vec
}

fn main() {
    let puz = read_puzzle_input();

    let main_vec = puz_to_vec(puz);
    println!("{:?}", main_vec);
}
