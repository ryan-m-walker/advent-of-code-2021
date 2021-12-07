use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read file");
    let mut fish: Vec<_> = input.split(",").map(|f| f.parse::<u32>().unwrap()).collect();

    for _ in 0..80 {
        let mut new_fish: Vec<u32> = vec![];

        for i in 0..fish.len() {
            let f = fish[i];

            if f == 0 {
                new_fish.push(8);
                fish[i] = 6;
            } else {
                fish[i] = f - 1;
            }
        }

        fish.append(&mut new_fish);
    }

    println!("Answer = {}", fish.len());
}
