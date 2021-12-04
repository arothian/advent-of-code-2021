use std::{fs};

struct PowerLevel {
    number_bits_on: Vec<i32>,
    reading_count: i32
}

impl PowerLevel {
    fn power_consumption(&self) -> u32 {
        self.gamma() * self.epsilon()
    }

    fn gamma(&self) -> u32 {
        self.number_bits_on.iter().enumerate().fold(0b0, |gamma, (bit, number_on)| {
            let mask = 0b01 << bit;
            let number_off = self.reading_count - number_on;

            if number_on > &number_off {
                gamma | mask
            } else {
                gamma
            }
        })
    }

    fn epsilon(&self) -> u32 {
        self.gamma() ^ 0b111111111111
    }

    fn process_readings(readings: Vec<u32>) -> PowerLevel {
        let mut power_level = PowerLevel { number_bits_on: vec![0; 12], reading_count: 0 };

        readings.iter().for_each(|reading_number| {
            let num_bits = power_level.number_bits_on.len();
            for bit in 0..num_bits {
                let mask = 0b01 << bit;
                let is_on = reading_number & mask == mask;
                if is_on {
                    power_level.number_bits_on[bit] += 1;
                }
            }
            power_level.reading_count +=1;
        });

        power_level
    }
}

struct LifeSupport {
    co2_scrubber_value: u32,
    o2_generator_value: u32
}

impl LifeSupport {
    fn new(diagnostic_readings: Vec<u32>) -> LifeSupport {
        let mask:u32 = 0b100000000000;
        LifeSupport {
            co2_scrubber_value: LifeSupport::find_value(diagnostic_readings.clone(), mask, false),
            o2_generator_value: LifeSupport::find_value(diagnostic_readings, mask, true)
        }
    }

    fn find_value(mut values: Vec<u32>, mut mask: u32, use_majority: bool) -> u32 {
        while values.len() > 1 {
            values = LifeSupport::reduce(values, mask, use_majority);
            mask >>= 1;
        }

        *values.first().unwrap()
    }

    fn reduce(values: Vec<u32>, mask: u32, use_majority: bool) -> Vec<u32> {
        let split:(Vec<u32>, Vec<u32>) = values.iter().partition(|reading| { *reading & mask == mask });

        if use_majority {
            if split.0.len() >= split.1.len() {
                split.0
            } else {
                split.1
            }
        } else if split.0.len() >= split.1.len() {
            split.1
        } else {
            split.0
        }
    }

    fn life_support_rating(&self) -> u32 {
        self.o2_generator_value * self.co2_scrubber_value
    }
}

pub fn execute_puzzle() {
    let file_result = fs::read_to_string("src/day3/input.txt");

    let puzzle_input = match file_result {
        Ok(input) => input,
        Err(error) => panic!("Unable to read puzzle input: {}", error),
    };

    let lines = puzzle_input.lines();
    let readings:Vec<u32> = lines.map(|reading| {
        u32::from_str_radix(reading, 2).unwrap()
    }).collect();

    let power_level = PowerLevel::process_readings(readings.clone());
    let life_support = LifeSupport::new(readings.clone());

    println!("Day 3 Puzzle -- power: {}, life support: {}", power_level.power_consumption(), life_support.life_support_rating());
}
