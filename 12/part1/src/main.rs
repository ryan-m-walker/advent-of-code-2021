use std::collections::{HashMap, HashSet};
use std::fs;
use std::mem;

const CAPS: &'static [char] = &[
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'P', 'Q', 'R', 'S', 'T',
    'U', 'V', 'W', 'X', 'Y', 'Z',
];

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let map = Map::from_string(input);
}

#[derive(Debug, Copy, Clone)]
enum CaveType {
    Start,
    End,
    Large,
    Small,
}

#[derive(Debug, Clone)]
struct Cave {
    id: String,
    cave_type: CaveType,
    neighbors: HashSet<String>,
    visited: bool,
}

impl Cave {
    fn new(id: String, cave_type: CaveType, neighbors: HashSet<String>) -> Self {
        Self {
            id,
            cave_type,
            neighbors,
            visited: false,
        }
    }

    fn add_neighbor(&mut self, neighbor: String) {
        self.neighbors.insert(neighbor);
    }
}

struct Map {
    caves: HashMap<String, Cave>,
}

impl Map {
    fn from_string(input: String) -> Self {
        let mut caves: HashMap<String, Cave> = HashMap::new();

        let lines: Vec<_> = input.lines().collect();

        for line in &lines {
            let components: Vec<_> = line.split("-").collect();
            let left = components[0].trim();
            let right = components[1].trim();

            if caves.get(left).is_none() {
                let cave_type = match left {
                    "start" => CaveType::Start,
                    "end" => CaveType::End,
                    _ => {
                        if is_capitalized(left) {
                            CaveType::Large
                        } else {
                            CaveType::Small
                        }
                    }
                };

                caves.insert(
                    left.to_string(),
                    Cave::new(left.to_string(), cave_type, HashSet::new()),
                );
            }

            let mut left_cave = caves.get(left).unwrap().clone();
            left_cave.add_neighbor(right.to_string());
            caves.insert(left.to_string(), left_cave);

            if caves.get(right).is_none() {
                let cave_type = match right {
                    "start" => CaveType::Start,
                    "end" => CaveType::End,
                    _ => {
                        if is_capitalized(right) {
                            CaveType::Large
                        } else {
                            CaveType::Small
                        }
                    }
                };

                caves.insert(
                    right.to_string(),
                    Cave::new(right.to_string(), cave_type, HashSet::new()),
                );
            }

            let mut right_cave = caves.get(right).unwrap().clone();
            right_cave.add_neighbor(left.to_string());
            caves.insert(right.to_string(), right_cave);
        }

        dbg!(&caves);

        Self { caves }
    }

    fn find_paths(&self) {
        let start = self.caves.get(&String::from("start")).unwrap();
    }
}

fn is_capitalized(input: &str) -> bool {
    let first_char = input.chars().next().unwrap();
    CAPS.contains(&first_char)
}
