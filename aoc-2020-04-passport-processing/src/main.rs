use std::collections::HashMap;
use std::fs::read_to_string;
use std::io;

#[derive(Debug)]
enum MyError {
    Io(io::Error),
}

struct Limits {
    start: i32,
    end: i32
}

fn check_range(data: &str, limits: Limits) -> bool {
    let val = match data.parse::<i32>() {
        Ok(x) => x,
        Err(_) => -1
    };
    limits.start <= val && val <= limits.end
}

fn check_height(data: &str) -> bool {
    let mut limits = Limits{start:2, end: 1};
    if data.ends_with("in") {
        limits = Limits{start: 59, end: 76};
    } else if data.ends_with("cm") {
        limits = Limits{start: 150, end: 193};
    }
    check_range(&data[0..(data.len()-2)], limits)
}

fn check_hair_color(data: &str) -> bool {
    data.starts_with('#') && data[1..].chars().all(|c| c.is_ascii_hexdigit())
}

fn check_eye_color(data: &str) -> bool {
    let eye_colors = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    eye_colors.contains(&data)
}

fn check_passport_id(data: &str) -> bool {
    let is_int = match data.parse::<i32>() {
        Ok(_) => 1,
        Err(_) => 0
    } == 1;
    is_int && data.len() == 9
}

fn is_valid(parts: &Vec<&str>) -> Result<(bool, bool), MyError> {
    let keys_valid: bool;
    let mut values_valid = true;
    let req_fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    let mut validations: HashMap<&str, fn(&str)-> bool> = HashMap::new();
    validations.insert("byr", |val| check_range(val, Limits{start: 1920, end: 2002}));
    validations.insert("iyr", |val| check_range(val, Limits{start: 2010, end: 2020}));
    validations.insert("eyr", |val| check_range(val, Limits{start: 2020, end: 2030}));
    validations.insert("hgt", check_height);
    validations.insert("hcl", check_hair_color);
    validations.insert("ecl", check_eye_color);
    validations.insert("pid", check_passport_id);

    let mut fields = Vec::new();
    for part in parts {
        let key_val: Vec<_> = part.split(':').collect();
        let key = key_val[0];
        fields.push(key);
        let val = key_val[1];
        if validations.contains_key(&key) && !validations[key](val) {
            // println!("Validation failed: {} -> {}", key, val);
            values_valid = false;
        }
    }
    keys_valid = req_fields.iter().all(|x| fields.contains(&x));

    Ok((keys_valid, keys_valid && values_valid))
}

fn main() -> Result<(), MyError> {
    let input_buffer = read_to_string("files/input.txt").map_err(MyError::Io)?;
    let passports: Vec<&str> = input_buffer.split("\n\n").collect();
    let mut valid1_count = 0;
    let mut valid2_count = 0;

    for passport in passports {
        let parts: Vec<&str> = passport.split_ascii_whitespace().collect();
        // let fields = parts.iter().map(|p| p.split(":")).collect();
        // println!("{:?}", &parts);

        let valid_check = is_valid(&parts)?;
        if valid_check.0 {
            // println!("{:?}", &parts);
            valid1_count += 1;
        }
        if valid_check.1 {
            valid2_count += 1;
        }
    }
    println!("Result 1: {}", valid1_count);
    println!("Result 2: {}", valid2_count);
    // println!("Result 2: {}", tree_map.values().fold(1 as u64, |prod, x| prod * x.trees));
    return Ok(());
}
