use std::fs::read_to_string;
use std::io;

#[derive(Debug)]
enum MyError {
    Io(io::Error),
    Error,
}

fn is_valid(fields: &Vec<&str>) -> (bool, bool) {
    // let lax_valid = true;
    // let strict_valid = true;
    let req_fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    if req_fields.iter().all(|x| fields.contains(&x)) {
        println!("{:?}", fields);
        return (true, false)
    }
    (false, false)
}

fn main() -> Result<(), MyError> {
    let input_buffer = read_to_string("files/input.txt").map_err(MyError::Io)?;
    let passports: Vec<&str> = input_buffer.split("\n\n").collect();
    let mut valid1_count = 0;
    for passport in passports {
        let parts: Vec<&str> = passport.split_ascii_whitespace().collect();
        let fields: Vec<&str> = parts
            .iter()
            .map(|p| p.split(":").next())
            .collect::<Option<Vec<&str>>>().ok_or(MyError::Error)?;

        let valid_check = is_valid(&fields);
        if valid_check.0 {
            println!("{:?}", &fields);
            valid1_count += 1;
        }
    }
    println!("Result 1: {}", valid1_count);
    // println!("Result 2: {}", tree_map.values().fold(1 as u64, |prod, x| prod * x.trees));
    return Ok(());
}
