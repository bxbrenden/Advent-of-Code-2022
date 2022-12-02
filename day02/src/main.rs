use std::env;
use std::fs;
use std::process;

#[derive(Debug)]
struct RPS<'a> {
    // A struct for holding Rock, Paper, Scissors moves
    opponent_moves: Vec<&'a str>,
    my_moves: Vec<&'a str>,
}

fn read_strategy_guide(guide: &str) -> RPS {
    let mut opponent: Vec<&str> = Vec::new();
    let mut mine: Vec<&str> = Vec::new();
    let spl = guide.trim().split("\n");
    for line in spl {
        let sp = line.trim().split("\n");
        let opp = &line.chars().nth(0).unwrap();
        let my = &line.chars().nth(2).unwrap();
        match opp {
            'A' => opponent.push("A"),
            'B' => opponent.push("B"),
            'C' => opponent.push("C"),
            _ => {
                println!("Unknown character {}", opp.to_string());
                process::exit(1);
            }
        }

        match my {
            'X' => mine.push("X"),
            'Y' => mine.push("Y"),
            'Z' => mine.push("Z"),
            _ => {
                println!("Unknown character {}", opp.to_string());
                process::exit(1);
            }
        }
    }

    RPS { opponent_moves: opponent, my_moves: mine }
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
    println!("{:?}", my_rps);
}
