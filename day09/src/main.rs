use std::collections::HashSet;
use std::env;
use std::fs;

const RADIX: u32 = 10;

#[derive(Debug)]
struct Pos {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Rope {
    head: Pos,
    tail: Pos,
    visited: HashSet<(i32, i32)>,
}

impl Pos {
    fn new(x: i32, y:i32) -> Self {
        Pos { x: x, y: y }
    }
}

impl Rope {
    fn new() -> Self {
        let mut head = Pos::new(0, 0);
        let mut tail = Pos::new(0, 0);
        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        visited.insert((0, 0));
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

fn parse_steps(puz: &String) -> Vec<(char, i32)> {
    let mut steps: Vec<(char, i32)> = Vec::new();
    let spl = puz.trim().split("\n");
    for sp in spl {
        println!("{}", sp);
        let way: char = sp.chars().nth(0).unwrap();
        let num: u32 = sp.chars().nth(2).unwrap().to_digit(RADIX).unwrap();
        steps.push((way, num as i32));
    }

    steps
}

fn take_steps(steps: Vec<(char, i32)>, rope: Rope) -> () {
    for step in steps.iter() {
        // println!("{:?}", step.1);
        match step.0 {
            'U' => (),  // Up means tail.y increases
            'L' => (),  // Left means tail.x decreases
            'R' => (),  // Right means tail.x increases
            'D' => (),  // Down means tail.y decreases
            _ => {
                println!("Unexpected direction: {}", step.0);
                break;
            }
        }
    }
}

fn main() {
    let puz: String = read_puzzle_input();
    println!("{}", puz);

    let mut rope = Rope::new();
    println!("{:?}", rope);

    let steps = parse_steps(&puz);
    println!("{:?}", steps);

    take_steps(steps, rope);
}
