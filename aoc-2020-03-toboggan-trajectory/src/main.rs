use std::collections::HashMap;
use std::fs::read_to_string;
use std::io;

#[derive(Debug)]
enum MyError {
    Io(io::Error),
    Error,
}

#[derive(Debug)]
struct State {
    trees: u64,
    column: usize
}

#[derive(Hash, PartialEq, Eq, Debug)]
struct Path {
    down: usize,
    right: usize
}

// impl std::hash::Hash for Path { };

fn main() -> Result<(), MyError> {
    let input_buffer = read_to_string("files/input.txt").map_err(MyError::Io)?;

    let matrix: Vec<Vec<char>> = input_buffer.lines().map(|l| l.chars().collect()).collect();
    let row_len = matrix.first().ok_or(MyError::Error)?.len();

    // let mut state = State{trees: 0, column: 0};
    let mut tree_map = HashMap::new();
    tree_map.insert(Path{down: 1, right: 1}, State{trees: 0, column: 0});
    tree_map.insert(Path{down: 1, right: 3}, State{trees: 0, column: 0});
    tree_map.insert(Path{down: 1, right: 5}, State{trees: 0, column: 0});
    tree_map.insert(Path{down: 1, right: 7}, State{trees: 0, column: 0});
    tree_map.insert(Path{down: 2, right: 1}, State{trees: 0, column: 0});

    for (i, row) in matrix.iter().enumerate() {
        for (path, state) in tree_map.iter_mut() {
            if i % path.down == 0 {
                if row[state.column] == '#' {
                    state.trees += 1;
                }
                state.column = (state.column + path.right) % row_len;
            }
        }
    }
    println!("Result 1: {}", tree_map[&Path{down: 1, right: 3}].trees);
    println!("Result 2: {}", tree_map.values().fold(1 as u64, |prod, x| prod * x.trees));
    return Ok(());
}
