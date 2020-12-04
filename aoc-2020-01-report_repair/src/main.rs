use std::collections::HashMap;
use std::collections::HashSet;
// use std::error::Error;
use std::io;
use std::num;
use std::fs::read_to_string;

#[derive(Debug)]
enum MyError {
    Io(io::Error),
    Parse(num::ParseIntError),
}

fn main() -> Result<(), MyError> {
// fn main() -> Result<(), Box<dyn Error>> {
    let mut numbers = HashSet::new();
    let mut sums = HashMap::new();
    let mut found = 0;

    let input_buffer = read_to_string("files/input.txt").map_err(MyError::Io)?;
    for line in input_buffer.as_str().lines() {
        let number = line.parse::<i32>().map_err(MyError::Parse)?;
        let diff = 2020 - number;

        if numbers.contains(&diff) {
            println!("Result (2 values): {}", number * diff);
            found += 1;
        }

        if sums.contains_key(&diff) {
            println!(
                "Result (3 values): {}",
                number * sums[&diff] * (2020 - number - sums[&diff])
            );
            found += 1;
        } else {
            for &num in &numbers {
                sums.insert(num + number, num);
            }
        }

        numbers.insert(number);
        if found == 2 {
            return Ok(());
        }
    }
    return Ok(());
}
