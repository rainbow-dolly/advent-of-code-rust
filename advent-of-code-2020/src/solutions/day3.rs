use grid::Grid;

use crate::shared::{files::fetch_grid_from_file, outputs::print_solutions};

pub fn solve_day_3() {
    let input_grid = fetch_grid_from_file("inputs/day-3.txt");

    let t_1_1 = calculate_slope(&input_grid, 1, 1);
    let t_3_1 = calculate_slope(&input_grid, 3, 1);
    let t_5_1 = calculate_slope(&input_grid, 5, 1);
    let t_7_1 = calculate_slope(&input_grid, 7, 1);
    let t_1_2 = calculate_slope(&input_grid, 1, 2);

    print_solutions(t_3_1, t_1_1 * t_3_1 * t_5_1 * t_7_1 * t_1_2);
}

fn calculate_slope(input_grid: &Grid<char>, right: usize, down: usize) -> i64 {
    let (rows, cols) = input_grid.size();
    let mut x = down;
    let mut y = 0;
    let mut trees = 0;

    while x < rows {
        y += right;

        if y >= cols {
            y -= cols;
        }

        if *input_grid.get(x, y).unwrap() == '#' {
            trees += 1;
        }

        x += down;
    }
    
    trees
}