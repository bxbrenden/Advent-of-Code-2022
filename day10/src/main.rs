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

fn show_status(cycle: &i32, x_reg: &i32) {
    println!("Cycle: {}, X: {}", cycle, x_reg);
}

fn parse_instrs(puz: &String) -> () {
    let instrs = puz.trim().split("\n");
    let mut cycle: i32 = 1;
    let mut x_reg: i32 = 1;
    show_status(&cycle, &x_reg);
    for inst in instrs {
        //show_status(&cycle, &x_reg);
        match inst {
            "noop" => {
                cycle += 1;
            }
            _ => {
                let num: i32 = inst.replace("addx ", "").parse().unwrap();
                cycle += 1;
                show_status(&cycle, &x_reg);
                x_reg += num;
                cycle += 1;
            }
        }
        show_status(&cycle, &x_reg);
    }
}

fn main() {
    let puz: String = read_puzzle_input();
    //println!("{}", puz);

    parse_instrs(&puz);
}
