use crate::shared;

pub fn solve_day_1() {
    let inputs = shared::files::fetch_numbers_from_file("inputs/day-1.txt");

    let part_1 = solve_part_1(&inputs);
    let part_2 = solve_part_2(&inputs);

    shared::outputs::print_solutions(part_1, part_2);
}

fn solve_part_1(inputs: &Vec<i64>) -> i64 {
    for i in 0..inputs.len() {
        for j in (i + 1)..inputs.len() {
            if inputs[i] + inputs[j] == 2020 {
                return inputs[i] * inputs[j]
            }
        }
    }

    return 0
}

fn solve_part_2(inputs: &Vec<i64>) -> i64 {
    for i in 0..inputs.len() {
        for j in (i + 1)..inputs.len() {
            for k in (j + 1)..inputs.len() {
                if inputs[i] + inputs[j] + inputs[k] == 2020 {
                    return inputs[i] * inputs[j] * inputs[k]
                }
            }
        }
    }

    return 0
}

