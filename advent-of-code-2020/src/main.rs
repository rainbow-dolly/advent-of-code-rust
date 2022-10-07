use std::env;

mod shared;
mod solutions;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please specify the day to solve")
    } else {
        match args[1].as_str() {
            "1" => solutions::day1::solve_day_1(),
            "2" => solutions::day2::solve_day_2(),
            "3" => solutions::day3::solve_day_3(),
            "4" => solutions::day4::solve_day_4(),
            "5" => solutions::day5::solve_day_5(),
            "6" => solutions::day6::solve_day_6(),
            "7" => solutions::day7::solve_day_7(),
            _ => println!("Invalid arguments: {}", args[0])
        }
    }
}