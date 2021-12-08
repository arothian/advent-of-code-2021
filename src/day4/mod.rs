struct BingoBoard {
    numbers: [i32; 25],
    marked_numbers: u32 // 25 bits,  0b00000000000000000000 for each number position
}

impl BingoBoard {
    fn mark_number(&mut self, number: i32) {
        if let Some(number_index) = self.numbers.iter().position(|board_number| *board_number == number) {
            self.mark_number_for_index(number_index.try_into().unwrap());
        }
    }

    fn is_marked(&self, index: usize) -> bool {
        let row = index / 5;
        let col = index % 5;
        let mask = 0b1 << ((row * 5) + col);
        self.marked_numbers & mask == mask
    }

    fn mark_number_for_index(&mut self, index: i32) {
        self.mark(index / 5, index % 5);
    }

    fn mark(&mut self, row:i32, col:i32) {
        let mask = 0b1 << ((row * 5) + col);
        self.marked_numbers |= mask;
    }

    fn check_row(&self, row:i32) -> bool {
        let mask = 0b11111 << (row * 5);
        self.marked_numbers & mask == mask
    }

    fn check_col(&self, col:i32) -> bool {
        let mask = 0b100001000010000100001 << col;
        self.marked_numbers & mask == mask
    }

    fn is_bingo(&self) -> bool {
        for val in 0..5 {
            if self.check_row(val) || self.check_col(val) {
                return true;
            }
        }
        false
    }

    fn score_board(&self, last_called_number:i32) -> i32 {
        let mut unmarked_sum = 0;
        for index in 0..self.numbers.len() {
            if !self.is_marked(index) {
                unmarked_sum += self.numbers[index];
            }
        }

        unmarked_sum * last_called_number
    }
}

pub fn execute_puzzle() {
    let puzzle_input = include_str!("input.txt");
    let mut lines = puzzle_input.lines();

    let mut boards: Vec<BingoBoard> = Vec::new();
    let numbers:Vec<i32> = lines.next().unwrap().split(',').map(|number| { number.parse::<i32>().unwrap() }).collect::<Vec<i32>>();

    let mut board_numbers:Vec<i32> = Vec::new();
    for line in lines {
        if line.is_empty() || line.contains(',') {
            continue;
        } else {
            let mut items = line.split_whitespace().map(|string| string.parse::<i32>().unwrap()).collect::<Vec<i32>>();
            board_numbers.append(&mut items);
            if board_numbers.len() == 25 {
                boards.push(BingoBoard {numbers: board_numbers[..].try_into().unwrap(), marked_numbers: 0 });
                board_numbers = Vec::new();
            }
        }
    }

    for number in numbers {
        boards.retain(|board| { !board.is_bingo() });
        for board in &mut boards {
            board.mark_number(number);
            // if board.is_bingo(){
            //     let winning_board_score = board.score_board(number);
            //     println!("Day 4 Puzzle -- {} ", winning_board_score);
            //     return;
            // }
        }
        if boards.len() == 1 && boards.get(0).unwrap().is_bingo() {
            let winning_board = boards.get(0).unwrap();
            let winning_board_score = winning_board.score_board(number);
            println!("Day 4 Puzzle -- {} ", winning_board_score);
            return;
        }
    }
}
