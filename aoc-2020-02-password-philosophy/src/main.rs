use std::fs::read_to_string;
use std::io;
use std::num;

#[derive(Debug)]
enum MyError {
    Io(io::Error),
    Parse(num::ParseIntError),
    Error,
}

fn main() -> Result<(), MyError> {
    let input_buffer = read_to_string("files/input.txt").map_err(MyError::Io)?;
    let mut cnt1 = 0;
    let mut cnt2 = 0;
    for line in input_buffer.as_str().lines() {
        let mut parts = line.split_ascii_whitespace();
        let mut limits = parts.next().ok_or(MyError::Error)?.split('-');
        let num1 = limits
            .next()
            .ok_or(MyError::Error)?
            .parse::<i32>()
            .map_err(MyError::Parse)?;
        let num2 = limits
            .next()
            .ok_or(MyError::Error)?
            .parse::<i32>()
            .map_err(MyError::Parse)?;
        let letter = parts
            .next()
            .ok_or(MyError::Error)?
            .chars()
            .nth(0)
            .ok_or(MyError::Error)?;
        let password = parts.next().ok_or(MyError::Error)?;

        let matches = password.matches(letter).count() as i32;
        if (num1 <= matches) && (matches <= num2) {
            cnt1 += 1;
        }

        let mut pw_chars = password.chars();
        let char1 = pw_chars
            .nth((num1 - 1) as usize)
            .ok_or(MyError::Error)?;
        let char2 = pw_chars
            .nth((num2 - num1 - 1) as usize)
            .ok_or(MyError::Error)?;
        if (letter == char1 || letter == char2) && char1 != char2 {
            cnt2 += 1;
        }
    }

    println!("Result 1: {}", cnt1);
    println!("Result 2: {}", cnt2);
    return Ok(());
}
