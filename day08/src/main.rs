use std::env;
use std::fs;

const RADIX: u32 = 10;

fn read_puzzle_input() -> String {
    let args: Vec<String> = env::args().collect();
    let file_path = match args.len() {
        1 => "sample_input.txt",
        2 => &args[1],
        _ => "sample_input.txt",
    };

    let puz = fs::read_to_string(file_path)
        .expect("Failed to read input file {input_file}");

    puz
}

fn get_tree_grid(puz: String) -> Vec<Vec<u32>> {
    let mut tree_grid: Vec<Vec<u32>> = Vec::new();
    let spl = puz.trim().split("\n");
    for s in spl {
        let mut inner_v: Vec<u32> = Vec::new();
        for c in s.chars() {
            let n: u32 = c.to_digit(RADIX).unwrap();
            inner_v.push(n);
        }
        tree_grid.push(inner_v);
    }

    tree_grid
}

fn get_grid_dimensions(tree_grid: &Vec<Vec<u32>>) -> (usize, usize) {
    let height: usize = tree_grid.len();
    let mut width: usize = 0;
    for row in tree_grid.iter() {
        width = row.len();
        break;
    }

    (height, width)
}

fn find_visible_trees(tree_grid: &Vec<Vec<u32>>) -> () {
    let grid_dims = get_grid_dimensions(tree_grid);
    for (index, row) in tree_grid.iter().enumerate() {
        println!("{}: {:?}", index, row);
    }
}

fn main() {
    let puz = read_puzzle_input();

    let tree_grid = get_tree_grid(puz);
    println!("{:?}", tree_grid);

    find_visible_trees(&tree_grid);
}
