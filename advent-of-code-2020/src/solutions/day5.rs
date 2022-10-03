use itertools::Itertools;

use crate::shared;

pub fn solve_day_5() {
    let inputs = shared::files::fetch_char_vecs_from_file("inputs/day-5.txt");

    let seat_ids = inputs.iter().map(calculate_seat_id).sorted().collect_vec();

    let part_1 = *seat_ids.last().unwrap();
    let mut part_2 = 0;
    
    for i in 0..(seat_ids.len()-1) {
        let id1 = seat_ids[i];
        let id2 = seat_ids[i+1];

        if id1 == id2 - 2 {
            part_2 = id1 + 1;
        }
    }

    shared::outputs::print_solutions(part_1, part_2);
}

fn calculate_seat_id(input: &Vec<char>) -> i64 {
    let mut row_lower_bound = 0;
    let mut row_upper_bound = 127;

    let mut col_lower_bound = 0;
    let mut col_upper_bound = 7;

    for character in input {
        match character {
            'F' => row_upper_bound -= (row_upper_bound - row_lower_bound) / 2 + 1,
            'B' => row_lower_bound += (row_upper_bound - row_lower_bound) / 2 + 1,
            'L' => col_upper_bound -= (col_upper_bound - col_lower_bound) / 2 + 1,
            'R' => col_lower_bound += (col_upper_bound - col_lower_bound) / 2 + 1,
            _ => ()
        }
    }

    row_lower_bound * 8 + col_lower_bound
}