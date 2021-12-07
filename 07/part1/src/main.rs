use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut crabs: Vec<_> = input.split(",").map(|f| f.parse::<i32>().unwrap()).collect();
    crabs.sort();

    let median = crabs.get(crabs.len() / 2).unwrap();

    let mut fuel = 0;

    for crab in &crabs {
        let diff = (crab - median).abs();
        fuel += diff;
    }

    println!("Answer = {}", fuel);
}
