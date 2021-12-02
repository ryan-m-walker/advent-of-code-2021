use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read file");
    let lines: Vec<_> = input.split('\n').collect();

    let mut pos = 0;
    let mut depth = 0;

    for line in lines {
        let split: Vec<_> = line.split(" ").collect();

        let direction = split.get(0).unwrap();

        let amount = split.get(1).unwrap();
        let amount = amount.parse::<u32>().unwrap();

        match *direction {
            "forward" => pos += amount,
            "down" => depth += amount,
            "up" => depth -= amount,
            _ => panic!("unexpected direction")
        }
    }

    println!("Answer = {}", pos * depth);
}
