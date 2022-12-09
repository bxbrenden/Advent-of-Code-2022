use std::collections::HashSet;
use std::env;
use std::fs;

const RADIX: u32 = 10;

#[derive(Debug)]
struct Pos {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Rope {
    head: Pos,
    tail: Pos,
    visited: HashSet<(i32, i32)>,
}

impl Pos {
    fn new(x: i32, y: i32) -> Self {
        Pos { x: x, y: y }
    }
}

impl Rope {
    fn new() -> Self {
        let mut head = Pos::new(0, 0);
        let mut tail = Pos::new(0, 0);
        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        visited.insert((0, 0));
        Rope {
            head: head,
            tail: tail,
            visited: visited,
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

    let puz = fs::read_to_string(file_path).expect("Failed to read file \"{file_path}\"");
    puz
}

fn parse_steps(puz: &String) -> Vec<(char, i32)> {
    let mut steps: Vec<(char, i32)> = Vec::new();
    let spl = puz.trim().split("\n");
    for sp in spl {
        println!("{}", sp);
        let way: char = sp.chars().nth(0).unwrap();
        let num: u32 = sp.chars().nth(2).unwrap().to_digit(RADIX).unwrap();
        steps.push((way, num as i32));
    }

    steps
}

/// Check if the head and tail of a rope are touching
fn touching(head: &Pos, tail: &Pos) -> bool {
    // Case 1: Head covers tail
    if head.x == tail.x && head.y == tail.y {
        return true;
    }
    // Case 2: Head is one space to the right (x+1) of tail
    else if head.x - tail.x == 1 && head.y == tail.y {
        return true;
    }
    // Case 3: Head is one space to the left (x-1) of tail
    else if head.x - tail.x == -1 && head.y == tail.y {
        return true;
    }
    // Case 4: Head is one space above (y+1) tail
    else if head.x == tail.x && head.y - tail.y == 1 {
        return true;
    }
    // Case 5: Head is one space below (y-a) tail
    else if head.x == tail.x && head.y - tail.y == -1 {
        return true;
    }
    // Case 6: Head is one space right (x+1) and one space above (y+1) tail
    else if head.x - tail.x == 1 && head.y - tail.y == 1 {
        return true;
    }
    // Case 7: Head is one space right (x+1) and one space below (y-1) tail
    else if head.x - tail.x == 1 && head.y - tail.y == -1 {
        return true;
    }
    // Case 8: Head is one space left (x-1) and one space above (y+1) tail
    else if head.x - tail.x == -1 && head.y - tail.y == 1 {
        return true;
    }
    // Case 9: Head is one space left (x-1) and one space below (y-1) tail
    else if head.x - tail.x == -1 && head.y - tail.y == -1 {
        return true;
    }
    false
}

/// Ensure the tail stays adjacent to the head.
/// Also, update list of visited coordinates for the tail.
fn manage_tail(rope: &mut Rope) {
    let t1 = touching(&rope.head, &rope.tail);
    println!("Touching? {}. {:?}, {:?}", t1, &rope.head, &rope.tail);

    let head = &mut rope.head;
    let tail = &mut rope.tail;
    // If not touching, move tail to make it adjacent
    if t1 == false {
        // Case 1: head is two spaces right (x+2) of tail. Move tail one right (x+1)
        if head.x - tail.x == 2 && head.y == tail.y {
            tail.x += 1;
        } // Case 2: head is two spaces above (y+2) of tail. Move tail one up (y+1)
        else if head.x == tail.x && head.y - tail.y == 2 {
            tail.y += 1;
        } // Case 3: head is two spaces left (x-2) of tail. Move tail one left (x-1)
        else if head.x - tail.x == -2 && head.y == tail.y {
            tail.x -= 1;
        } // Case 4: head is two spaces below (y-2) of tail. Move tail one down (y-1)
        else if head.x == tail.x && head.y - tail.y == -2 {
            tail.y -= 1;
        } // Case 5: Head is one space right (x+1) and two spaces up (y+2).
          // Move tail diagonally up and right (y+1) (x+1).
          // .H
          // ..
          // T.
        else if head.x - tail.x == 1 && head.y - tail.y == 2 {
            tail.x += 1;
            tail.y += 1;
        } //Case 6: Head is two spaces right (x+2) and one space up (y+1).
          // Move tail diagonally up and right (y+1) (x+1)
          // ..H
          // T..
        else if head.x - tail.x == 2 && head.y - tail.y == 1 {
            tail.x += 1;
            tail.y += 1;
        } // Case 7: Head is one space left (x-1) and two up (y+2)
          // Move tail diagonally up and left (y+1) (x+1)
          // H.
          // ..
          // .T
        else if head.x - tail.x == -1 && head.y - tail.y == 2 {
            tail.x -= 1;
            tail.y += 1;
        } // Case 8: Head is two spaces left (x-2) and one up (y+1)
          // Move tail diagonally up and left (y+1) (x+1)
          // H..
          // ..T
        else if head.x - tail.x == -2 && head.y - tail.y == 1 {
            tail.x -= 1;
            tail.y += 1;
        } // Case 9: Head is one space left (x-1) and two spaces down (y-2)
          // Move tail diagonally down and left (y-1) (x-1)
          // .T
          // ..
          // H.
        else if head.x - tail.x == -1 && head.y - tail.y == -2 {
            tail.x -= 1;
            tail.y -= 1;
        } // Case 10: Head is two spaces left (x-2) and one space down (y-2)
          // Move tail diagonally down and left (y-1) (x-1)
        else if head.x - tail.x == -2 && head.y - tail.y == -1 {
            tail.x -= 1;
            tail.y -= 1;
        } // Case 11: Head is one space right (x+1) and two spaces down (y-2)
          // Move tail diagonally down and right (y-1), (x+1)
          // T.
          // ..
          // .H
        else if head.x - tail.x == 1 && head.y - tail.y == -2 {
            tail.x += 1;
            tail.y -= 1;
        } // Case 12: Head is two spaces right (x+2) and one space down (y-1)
          // Move tail diagonally down and right (y-1) (x+1)
          // T..
          // ..H
        else if head.x - tail.x == 2 && head.y - tail.y == -1 {
            tail.x += 1;
            tail.y -= 1;
        } // Case 13: No such case, freak out
        else {
            panic!("Unknown case for non-adjacent head and tail: {:?}", rope);
        }
    }

    rope.visited.insert((tail.x, tail.y));
    let t2 = touching(&rope.head, &rope.tail);
    println!("Touching? {}. {:?}, {:?}", t2, &rope.head, &rope.tail);
}

fn take_steps(steps: Vec<(char, i32)>, mut rope: Rope) -> Rope {
    for step in steps.iter() {
        let num_moves = step.1;
        match step.0 {
            'U' => {
                // Up means head.y increases
                for _ in 0..num_moves {
                    rope.head.y += 1;
                    manage_tail(&mut rope);
                }
            }
            'L' => {
                // Left means head.x decreases
                for _ in 0..num_moves {
                    rope.head.x -= 1;
                    manage_tail(&mut rope);
                }
            }
            'R' => {
                // Right means head.x increases
                for _ in 0..num_moves {
                    rope.head.x += 1;
                    manage_tail(&mut rope);
                }
            }
            'D' => {
                // Down means head.y decreases
                for _ in 0..num_moves {
                    rope.head.y -= 1;
                    manage_tail(&mut rope);
                }
            }
            _ => {
                println!("Unexpected direction: {}", step.0);
                break;
            }
        }
        println!("{:?}", rope);
    }

    rope
}

fn plot_grid(visited: &HashSet<(i32, i32)>) {
    let mut grid_width: i32 = 0;
    let mut grid_height: i32 = 0;

    for h in visited.iter() {
        let x = h.0;
        let y = h.1;
        grid_width = if x > grid_width {x} else {grid_width};
        grid_height = if y > grid_height {y} else {grid_height};
    }

    // Match the padding from the example
    grid_width += 2;
    grid_height += 1;

    println!("Grid Width: {}, Grid Height: {}", grid_width, grid_height);

    //TODO: create a blank grid which is Vec<Vec<char>>
    //   all rows will be "....." when rendered initially

    let mut grid: Vec<Vec<char>> = Vec::new();
    for _ in 0..grid_height {
        let mut row: Vec<char> = Vec::new();
        for _ in 0..grid_width {
            row.push('.');
        }
        grid.push(row);
    }

    for row in grid.into_iter().rev() {
        let s: String = row.into_iter().collect();
        println!("{}", s);
    }
}

fn main() {
    let puz: String = read_puzzle_input();
    // println!("{}", puz);

    let mut rope = Rope::new();
    // println!("{:?}", rope);

    let steps = parse_steps(&puz);
    // println!("{:?}", steps);

    let final_rope = take_steps(steps, rope);
    println!("Unique spots visited: {}", &final_rope.visited.len());
    plot_grid(&final_rope.visited);
}
