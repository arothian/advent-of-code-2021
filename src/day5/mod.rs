use std::{fs, collections::HashMap, fmt, cmp};

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
struct Coordinate {
    x: i32,
    y: i32
}

impl Coordinate {
    fn from_string(string: &str) -> Coordinate {
        let parts:Vec<&str> = string.split(',').collect();
        Coordinate {
            x: parts.get(0).unwrap().parse::<i32>().unwrap(),
            y: parts.get(1).unwrap().parse::<i32>().unwrap()
        }
    }

    fn is_horizontal(a: Coordinate, b: Coordinate) -> bool {
        a.y == b.y
    }

    fn is_vertical(a: Coordinate, b: Coordinate) -> bool {
        a.x == b.x
    }

    fn generate_horizontal_line(a: Coordinate, b: Coordinate) -> Vec<Coordinate> {
        let mut line = Vec::new();
        for number in cmp::min(a.x,b.x)..=cmp::max(a.x, b.x) {
            line.push(Coordinate { x: number, y: a.y })
        }

        line
    }

    fn generate_vertical_line(a: Coordinate, b: Coordinate) -> Vec<Coordinate> {
        let mut line = Vec::new();
        for number in cmp::min(a.y,b.y)..=cmp::max(a.y, b.y) {
            line.push(Coordinate { y: number, x: a.x })
        }

        line
    }

    fn generate_diagonal_line(a: Coordinate, b: Coordinate) -> Vec<Coordinate> {
        let mut line = Vec::new();

        let x_mod = if a.x < b.x {
            1
        } else {
            -1
        };
        let y_mod = if a.y < b.y {
            1
        } else {
            -1
        };

        for step in 0..=(a.x - b.x).abs() {
            line.push(Coordinate { x: a.x+(step*x_mod), y: a.y+(step*y_mod) })
        }

        line
    }
}

impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

pub fn execute_puzzle() {
    let file_result = fs::read_to_string("src/day5/input.txt");

    let puzzle_input = match file_result {
        Ok(input) => input,
        Err(error) => panic!("Unable to read puzzle input: {}", error),
    };

    let lines = puzzle_input.lines();

    let mut map: HashMap<Coordinate, i32> = HashMap::new();

    for line in lines {
        let parts: Vec<Coordinate> = line.split(" -> ").map(|str_coord| { Coordinate::from_string(str_coord) }).collect();

        let a = *parts.get(0).unwrap();
        let b = *parts.get(1).unwrap();

        if Coordinate::is_horizontal(a, b) {
            for cord in Coordinate::generate_horizontal_line(a, b) {
                map.insert(cord, map.get(&cord).unwrap_or(&0) + 1);
            }
        } else if Coordinate::is_vertical(a, b) {
            for cord in Coordinate::generate_vertical_line(a, b) {
                map.insert(cord, map.get(&cord).unwrap_or(&0) + 1);
            }
        } else {
            for cord in Coordinate::generate_diagonal_line(a, b) {
                map.insert(cord, map.get(&cord).unwrap_or(&0) + 1);
            }
        }
    }
    let count = map.values().fold(0, |count, coord_count| {
        if *coord_count >= 2 {
            count + 1
        } else {
            count
        }
    });

    println!("Day 5 Puzzle -- {}", count);
}
