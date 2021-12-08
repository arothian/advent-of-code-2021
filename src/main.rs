use took::took_print;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

fn main() {
    println!("Hello, Advent of Code 2021!");

    for day in 1..=8 {
       let puzzle_fn = match day {
            1 => day1::execute_puzzle,
            2 => day2::execute_puzzle,
            3 => day3::execute_puzzle,
            4 => day4::execute_puzzle,
            5 => day5::execute_puzzle,
            6 => day6::execute_puzzle,
            7 => day7::execute_puzzle,
            8 => day8::execute_puzzle,
            _ => unreachable!()
        };
        let _ = took_print(format!("Day {}", day).as_str(), puzzle_fn);
    }
}
