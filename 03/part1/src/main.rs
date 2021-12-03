use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("unable to read file");
    let lines: Vec<_> = input.split("\n").collect();

    let mut counts = vec![];

    for line in lines {
        for (i, bit) in line.chars().enumerate() {
            if counts.get(i).is_none() {
                counts.push(0);
            }

            let count = counts.get(i).unwrap();
            let new_count = if bit == '0' { 
                count - 1
            } else {
                count + 1
            };
            counts[i] = new_count;
        }
    }

    let mut gamma = vec![];

    for count in counts.iter() {
        if *count >= 0 {
            gamma.push("1");
        } else {
            gamma.push("0");
        }
    }

    let epsilon = flip_bits(&gamma);

    let gamma = u32::from_str_radix(&gamma.join(""), 2).unwrap();
    let epsilon = u32::from_str_radix(&epsilon.join(""), 2).unwrap();

    println!("Answer = {}", gamma * epsilon);
}

fn flip_bits<'a>(bits: &Vec<&str>) -> Vec<&'a str> {
    let mut flipped_bits = vec![];

    for bit in bits {
        if *bit == "0" {
            flipped_bits.push("1");
        } else {
            flipped_bits.push("0");
        }
    }

    flipped_bits
}