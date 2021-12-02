use std::fs;


struct Submarine {
    horizontal: i32,
    depth: i32,
    aim: i32
}

impl Submarine {
    fn forward(&mut self, change: i32) {
        self.horizontal += change;
        self.depth += self.aim * change;
    }

    fn down(&mut self, change: i32) {
        self.aim += change;
    }

    fn up(&mut self, change: i32) {
        self.aim -= change;
    }

    fn command(&mut self, command: &str, value: i32) {
        match command {
            "down" => self.down(value),
            "up" => self.up(value),
            "forward" => self.forward(value),
            _ => println!("Unknown command: {}", command)
        }
    }
}

pub fn execute_puzzle() {
    let file_result = fs::read_to_string("src/day2/input.txt");

    let puzzle_input = match file_result {
        Ok(input) => input,
        Err(error) => panic!("Unable to read puzzle input: {}", error),
    };

    let lines = puzzle_input.lines();

    let submarine = lines.fold(Submarine { horizontal: 0, depth: 0, aim: 0 }, |mut sub, line| {
        let mut instruction = line.split(' ');
        sub.command(instruction.next().unwrap(), instruction.next().unwrap().parse::<i32>().unwrap());
        sub
    });


    println!("Day 2 Puzzle -- {}", submarine.horizontal * submarine.depth);
}
