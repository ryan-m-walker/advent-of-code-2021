use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("unable to read file");

    let numbers: Vec<_> = input
        .split("\n")
        .map(|x| x.parse::<f32>().unwrap())
        .collect();

    let mut count = 0;

    for (i, number) in numbers.iter().enumerate() {
        if i == 0 {
            continue;
        }

        let prev = numbers.get(i - 1).unwrap();

        if prev < number {
            count += 1;
        }
    }

    println!("Count = {}", count);
}
