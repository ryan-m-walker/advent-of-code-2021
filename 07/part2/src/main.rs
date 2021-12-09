use std::fs;

// slow but it gets the job done

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut crabs: Vec<_> = input
        .split(",")
        .map(|f| f.parse::<i32>().unwrap())
        .collect();
    crabs.sort();

    let lowest = crabs[0];
    let highest = crabs[crabs.len() - 1];

    let mut least_fuel = std::i32::MAX;

    for i in lowest..highest {
        let mut fuel = 0;

        for crab in &crabs {
            let diff = (crab - i).abs();
            for value in 0..diff {
                fuel += value + 1;
            }
        }

        if fuel < least_fuel {
            least_fuel = fuel;
        }
    }

    println!("Answer = {}", least_fuel);
}
