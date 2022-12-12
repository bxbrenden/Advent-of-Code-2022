use std::env;
use std::fs;

fn read_puzzle_input() -> String {
    let args: Vec<String> = env::args().collect();
    let file_path = match args.len() {
        1 => "sample_input.txt",
        2 => &args[1],
        _ => "sample_input.txt",
    };

    let mut puz = fs::read_to_string(file_path)
        .expect("failed to read file input: {file_path}.");

    puz
}

fn show_status(cycle: &i32, x_reg: &i32) -> bool {
    if (cycle + 20) % 40 == 0 {
        println!("Cycle: {}, X: {}", cycle, x_reg);
        return true;
    }
    false
}

fn accumulate(cycle: &i32, x_reg: &i32, signal: &mut i32) {
    *signal += cycle * x_reg;
    println!("{} * {} = {}", cycle, x_reg, signal);
}

fn parse_instrs(puz: &String) -> i32 {
    let instrs = puz.trim().split("\n");
    let mut signal: i32 = 0;
    let mut cycle: i32 = 1;
    let mut x_reg: i32 = 1;
    show_status(&cycle, &x_reg);
    for inst in instrs {
        match inst {
            "noop" => {
                cycle += 1;
            }
            _ => {
                let num: i32 = inst.replace("addx ", "").parse().unwrap();
                cycle += 1;
                if show_status(&cycle, &x_reg) {
                    accumulate(&cycle, &x_reg, &mut signal);
                };
                x_reg += num;
                cycle += 1;
            }
        }
        if show_status(&cycle, &x_reg) {
            accumulate(&cycle, &x_reg, &mut signal);
        }
    }
    signal
}

fn main() {
    let puz: String = read_puzzle_input();
    //println!("{}", puz);

    let signal_total: i32 = parse_instrs(&puz);
    println!("Total: {}", signal_total);
}
