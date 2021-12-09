use std::fs;

fn main() {
    let coords = parse_input();
    let mut map = Map::from_coords(&coords);

    for (start, end) in &coords {
        let coords_to_add = get_coords_between(start, end);

        if coords_to_add.len() > 0 {
            for to_add in coords_to_add {
                map.set(&to_add);
            }
        }
    }

    let answer = map.get_dangerous_spot_count();
    println!("Answer = {}", answer);
}

fn parse_input() -> Vec<(Coord, Coord)> {
    let input = fs::read_to_string("input.txt").unwrap();
    let input: Vec<_> = input.split("\n").collect();
    let lines: Vec<_> = input.into_iter().map(|l| get_coords(l)).collect();
    return lines;
}

fn get_coords(input: &str) -> (Coord, Coord) {
    let input: Vec<_> = input.split(" -> ").collect();
    let left = input.get(0).unwrap();
    let right = input.get(1).unwrap();

    return (Coord::from_str(left), Coord::from_str(right));
}

fn get_max_map_size(coords: &Vec<(Coord, Coord)>) -> (i32, i32) {
    let mut max_x = 0;
    let mut max_y = 0;

    for (left, right) in coords {
        if left.x > max_x {
            max_x = left.x
        }
        if right.x > max_x {
            max_x = right.x
        }

        if left.y > max_y {
            max_y = left.y
        }
        if right.y > max_y {
            max_y = right.y
        }
    }

    (max_x, max_y)
}

fn get_coords_between(start: &Coord, end: &Coord) -> Vec<Coord> {
    let mut coords = vec![];

    let x_diff = end.x - start.x;
    let y_diff = end.y - start.y;

    if x_diff == 0 {
        for y in 0..y_diff.abs() + 1 {
            let new_y = if is_negative(y_diff) {
                start.y - y
            } else {
                start.y + y
            };
            coords.push(Coord::new(start.x, new_y));
        }
    } else if y_diff == 0 {
        for x in 0..x_diff.abs() + 1 {
            let new_x = if is_negative(x_diff) {
                start.x - x
            } else {
                start.x + x
            };
            coords.push(Coord::new(new_x, start.y));
        }
    } else {
        for x in 0..x_diff.abs() + 1 {
            let new_x = if is_negative(x_diff) {
                start.x - x
            } else {
                start.x + x
            };

            let new_y = if is_negative(y_diff) {
                start.y - x
            } else {
                start.y + x
            };

            coords.push(Coord::new(new_x, new_y));
        }
    }

    coords
}

#[derive(Debug)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn from_str(input: &str) -> Self {
        let input: Vec<_> = input.split(",").collect();
        let x = input.get(0).unwrap().parse::<i32>().unwrap();
        let y = input.get(1).unwrap().parse::<i32>().unwrap();

        Self { x, y }
    }
}

#[derive(Debug, Clone)]
struct Map {
    cols: i32,
    rows: i32,
    data: Vec<i32>,
}

impl Map {
    fn from_coords(coords: &Vec<(Coord, Coord)>) -> Self {
        let (cols, rows) = get_max_map_size(&coords);

        Self {
            cols: cols + 1,
            rows: rows + 1,
            data: vec![0; ((cols + 1) * (rows + 1)) as usize],
        }
    }

    fn print(&self) {
        print!("\n");

        for i in 0..self.data.len() {
            if i as i32 % self.cols == 0 && i != 0 {
                print!("\n");
            }

            let item = self.data[i];

            if item == 0 {
                print!(".");
            } else {
                print!("{}", item);
            }
        }

        print!("\n\n");
    }

    fn get(&self, coord: &Coord) -> Option<&i32> {
        let index = self.get_index(coord);
        let item = self.data.get(index as usize);
        return item;
    }

    fn set(&mut self, coord: &Coord) {
        let index = self.get_index(coord);

        self.data[index] = self.data[index] + 1;
    }

    fn get_index(&self, coord: &Coord) -> usize {
        (coord.y * self.cols + coord.x) as usize
    }

    fn get_dangerous_spot_count(&self) -> u32 {
        let mut count = 0;

        for item in &self.data {
            if *item > 1 {
                count += 1;
            }
        }

        count
    }
}

fn is_negative(n: i32) -> bool {
    n < 0
}
