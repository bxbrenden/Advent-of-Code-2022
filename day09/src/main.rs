use std::env;
use std::fs;

#[derive(Debug)]
struct Pos {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Rope {
    head: Pos,
    tail: Pos,
    visited: Vec<(i32, i32)>,
}

impl Pos {
    fn new(x: i32, y:i32) -> Self {
        Pos { x: x, y: y }
    }
}

impl Rope {
    fn new() -> Self {
        let head = Pos::new(0, 0);
        let tail = Pos::new(0, 0);
        let visited = vec!((0, 0));
        Rope { head: head, tail: tail, visited: visited }
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
        .expect("Failed to read file \"{file_path}\"");
    puz
}

fn main() {
    let puz: String = read_puzzle_input();
    println!("{}", puz);

    let rope = Rope::new();
    println!("{:?}", rope);
}
