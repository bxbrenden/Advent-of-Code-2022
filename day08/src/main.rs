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
    for (y, row) in tree_grid.iter().enumerate() {
        for (x, column) in row.iter().enumerate() {
            println!("({y}, {x}): {column}");
            match is_visible(&tree_grid, x, y) {
                (true, true, true) => println!("Visible from top, left, and right!"), //          111
                (true, true, false) => println!("Visible from top and left, but not right."),//   110
                (true, false, true) => println!("Visible from top and right but not left."),//    101
                (true, false, false) => println!("Visible from top but not left or right."),//    100
                (false, true, true) => println!("Visible from the left and right but not top."),//011
                (false, true, false) => println!("Visible from left, but not top or right."),//   010
                (false, false, true) => println!("Visible from right, but not top or left."),//   001
                (false, false, false) => println!("Not visible."),//                              000
            }
        }
    }

    println!("Outer visible: {visible}");
}

fn is_visible(tree_grid: &Vec<Vec<u32>>, x_loc: usize, y_loc: usize) -> (bool, bool, bool) {
    let mut x = x_loc;
    let mut y = y_loc;
    let grid_dims = get_grid_dimensions(tree_grid);
    let max_x = grid_dims.1 as usize;
    let max_y = grid_dims.0 as usize;
    // Check for top visibility by decrementing y values to 0
    let mut top_highest: u32 = 0;
    let cur_height = tree_grid[y_loc][x_loc];
    while y > 0 {
        y -= 1;
        let test = tree_grid[y][x];
        if test > top_highest {
            top_highest = test;
        }
    }

    // Check for left visibility by decrementing x values to 0
    let mut left_highest: u32 = 0;
    while x > 0 {
        x -= 1;
        let test = tree_grid[y][x];
        if test > left_highest {
            left_highest = test;
        }
    }

    // Check for right visibility by incrementing x values
    let mut right_highest: u32 = 0;
    while x < max_x - 1 {
        x += 1;
        let test = tree_grid[y][x];
        if test > right_highest {
            right_highest = test;
        }
    }

    (cur_height > top_highest,
     cur_height > left_highest,
     cur_height > right_highest,)
}

fn main() {
    let puz = read_puzzle_input();
    println!("{}", puz);

    let tree_grid = get_tree_grid(puz);

    find_visible_trees(&tree_grid);
}
