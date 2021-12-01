use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("unable to read file");

    let numbers: Vec<_> = input
        .split("\n")
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    let mut count: u32 = 0;
    let mut last_sum: Option<u32> = None;

    for (i, number) in numbers.iter().enumerate() {
        let a = number;
        let b = numbers.get(i + 1);
        let c = numbers.get(i + 2);

        if b.is_some() && c.is_some() {
            let sum: u32 = a + b.unwrap() + c.unwrap();

            if last_sum.is_some() {
                if last_sum.unwrap() < sum {
                    count += 1
                }
            }

            last_sum = Some(sum);
        }
    }

    println!("Count = {}", count);
}
