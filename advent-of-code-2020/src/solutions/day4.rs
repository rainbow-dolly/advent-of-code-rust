use std::collections::HashMap;

use itertools::Itertools;
use regex::Regex;

use crate::shared::{files::fetch_strings_from_file, self};

pub fn solve_day_4() {
    let inputs = fetch_strings_from_file("inputs/day-4.txt");
    let passports = parse_passports(&inputs);

    let mut valid_passports_1 = 0;
    let mut valid_passports_2 = 0;

    for passport in passports {
        if is_passport_valid_1(&passport) {
            valid_passports_1 += 1;

            if is_passport_valid_2(&passport) {
                valid_passports_2 += 1;
            }
        }
    }

    shared::outputs::print_solutions(valid_passports_1, valid_passports_2);
}

fn parse_passports(inputs: &Vec<String>) -> Vec<HashMap<String, String>> {
    let mut passports: Vec<HashMap<String, String>> = Vec::new();
    let mut current_passport: HashMap<String, String> = HashMap::new();

    for line in inputs {
        if line.is_empty() {
            passports.push(current_passport);
            current_passport = HashMap::new();
        } else {
            for part in line.split(" ") {
                let (key, value) = part.split(":").collect_tuple().unwrap();
                current_passport.insert(key.to_string(), value.to_string());
            }
        }
    }

    passports.push(current_passport);
    passports
}

fn is_passport_valid_1(passport: &HashMap<String, String>) -> bool {
    passport.contains_key("byr") &&
    passport.contains_key("iyr") &&
    passport.contains_key("eyr") &&
    passport.contains_key("hgt") &&
    passport.contains_key("hcl") &&
    passport.contains_key("ecl") &&
    passport.contains_key("pid")
}

fn is_passport_valid_2(passport: &HashMap<String, String>) -> bool {
    byr_valid(passport.get("byr").unwrap()) &&
    iyr_valid(passport.get("iyr").unwrap()) &&
    eyr_valid(passport.get("eyr").unwrap()) &&
    hgt_valid(passport.get("hgt").unwrap()) &&
    hcl_valid(passport.get("hcl").unwrap()) &&
    ecl_valid(passport.get("ecl").unwrap()) &&
    pid_valid(passport.get("pid").unwrap())
}

fn byr_valid(byr: &String) -> bool {
    match byr.parse::<i64>() {
        Ok(numeric) => numeric >= 1920 && numeric <= 2002,
        Err(_) => false,
    }
}

fn iyr_valid(iyr: &String) -> bool {
    match iyr.parse::<i64>() {
        Ok(numeric) => numeric >= 2010 && numeric <= 2020,
        Err(_) => false,
    }
}

fn eyr_valid(eyr: &String) -> bool {
    match eyr.parse::<i64>() {
        Ok(numeric) => numeric >= 2020 && numeric <= 2030,
        Err(_) => false,
    }
}

fn hgt_valid(hgt: &String) -> bool {
    if hgt.ends_with("cm") {
        let numeric = hgt.replace("cm", "").parse::<i64>();
        match numeric {
            Ok(numeric) => numeric >= 150 && numeric <= 193,
            Err(_) => false
        }
    } else if hgt.ends_with("in") {
        let numeric = hgt.replace("in", "").parse::<i64>();
        match numeric {
            Ok(numeric) => numeric >= 59 && numeric <= 76,
            Err(_) => false
        }
    } else {
        false
    }
}

fn hcl_valid(hcl: &String) -> bool {
    Regex::new("^#[0-9a-f]{6}$").unwrap().is_match(hcl)
}

fn ecl_valid(ecl: &String) -> bool {
    ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&ecl.as_str())
}

fn pid_valid(pid: &String) -> bool {
    Regex::new("^[0-9]{9}$").unwrap().is_match(pid)
}