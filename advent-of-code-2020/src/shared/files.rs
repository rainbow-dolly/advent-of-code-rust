use std::{io::{self, BufRead}, fs::File};

pub fn fetch_numbers_from_file(path: &str) -> Vec<i64> {
    io::BufReader::new(File::open(path).unwrap())
        .lines()
        .map(|line| line.unwrap())
        .map(|line| line.parse::<i64>().unwrap())
        .collect()
}

pub fn fetch_strings_from_file(path: &str) -> Vec<String> {
    io::BufReader::new(File::open(path).unwrap())
        .lines()
        .map(|line| line.unwrap())
        .collect()
}