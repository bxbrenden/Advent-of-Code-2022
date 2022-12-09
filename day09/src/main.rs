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
    fn new(x: i32, y: i32) -> Self {
        Pos { x: x, y: y }
    }
}

impl Rope {
    fn new() -> Self {
        let mut head = Pos::new(0, 0);
        let mut tail = Pos::new(0, 0);
        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        visited.insert((0, 0));
        Rope {
            head: head,
            tail: tail,
            visited: visited,
        }
    }
}

fn read_puzzle_input() -> String {
    let args: Vec<String> = env::args().collect();
    let file_path = match args.len() {
        1 => "sample_input.txt",
        2 => &args[1],
        _ => "sample_input.txt",
    };

    let puz = fs::read_to_string(file_path).expect("Failed to read file \"{file_path}\"");
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

/// Check if the head and tail of a rope are touching
fn touching(head: &Pos, tail: &Pos) -> bool {
    // Case 1: Head covers tail
    if head.x == tail.x && head.y == tail.y {
        return true;
    }
    // Case 2: Head is one space to the right (x+1) of tail
    else if head.x - tail.x == 1 && head.y == tail.y {
        return true;
    }
    false
}

/// Ensure the tail stays adjacent to the head.
/// Also, update list of visited coordinates for the tail.
fn manage_tail(rope: &Rope) {
    println!(
        "Touching? {}. {:?}, {:?}",
        touching(&rope.head, &rope.tail),
        rope.head,
        rope.tail
    );
}

fn take_steps(steps: Vec<(char, i32)>, mut rope: Rope) -> () {
    for step in steps.iter() {
        let num_moves = step.1;
        match step.0 {
            'U' => {
                // Up means head.y increases
                for _ in 0..num_moves {
                    rope.head.y += 1;
                    manage_tail(&rope);
                }
            }
            'L' => {
                // Left means head.x decreases
                for _ in 0..num_moves {
                    rope.head.x -= 1;
                    manage_tail(&rope);
                }
            }
            'R' => {
                // Right means head.x increases
                for _ in 0..num_moves {
                    rope.head.x += 1;
                    manage_tail(&rope);
                }
            }
            'D' => {
                // Down means head.y decreases
                for _ in 0..num_moves {
                    rope.head.y -= 1;
                    manage_tail(&rope);
                }
            }
            _ => {
                println!("Unexpected direction: {}", step.0);
                break;
            }
        }
        println!("{:?}", rope);
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
