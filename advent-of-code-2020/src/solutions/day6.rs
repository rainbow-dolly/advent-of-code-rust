use std::collections::HashSet;

use itertools::Itertools;

use crate::shared;

pub fn solve_day_6() {
    let inputs = shared::files::fetch_char_groups_from_file("inputs/day-6.txt");

    let part_1: usize = inputs.iter().map(|input| input.iter().flatten().unique().count()).sum();
    
    let part_2 = inputs.iter().map(|input| {
        let mut final_set: HashSet<char> = HashSet::from_iter(input[0].clone());

        for answers in input.iter().skip(1) {
            final_set = final_set.intersection(&HashSet::from_iter(answers.clone())).map(|character| *character).collect::<HashSet<char>>();
        }

        final_set.len()
    }).sum();

    shared::outputs::print_solutions(part_1, part_2);
}