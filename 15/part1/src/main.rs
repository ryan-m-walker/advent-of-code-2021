fn main() {
    let input = include_str!("./input.txt");
    let mut map = Map::from_str(input);

    let mut paths = map.get_neighbors(&Path::new((0, 0), 0));

    let mut i = 0;

    while paths.len() > 0 {
        i += 1;
        let mut new_paths = vec![];

        for path in paths {
            let neighbors = map.get_neighbors(&path);
            for n in neighbors {
                new_paths.push(n);
            }
        }

        new_paths.sort_by(|a, b| a.total.cmp(&b.total));

        paths = limit_vec(new_paths, 100000);
    }

    let mut least_risky_path = u32::MAX;

    for risk in &map.finished_paths {
        if *risk < least_risky_path {
            least_risky_path = *risk;
        }
    }

    println!("Answer = {}", least_risky_path);
}

type Coord = (usize, usize);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Path {
    coord: Coord,
    total: u32,
}

impl Path {
    fn new(coord: Coord, total: u32) -> Self {
        Self { coord, total }
    }
}

#[derive(Debug, Clone)]
struct Map {
    data: Vec<Vec<u32>>,
    finished_paths: Vec<u32>,
}

impl Map {
    fn from_str(input: &str) -> Self {
        let data: Vec<_> = input
            .trim()
            .lines()
            .map(|line| {
                let mut nums = vec![];
                let split: Vec<_> = line.split("").collect();
                for num in split {
                    if num == "" {
                        continue;
                    }

                    nums.push(num.parse::<u32>().unwrap());
                }
                return nums;
            })
            .collect();

        Self {
            data,
            finished_paths: vec![],
        }
    }

    fn get(&self, (x, y): Coord) -> Option<&u32> {
        self.data.get(y).and_then(|row| row.get(x))
    }

    fn get_last_coord(&self) -> Coord {
        let row = self.data.len() - 1;
        let col = self.data[0].len() - 1;

        (col, row)
    }

    fn get_neighbors(&mut self, path: &Path) -> Vec<Path> {
        let (x, y) = path.coord;
        let total = path.total;

        let right_coord = (x + 1, y);
        let down_coord = (x, y + 1);

        let mut neighbors: Vec<Path> = vec![];

        let last_coord = self.get_last_coord();
        let last_val = self.get(last_coord).unwrap();

        let mut new_finished_paths = vec![];

        if right_coord == last_coord {
            new_finished_paths.push(last_val + total);
        } else {
            if let Some(value) = self.get(right_coord) {
                neighbors.push(Path::new(right_coord, value + total));
            }
        }

        if down_coord == last_coord {
            new_finished_paths.push(last_val + total);
        } else {
            if let Some(value) = self.get(down_coord) {
                neighbors.push(Path::new(down_coord, value + total));
            }
        }

        self.finished_paths.append(&mut new_finished_paths);

        neighbors
    }
}

fn limit_vec(vec: Vec<Path>, limit: usize) -> Vec<Path> {
    if vec.len() < limit {
        return vec;
    }

    vec[0..limit].to_vec()
}
