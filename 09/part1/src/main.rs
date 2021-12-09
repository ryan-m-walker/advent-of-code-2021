use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let map = Map::from_string(input);
    let risk_level = map.get_risk_level();

    println!("Answer = {:?}", risk_level);
}

struct Map {
    data: Vec<u32>,
    cols: i32,
    rows: i32,
}

impl Map {
    fn from_string(input: String) -> Self {
        let lines: Vec<_> = input.split("\n").collect();
        let cols = lines.get(0).unwrap().len() as i32;
        let rows = lines.len() as i32;

        let mut data = vec![];

        for line in lines {
            let digits: Vec<_> = line.split("").collect();

            for digit in digits {
                if digit == "" {
                    continue;
                }

                let parsed = digit.parse::<u32>().unwrap();
                data.push(parsed);
            }
        }

        Self { data, cols, rows }
    }

    fn get_risk_level(&self) -> u32 {
        let low_points = self.find_low_points();

        let mut risk_level = 0;

        for low_point in low_points {
            let level = 1 + low_point;
            risk_level += level;
        }

        risk_level
    }

    fn find_low_points(&self) -> Vec<&u32> {
        let mut low_points = vec![];

        for row in 0..self.rows {
            for col in 0..self.cols {
                if self.is_low_point(col, row) {
                    let point = self.get(col, row).unwrap();
                    println!("point = {}, ({}, {})", point, col, row);
                    low_points.push(point);
                }
            }
        }

        low_points
    }

    fn is_low_point(&self, x: i32, y: i32) -> bool {
        let point = self.get(x, y).unwrap();

        let top = self.get(x, y - 1);
        let right = self.get(x + 1, y);
        let bottom = self.get(x, y + 1);
        let left = self.get(x - 1, y);

        if top.is_some() && top.unwrap() <= point {
            return false;
        }

        if x + 1 < self.cols {
            if right.is_some() && right.unwrap() <= point {
                return false;
            }
        }

        if bottom.is_some() && bottom.unwrap() <= point {
            return false;
        }

        if x > 0 {
            if left.is_some() && left.unwrap() <= point {
                return false;
            }
        }

        true
    }

    fn get_index(&self, x: i32, y: i32) -> usize {
        (y * self.cols + x) as usize
    }

    fn get(&self, x: i32, y: i32) -> Option<&u32> {
        let index = self.get_index(x, y);
        self.data.get(index)
    }
}
