use std::collections::HashMap;
use std::fs::read_to_string;
use std::io;

#[derive(Debug)]
enum MyError {
    Io(io::Error),
    Error,
}

fn main() -> Result<(), MyError> {
    // let trees: Vec<Vec<char>>;
    let input_buffer = read_to_string("files/input.txt").map_err(MyError::Io)?;
    // for line in input_buffer.as_str().lines() {
    //     let mut foo = line.chars().collect::<Vec<char>>();
    //     trees.append(&foo);
    // }
    let matrix: Vec<Vec<char>> = input_buffer.lines().map(|l| l.chars().collect()).collect();
    let mut cur_col = 0;
    // let mut tree_map: HashMap<(i32, i32), (i32, i32)> = ((1, 1), (3, 1), (5, 1), (7, 1), (1,2))
    //     .map(|x, y| (0, 0))
    //     .collect();

    let row_len = matrix.first().ok_or(MyError::Error)?.len();

    let mut tree_count = 0;
    for row in matrix {
        if row[cur_col] == '#' {
            tree_count += 1;
        }
        cur_col = (cur_col + 3) % row_len;
    }
    println!("Result 1: {}", tree_count);
    // println!("Result 2: {}", cnt2);
    return Ok(());
}
