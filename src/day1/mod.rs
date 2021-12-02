use std::fs;
use std::collections::VecDeque;

pub fn execute_puzzle() {
    let file_result = fs::read_to_string("src/day1/input.txt");

    let puzzle_input = match file_result {
        Ok(input) => input,
        Err(error) => panic!("Unable to read puzzle input: {}", error),
    };

    let lines = puzzle_input.lines();
    let lines_offset = puzzle_input.lines().skip(1);

    let number_increasing = lines.zip(lines_offset).fold(0, increasing_depth_str);

    // TODO: I'd like to do something with scan here instead
    let lines = puzzle_input.lines();
    let mut state: VecDeque<i32> = VecDeque::with_capacity(3);
    let mut depths: Vec<i32> = Vec::new();
    lines.for_each(|depth| {
        if state.len() == 3 {
            state.pop_front();
        }
        state.push_back(depth.parse().unwrap());

        if state.len() == 3 {
            depths.push(state.get(0).unwrap() + state.get(1).unwrap() + state.get(2).unwrap());
        }
    });

    let increasing_averages = depths.iter().zip(depths.iter().skip(1)).fold(0, increasing_depth_int);

    println!("Day 1 Puzzle -- Part One:{}, Part Two: {}", number_increasing, increasing_averages);
}

fn increasing_depth_str(number_increasing: i32, depths: (&str, &str)) -> i32 {
    increasing_depth_int(number_increasing, (&depths.0.parse::<i32>().unwrap(), &depths.1.parse::<i32>().unwrap()))
}

fn increasing_depth_int(number_increasing: i32, depths: (&i32, &i32)) -> i32 {
    if depths.0 < depths.1 {
        number_increasing + 1
    } else {
        number_increasing
    }
}
