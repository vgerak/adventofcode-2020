use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = File::open("files/input.txt")?;
    let mut input_buffer = String::new();
    let mut numbers = HashSet::new();

    input.read_to_string(&mut input_buffer)?;
    for line in input_buffer.as_str().lines() {
        let number =  line.parse::<i32>()?;
        if numbers.contains(&(2020 - number)) {
            println!("Result: {}", number * (2020-number));
            return Ok(());
        }
        numbers.insert(number);
    }
    return Ok(());
}
