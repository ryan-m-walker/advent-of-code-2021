use std::fs;
use std::mem;

fn main() {
    let numbers = fs::read_to_string("numbers.txt").unwrap();
    let numbers: Vec<_> = numbers
        .split(",")
        .map(|n| n.parse::<u32>().unwrap())
        .collect();

    let boards = fs::read_to_string("boards.txt").unwrap();
    let boards: Vec<_> = boards.split("\n\n").collect();
    let mut boards: Vec<_> = boards.iter().map(|b| Board::from_str(b)).collect();

    let mut winner_count = 0;

    'outer: for number in numbers {
        for i in 0..boards.len() {
            let mut board = mem::replace(&mut boards[i], Board::empty());
            board.mark_space(number);
            if board.check_for_win() {
                if board.is_winner != true {
                    winner_count += 1;
                    board.is_winner = true;

                    if winner_count == boards.len() {
                        let sum = board.get_sum_of_unmarked();
                        println!("Answer = {}", sum * number);
                        break 'outer;
                    }
                }
            }

            boards[i] = board;
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Cell {
    Space(u32),
    Hit,
}

#[derive(Debug, Clone)]
struct Board {
    data: Vec<Cell>,
    cols: u32,
    is_winner: bool,
}

impl Board {
    fn from_str(input: &str) -> Self {
        let cells: Vec<_> = input.split_whitespace().collect();

        let data: Vec<_> = cells
            .iter()
            .map(|c| Cell::Space(c.parse::<u32>().unwrap()))
            .collect();

        Self {
            cols: 5,
            data,
            is_winner: false,
        }
    }

    fn empty() -> Self {
        let mut data = vec![];

        for _ in 0..25 {
            data.push(Cell::Space(0));
        }

        Self {
            cols: 5,
            data,
            is_winner: false,
        }
    }

    fn print(&self) {
        print!("\n");

        for i in 0..self.data.len() {
            if i % self.cols as usize == 0 && i != 0 {
                print!("\n");
            }

            let cell = self.data.get(i).unwrap();

            match cell {
                Cell::Space(n) => {
                    if *n < 10 {
                        print!(" ");
                    }

                    print!("{}", n);
                }
                Cell::Hit => print!(" X"),
            }

            print!(" ");
        }

        print!("\n\n");
    }

    fn mark_space(&mut self, number: u32) {
        for i in 0..self.data.len() {
            let space = self.data.get(i).unwrap();
            if let Cell::Space(n) = space {
                if *n == number {
                    self.data[i] = Cell::Hit;
                }
            };
        }
    }

    fn check_for_win(&mut self) -> bool {
        // check horizontal
        for row in 0..self.cols {
            let mut hits = 0;

            for col in 0..self.cols {
                let index = self.get_index(col, row);
                let cell = self.data.get(index).unwrap();

                if let Cell::Hit = cell {
                    hits += 1;
                }
            }

            if hits == 5 {
                return true;
            }
        }

        // check vertical
        for col in 0..self.cols {
            let mut hits = 0;

            for row in 0..self.cols {
                let index = self.get_index(col, row);
                let cell = self.data.get(index).unwrap();

                if let Cell::Hit = cell {
                    hits += 1;
                }
            }

            if hits == 5 {
                return true;
            }
        }

        false
    }

    fn get_index(&self, x: u32, y: u32) -> usize {
        (y * self.cols + x) as usize
    }

    fn get_sum_of_unmarked(&self) -> u32 {
        let mut sum = 0;

        for i in 0..self.data.len() {
            let cell = self.data.get(i).unwrap();

            if let Cell::Space(n) = cell {
                sum += n;
            }
        }

        sum
    }
}
