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
    let mut visible: usize = 0;
    let grid_dims = get_grid_dimensions(tree_grid);
    let inner_grid_size: usize = (grid_dims.0 - 2) * (grid_dims.1 - 2);
    let outer_visible: usize = (grid_dims.0 * grid_dims.1) - inner_grid_size;
    visible += outer_visible;
    // The product of grid dimensions minus (dims.0 -1 * dims.1 - 1)
    //   will always equal the visible outer trees
    for (index_y, row) in tree_grid.iter().enumerate() {
        for (index_x, column) in row.iter().enumerate() {
            println!("({index_y}, {index_x}): {column}");
        }
        // println!("{}: {:?}", index, row);
    }

    println!("Outer visible: {visible}");
}

fn main() {
    let puz = read_puzzle_input();
    println!("{}", puz);

    let tree_grid = get_tree_grid(puz);

    find_visible_trees(&tree_grid);
}
