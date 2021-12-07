use std::{fs};

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
struct Lanternfish {
    colony: [u64; 9]
}

impl Lanternfish {
    fn add_fish_from_string(&mut self, string: &str) {
        self.colony[string.parse::<usize>().unwrap()] += 1;
    }

    fn tock(&mut self) {
        // Get num new fish to be made
        let new_gen_size = self.colony[0];

        // Rotate the normal lifecycle 0..7 left once
        self.colony[0..7].rotate_left(1);

        // Move down the new spawn counts
        self.colony[6] += self.colony[7];
        self.colony[7] = self.colony[8];
        self.colony[8] = new_gen_size;
    }
}

pub fn execute_puzzle() {
    let file_result = fs::read_to_string("src/day6/input.txt");

    let puzzle_input = match file_result {
        Ok(input) => input,
        Err(error) => panic!("Unable to read puzzle input: {}", error),
    };

    let mut colony: Lanternfish = puzzle_input.split(',')
        .fold(Lanternfish { colony: [0; 9] },|mut colony, number_str | {
            colony.add_fish_from_string(number_str);
            colony
        });

    for _day in 0..256 {
        colony.tock();
    }

    println!("Day 6 Puzzle -- {}", colony.colony.iter().sum::<u64>());
}
