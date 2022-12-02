use std::env;
use std::fs;
use std::process;

#[derive(Debug)]
struct RPS<'a> {
    opponent_moves: Vec<&'a str>,
    my_moves: Vec<&'a str>,
}

fn read_strategy_guide(guide: &str) -> RPS {
    let opponent: Vec<&str> = Vec::new();
    let mine: Vec<&str> = Vec::new();
    let spl = guide.trim().split("\n");
    // for line in spl {
    //     println!("{}", line);
    // }

    RPS { opponent_moves: opponent, my_moves: mine }
}

fn main() {
    let file_path = "sample_input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Could not read file contents");

    let my_rps = read_strategy_guide(&contents);
    println!("{:?}", my_rps);
}
