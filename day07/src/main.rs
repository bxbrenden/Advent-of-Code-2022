use std::env;
use std::fs;

fn read_puzzle_input() -> String {
    let args: Vec<String> = env::args().collect();

    let file_path = match args.len() {
        1 => "sample_input.txt",
        2 => &args[1],
        _ => "sample_input.txt",
    };

    let mut puz = fs::read_to_string(file_path).expect("Failed to read input file {file_path}");

    puz = puz.trim().to_string();
    puz
}

fn process_command(cmd: &str, cwd: &str) -> String {
    if cmd.contains("$ cd") {
        println!("Command is a change directory: {}", cmd);
        cmd.chars().collect::<&str>()[5..].to_string()
    } else if cmd.contains("$ ls") {
        println!("Command is a list: {}", cmd);
    }
}

fn walk_tree(lines: &Vec<&str>) -> () {
    let cwd: String = String::new();
    for (line_num, line) in lines.iter().enumerate() {
        match line.chars().nth(0).unwrap() {
            '$' => {
                println!("Line number {} is a command.", line_num + 1);
                process_command(line, &cwd);
            },
            _ => (),
        }
    }
}

fn main() {
    let puz = read_puzzle_input();

    let lines: Vec<&str> = puz.split("\n").collect();
    walk_tree(&lines);
}
