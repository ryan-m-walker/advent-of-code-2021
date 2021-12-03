use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("unable to read file");
    let lines: Vec<_> = input.split("\n").collect();

    let mut counts = vec![];

    for line in lines {
        for (i, bit) in line.chars().enumerate() {
            // if counts.get(i).is_none() {
            //     counts.push(0);
            // }

            // let count = counts.get(i).unwrap();
            // let new_count = if bit == '0' { 
            //     count - 1
            // } else {
            //     count + 1
            // };
            // counts[i] = new_count;
        }
    }
}

