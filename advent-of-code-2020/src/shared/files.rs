use std::{io::{self, BufRead}, fs::File};
use grid::*;
use itertools::Itertools;

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

pub fn fetch_grid_from_file(path: &str) -> Grid<char> {
    let mut grid = grid![];

    let strings = fetch_strings_from_file(path);

    for string in strings {
        grid.push_row(string.chars().collect_vec());
    }

    grid
}

pub fn fetch_char_vecs_from_file(path: &str) -> Vec<Vec<char>> {
    io::BufReader::new(File::open(path).unwrap())
        .lines()
        .map(|line| line.unwrap().chars().collect_vec())
        .collect()
}

pub fn fetch_char_groups_from_file(path: &str) -> Vec<Vec<Vec<char>>> {
    let lines = fetch_char_vecs_from_file(path);
    let mut current_group: Vec<Vec<char>> = Vec::new();
    let mut groups: Vec<Vec<Vec<char>>> = Vec::new();

    for line in lines {
        if line.len() == 0 {
            groups.push(current_group);
            current_group = Vec::new();
        } else {
            current_group.push(line);
        }
    }

    if current_group.len() > 0 {
        groups.push(current_group);
    }

    groups
}