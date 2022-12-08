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

fn find_visible_trees(tree_grid: &Vec<Vec<u32>>) -> usize {
    let mut visible: usize = 0;
    let grid_dims = get_grid_dimensions(tree_grid);
    // let inner_grid_size: usize = (grid_dims.0 - 2) * (grid_dims.1 - 2);
    // let outer_visible: usize = (grid_dims.0 * grid_dims.1) - inner_grid_size;
    // visible += outer_visible;
    // The product of grid dimensions minus (dims.0 -1 * dims.1 - 1)
    //   will always equal the visible outer trees
    for (y, row) in tree_grid.iter().enumerate() {
        for (x, column) in row.iter().enumerate() {
            println!("({y}, {x}): {column}");
            let viz = is_visible(&tree_grid, x, y);
            match viz.iter().any(|&b| b == true) {
                true => {
                    println!("Visible!");
                    visible += 1;
                },
                false => println!("Not visible"),
            }
        }
    }
    visible
}

fn is_visible(tree_grid: &Vec<Vec<u32>>, x_loc: usize, y_loc: usize) -> Vec<bool> {
    if y_loc == 0 || x_loc == 0 {
        println!("Returning early");
        return vec!(true);
    }
    let mut x = x_loc;
    let mut y = y_loc;
    let grid_dims = get_grid_dimensions(tree_grid);
    let max_x = grid_dims.1 - 1 as usize;
    let max_y = grid_dims.0 - 1 as usize;
    println!("Max X: {max_x}, Max Y: {max_y}");
    if y_loc == max_y || x_loc == max_x {
        println!("Returning early");
        return vec!(true);
    }
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
    // Reset x
    x = x_loc;
    y = y_loc;
    let mut left_highest: u32 = 0;
    while x > 0 {
        x -= 1;
        let test = tree_grid[y][x];
        if test > left_highest {
            left_highest = test;
        }
    }

    // Check for right visibility by incrementing x values
    // Reset x
    x = x_loc;
    y = y_loc;
    let mut right_highest: u32 = 0;
    while x < max_x {
        x += 1;
        let test = tree_grid[y][x];
        if test > right_highest {
            right_highest = test;
        }
    }

    // Check for bottom visibility by incrementing y values
    // Reset y
    y = y_loc;
    x = x_loc;
    let mut bottom_highest: u32 = 0;
    while y < max_y {
        y += 1;
        let test = tree_grid[y][x];
        if test > bottom_highest {
            bottom_highest = test;
        }
    }

    let answer = vec!(cur_height > top_highest,
                      cur_height > left_highest,
                      cur_height > right_highest,
                      cur_height > bottom_highest,
    );
    println!("{:?}", answer);
    answer
}

fn main() {
    let puz = read_puzzle_input();
    println!("{}", puz);

    let tree_grid = get_tree_grid(puz);

    let visible = find_visible_trees(&tree_grid);
    println!("{} visible trees", visible);
}
