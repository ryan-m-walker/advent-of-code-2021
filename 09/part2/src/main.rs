use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut map = Map::from_string(input);
    map.get_basins();
}

enum MapPoint {
    Visited,
    Unvisited,
    Boundary,
}

struct Map {
    data: Vec<MapPoint>,
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

                if parsed == 9 {
                    data.push(MapPoint::Boundary);
                } else {
                    data.push(MapPoint::Unvisited);
                }
            }
        }

        Self { data, cols, rows }
    }

    fn get_basins(&mut self) {
        let mut basins = vec![];

        for row in 0..self.rows {
            for col in 0..self.cols {
                let point = self.get(col, row).unwrap();

                if let MapPoint::Unvisited = point {
                    let mut to_visit = vec![(col, row)];
                    let mut basin_size = 0;

                    self.set_visited(col, row);

                    loop {
                        if to_visit.len() == 0 {
                            break;
                        }

                        basin_size += 1;

                        let (x, y) = to_visit.pop().unwrap();
                        let mut neighbors = self.get_neighbors(x, y);

                        for n in &neighbors {
                            self.set_visited(n.0, n.1);
                        }

                        to_visit.append(&mut neighbors);
                    }

                    basins.push(basin_size);
                }
            }
        }

        basins.sort_by(|a, b| b.cmp(a));

        let a = basins[0];
        let b = basins[1];
        let c = basins[2];

        println!("Answer = {}", a * b * c);
    }

    fn get_neighbors(&mut self, x: i32, y: i32) -> Vec<(i32, i32)> {
        let mut neighbors = vec![];

        let top = self.get(x, y - 1);
        let right = self.get(x + 1, y);
        let bottom = self.get(x, y + 1);
        let left = self.get(x - 1, y);

        if top.is_some() {
            if let MapPoint::Unvisited = top.unwrap() {
                neighbors.push((x, y - 1));
            }
        }
        if bottom.is_some() {
            if let MapPoint::Unvisited = bottom.unwrap() {
                neighbors.push((x, y + 1));
            }
        }
        if x + 1 < self.cols && right.is_some() {
            if let MapPoint::Unvisited = right.unwrap() {
                neighbors.push((x + 1, y));
            }
        }
        if x > 0 && left.is_some() {
            if let MapPoint::Unvisited = left.unwrap() {
                neighbors.push((x - 1, y));
            }
        }

        neighbors
    }

    fn get_index(&self, x: i32, y: i32) -> usize {
        (y * self.cols + x) as usize
    }

    fn get(&self, x: i32, y: i32) -> Option<&MapPoint> {
        let index = self.get_index(x, y);
        self.data.get(index)
    }

    fn set_visited(&mut self, x: i32, y: i32) {
        let index = self.get_index(x, y);
        self.data[index] = MapPoint::Visited;
    }
}
