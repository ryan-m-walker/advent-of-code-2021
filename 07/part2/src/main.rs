use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut crabs: Vec<_> = input.split(",").map(|f| f.parse::<i32>().unwrap()).collect();
    crabs.sort();

    let mut sum = 0;
    for crab in &crabs {
        sum += crab;
    }
    let average = sum / crabs.len() as i32;

    println!("AVERAGE = {}", average);

    // let mut fuel = 0;

    // for crab in &crabs {
        // let diff = (crab - median).abs();
        // fuel += diff;
    // }

    // println!("Answer = {}", fuel);
}