use std::collections::HashMap;

const A_WIRE: u32 = 0b1000000;
const B_WIRE: u32 = 0b0100000;
const C_WIRE: u32 = 0b0010000;
const D_WIRE: u32 = 0b0001000;
const E_WIRE: u32 = 0b0000100;
const F_WIRE: u32 = 0b0000010;
const G_WIRE: u32 = 0b0000001;

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
struct Signal {
    binary_signal: u32
}

impl Signal {
    fn from_string(signal_string: &str) -> Signal {
        let binary_signal: u32 = signal_string.chars().map(|letter| {
            match letter {
                'a' => A_WIRE,
                'b' => B_WIRE,
                'c' => C_WIRE,
                'd' => D_WIRE,
                'e' => E_WIRE,
                'f' => F_WIRE,
                'g' => G_WIRE,
                _ => unreachable!()
            }
        }).fold(0b0, |binary_signal, input_mask| {
            binary_signal | input_mask
        });

        Signal { binary_signal }
    }

    fn segements(&self) -> u32 {
        [A_WIRE, B_WIRE, C_WIRE, D_WIRE, E_WIRE, F_WIRE, G_WIRE].into_iter().filter(|wire| {
            self.binary_signal & wire == *wire
        }).count() as u32
    }
 }

fn execute_puzzle_with(puzzle_input: &str) -> i32 {
    let mut total_sum = 0;
    for line in puzzle_input.lines() {
        let mut parts = line.split('|')
            .map(|part| {
                part.split_whitespace().map(|signal| {
                    Signal::from_string(signal)
                }).collect::<Vec<Signal>>()
            });
        let signal_patterns = parts.next().unwrap();
        let output_patterns = parts.next().unwrap();

        let one = signal_patterns.iter().find(|signal| signal.segements() == 2).unwrap();
        let four = signal_patterns.iter().find(|signal| signal.segements() == 4).unwrap();
        let seven = signal_patterns.iter().find(|signal| signal.segements() == 3).unwrap();
        let eight = signal_patterns.iter().find(|signal| signal.segements() == 7).unwrap();

        let seg_a = seven.binary_signal ^ one.binary_signal;
        let nine = signal_patterns.iter()
            .filter(|signal| { signal.segements() == 6 })
            .find(|signal| {
                let bits = signal.binary_signal ^ (seg_a | four.binary_signal);
                (bits & (bits-1)) == 0
            }).unwrap();
        let seg_g = nine.binary_signal ^ (four.binary_signal | seg_a);
        let seg_e = nine.binary_signal ^ eight.binary_signal;

        let c_and_d = signal_patterns.iter()
            .filter(|signal| { *signal != nine && signal.segements() == 6  })
            .fold(0, |merged, signal| { signal.binary_signal ^ merged });
        let seg_b = (c_and_d | one.binary_signal) ^ four.binary_signal;
        let seg_d = (one.binary_signal | seg_b) ^ four.binary_signal;
        let seg_c = c_and_d ^ seg_d;
        let seg_f = one.binary_signal ^ seg_c;

        let mut signal_to_number: HashMap<u32,i32> = HashMap::new();
        signal_to_number.insert(eight.binary_signal ^ seg_d, 0);
        signal_to_number.insert(one.binary_signal, 1);
        signal_to_number.insert(seg_a | seg_c | seg_d | seg_e | seg_g, 2);
        signal_to_number.insert(seg_a | seg_c | seg_d | seg_f | seg_g, 3);
        signal_to_number.insert(four.binary_signal, 4);
        signal_to_number.insert(nine.binary_signal ^ seg_c, 5);
        signal_to_number.insert(eight.binary_signal ^ seg_c, 6);
        signal_to_number.insert(seven.binary_signal, 7);
        signal_to_number.insert(eight.binary_signal, 8);
        signal_to_number.insert(nine.binary_signal, 9);

        let output_number: String = output_patterns.iter().map(|output_signal| {
            signal_to_number.get(&output_signal.binary_signal).unwrap()
        }).fold("".to_string(), |mut num_string, number| {
            num_string.push_str(number.to_string().as_str());
            num_string
        });

        total_sum += output_number.parse::<i32>().unwrap();
    }

    println!("Day 8 Puzzle -- {}", total_sum);
    total_sum
}

pub fn execute_puzzle() {
    execute_puzzle_with(include_str!("input.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle() {
        assert_eq!(execute_puzzle_with(include_str!("test.txt")), 61229);
    }
}
