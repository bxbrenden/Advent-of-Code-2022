use std::env;
use std::fs;

#[derive(Debug)]
struct Monkey {
    name: String,
    start: Vec<i32>,
    oper: String,
    test: i32,
    t: usize,
    f: usize,
}

impl Monkey {
    fn new(name: String, start: Vec<i32>, oper: String, test: i32, t: usize, f: usize) -> Self {
        Monkey {
            name,
            start,
            oper,
            test,
            t,
            f,
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

    let puz = fs::read_to_string(file_path).expect("Failed to read puzzle input: {file_path}");
    puz
}

fn parse_input(puz: &String, monkeys: &mut Vec<Monkey>) {
    let blocks = puz.trim().split("\n\n");
    for block in blocks {
        let lines = block.split("\n");
        for (index, line) in lines.enumerate() {
            let mut name: String = String::new();
            let mut start: Vec<i32> = Vec::new();
            let mut oper: String = String::new();
            let mut test: i32 = 0;
            let mut t: usize = 0;
            let mut f: usize = 0;
            match index {
                0 => name = line.to_string(),
                1 => {
                    line.replace("  Starting items: ", "")
                        .split(", ")
                        .map(|s| s.parse::<i32>().unwrap())
                        .map(|n| start.push(n));
                }
                2 => oper = line.replace("  Operation: ", "").to_string(),
                3 => {
                    test = line.replace("  Test: divisible by ", "")
                        .parse::<i32>()
                        .unwrap();
                }
                4 => {
                    t = line.replace("    If true: throw to monkey ", "")
                        .parse::<usize>()
                        .unwrap();
                }
                5 => {
                    f = line.replace("    If false: throw to monkey ", "")
                        .parse::<usize>()
                        .unwrap();
                }
                _ => panic!("Unexpected index in block!"),
            }
            let m = Monkey::new(name.clone(), start.clone(), oper.clone(), test, t, f);
            monkeys.push(m)
        }
    }
}

fn main() {
    let puz = read_puzzle_input();
    let mut monkeys: Vec<Monkey> = Vec::new();
    parse_input(&puz, &mut monkeys);
    println!("{:#?}", monkeys);

    // Let m1 = Monkey::new(
    //     "Monkey 0".to_string(),
    //     vec![23, 31, 6],
    //     "old * 3".to_string(),
    //     7,
    //     6usize,
    //     3usize,
    // );
    // Println!("{:?}", m1);
}
