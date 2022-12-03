use std::env;
use std::fs;
use std::process;

#[derive(Debug)]
struct RPS {
    // A struct for holding Rock, Paper, Scissors moves
    opponent: Vec<char>,
    mine: Vec<char>,
}

fn read_strategy_guide(guide: &str) -> RPS {
    let mut opponent: Vec<char> = Vec::new();
    let mut mine: Vec<char> = Vec::new();
    let spl = guide.trim().split("\n");
    for line in spl {
        let sp = line.trim().split("\n");
        let opp = line.chars().nth(0).unwrap();
        let my = line.chars().nth(2).unwrap();
        opponent.push(opp);
        mine.push(my);
    }

    RPS { opponent: opponent, mine: mine }
}

fn resolve_rps(rps: RPS) -> u32 {
    // loss = 0, draw = 3, win = 6
    // rock = 1, paper = 2, scissors = 3
    let mut points: u32 = 0;
    let zipped = rps.opponent.iter().zip(rps.mine.iter());
    for z in zipped {
        match z {
            ('A', 'X') => points += 4,
            ('A', 'Y') => points += 8,
            ('A', 'Z') => points += 3,
            ('B', 'X') => points += 1,
            ('B', 'Y') => points += 5,
            ('B', 'Z') => points += 9,
            ('C', 'X') => points += 7,
            ('C', 'Y') => points += 2,
            ('C', 'Z') => points += 6,
            _ => println!("{:?} was not a match for any pattern", z),
        }
    }
    points
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = match args.len() {
        1 => "sample_input.txt",
        2 => &args[1],
        _ => "sample_input.txt",
    };
    let contents = fs::read_to_string(file_path)
        .expect("Could not read file contents");

    let my_rps = read_strategy_guide(&contents);
    // println!("{:?}", my_rps);
    let points = resolve_rps(my_rps);
    println!("Total points: {}", points);
}
