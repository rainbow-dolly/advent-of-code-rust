use itertools::Itertools;

use crate::shared;

pub fn solve_day_2() {
    let inputs: Vec<PasswordToCheck> = shared::files::fetch_strings_from_file("inputs/day-2.txt").iter().map(parse_line).collect();

    let part_1 = inputs.iter().filter(|input| is_password_valid_1(*input)).count();
    let part_2 = inputs.iter().filter(|input| is_password_valid_2(*input)).count();

    shared::outputs::print_solutions(part_1, part_2)
}

struct PasswordToCheck {
    min: usize,
    max: usize,
    required_character: char,
    password: String
}

fn parse_line(line: &String) -> PasswordToCheck {
    let (rules, password) = line.split(": ").collect_tuple::<(&str, &str)>().unwrap();
    let (range, character) = rules.split(" ").collect_tuple::<(&str, &str)>().unwrap();
    let (min, max) = range.split("-").collect_tuple::<(&str, &str)>().unwrap();

    PasswordToCheck {
        min: min.parse().unwrap(),
        max: max.parse().unwrap(),
        required_character: character.chars().nth(0).unwrap(),
        password: password.to_string()
    }
}

fn is_password_valid_1(input: &PasswordToCheck) -> bool {
    let char_count = input.password.chars().filter(|c| *c == input.required_character).count();
    char_count >= input.min && char_count <= input.max
}

fn is_password_valid_2(input: &PasswordToCheck) -> bool {
    let password_characters: Vec<char> = input.password.chars().collect();
    let (char_1, char_2) = (password_characters[input.min - 1], password_characters[input.max - 1]);
    
    if char_1 == char_2 && char_2 == input.required_character {
        false
    } else if char_1 == input.required_character  {
        true
    } else if char_2 == input.required_character  {
        true
    } else {
        false
    }
}