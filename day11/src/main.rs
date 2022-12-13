use std::env;
use std::fs;

#[derive(Debug)]
struct Monkey {
    start: Vec<i32>,
    oper: String,
    test: i32,
    t: usize,
    f: usize,
}

impl Monkey {
    fn new(start: Vec<i32>,
           oper: String,
           test: i32,
           t: usize,
           f: usize) -> Self {
        Monkey { start, oper, test, t, f}
    }
}

fn read_puzzle_input() -> String {
    let args: Vec<String> = env::args().collect();
    let file_path = match args.len() {
        1 => "sample_input.txt",
        2 => &args[1],
        _ => "sample_input.txt",
    };

    let puz = fs::read_to_string(file_path)
        .expect("Failed to read puzzle input: {file_path}");
    puz
}

fn main() {
    let puz = read_puzzle_input();
    println!("{}", puz);
    let m1 = Monkey::new(
        vec!(23, 31, 6),
        "old * 3".to_string(),
        7,
        6usize,
        3usize
    );
    println!("{:?}", m1);
}
