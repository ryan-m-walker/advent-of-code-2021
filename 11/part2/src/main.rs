use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let mut octs = Octs::from_string(input);
    let count = octs.get_flashed();

    println!("Answer = {}", count);
}

#[derive(Debug, Copy, Clone)]
struct Oct {
    energy: u32,
    has_flashed: bool,
}

impl Oct {
    fn new(energy: u32, has_flashed: bool) -> Self {
        Self {
            energy,
            has_flashed,
        }
    }
}

#[derive(Debug, Clone)]
struct Octs {
    data: Vec<Oct>,
    flashed_count: u32,
    cols: i32,
    rows: i32,
}

impl Octs {
    fn from_string(input: String) -> Self {
        let lines: Vec<_> = input.trim().lines().collect();
        let mut data = vec![];

        let mut cols = 0;

        for i in 0..lines.len() {
            let line = lines[i];
            let chars: Vec<_> = line.split("").collect();
            for c in chars {
                if c == "" {
                    continue;
                }

                if i == 0 {
                    cols += 1;
                }

                data.push(Oct::new(c.parse().unwrap(), false));
            }
        }

        Self {
            data,
            flashed_count: 0,
            rows: lines.len() as i32,
            cols,
        }
    }

    fn get_index(&self, x: i32, y: i32) -> usize {
        (y * self.cols + x) as usize
    }

    fn get_flashed(&mut self) -> u32 {
        let mut count = 0;

        loop {
            count += 1;
            self.increment_all();
            self.handle_flashes();
            self.reset_has_flashed();
            if self.check_is_sync() {
                return count;
            }
        }
    }

    fn check_is_sync(&self) -> bool {
        let mut num: Option<u32> = None;

        for i in 0..self.data.len() {
            let oct = self.data[i];

            if num.is_none() {
                num = Some(oct.energy);
            } else {
                let num = num.unwrap();
                if num != oct.energy {
                    return false;
                }
            }
        }

        true
    }

    fn increment_all(&mut self) {
        for index in 0..self.data.len() {
            let oct = self.data[index];
            let new_energy = oct.energy + 1;
            self.data[index] = Oct::new(new_energy, false);
        }
    }

    fn handle_flashes(&mut self) {
        for index in 0..self.data.len() {
            let oct = self.data[index];

            if oct.energy > 9 && !oct.has_flashed {
                self.handle_flash(index);
            }
        }
    }

    fn handle_flash(&mut self, index: usize) {
        let oct = self.data[index];
        self.flashed_count += 1;
        self.data[index] = Oct::new(oct.energy, true);
        self.increment_all_neighbors(index);
    }

    fn increment(&mut self, index: usize) {
        let oct = self.data[index];
        let new_energy = oct.energy + 1;

        if new_energy > 10 {
            return;
        }

        self.data[index] = Oct::new(new_energy, false);

        if new_energy > 9 && !oct.has_flashed {
            self.handle_flash(index);
        }
    }

    fn increment_all_neighbors(&mut self, index: usize) {
        let (x, y) = self.index_to_coords(index as i32);

        let has_top = y - 1 >= 0;
        let has_right = x + 1 < self.cols;
        let has_bottom = y + 1 < self.rows;
        let has_left = x - 1 >= 0;

        // Top
        if has_top {
            self.increment(self.get_index(x, y - 1));
        }

        // Top Right
        if has_top && has_right {
            self.increment(self.get_index(x + 1, y - 1));
        }

        // Right
        if has_right {
            self.increment(self.get_index(x + 1, y));
        }

        // Bottom Right
        if has_right && has_bottom {
            self.increment(self.get_index(x + 1, y + 1));
        }

        // Bottom
        if has_bottom {
            self.increment(self.get_index(x, y + 1));
        }

        // Bottom Left
        if has_bottom && has_left {
            self.increment(self.get_index(x - 1, y + 1));
        }

        // Left
        if has_left {
            self.increment(self.get_index(x - 1, y));
        }

        // Top Left
        if has_top && has_left {
            self.increment(self.get_index(x - 1, y - 1));
        }
    }

    fn index_to_coords(&self, index: i32) -> (i32, i32) {
        let row = index / self.cols;
        let col = index % self.rows;

        (col, row)
    }

    fn reset_has_flashed(&mut self) {
        for i in 0..self.data.len() {
            let oct = self.data[i];
            if oct.energy > 9 {
                self.data[i] = Oct::new(0, false);
            }
        }
    }

    fn print(&self) {
        print!("\n");

        for i in 0..self.data.len() {
            if i != 0 && i % self.cols as usize == 0 {
                print!("\n");
            }

            let oct = self.data[i];
            print!("{}", oct.energy);
        }

        print!("\n\n");
    }
}
