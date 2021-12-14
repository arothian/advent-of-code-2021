#[derive(Clone, Hash, PartialEq, Eq)]
struct CrabSubs {
    subs: Vec<i32>
}

impl CrabSubs {
    fn add_sub_from_string(&mut self, string: &str) {
        self.subs.push(string.parse::<i32>().unwrap());
    }

    fn align(&mut self) -> i32 {
        let average_position: f32 = self.subs.iter().sum::<i32>() as f32 / self.subs.len() as f32;

        let power_floor = self.power_consuption(average_position.floor() as i32);
        let power_ceiling = self.power_consuption(average_position.ceil() as i32);

        power_ceiling.min(power_floor)
    }

    fn power_consuption(&self, target_position: i32) -> i32 {
        self.subs.iter().fold(0, |power_consuption, sub| {
            let n = (sub - target_position).abs();
            power_consuption + ((n*(n+1))/2)
        })
    }
}

fn execute_puzzle_with(puzzle_input: &str) -> i32 {
    // let puzzle_input = include_str!(input_file);

    let mut subs: CrabSubs = puzzle_input.split(',')
        .fold(CrabSubs { subs: vec![] },|mut subs, number_str | {
            subs.add_sub_from_string(number_str);
            subs
        });

    let power_consumed = subs.align();

    println!("Day 7 Puzzle -- {}", power_consumed);
    power_consumed
}

pub fn execute_puzzle() {
    execute_puzzle_with(include_str!("input.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle() {
        assert_eq!(execute_puzzle_with(include_str!("test.txt")), 168);
    }
}
