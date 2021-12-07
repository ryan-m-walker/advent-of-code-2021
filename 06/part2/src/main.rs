use std::fs;
use std::mem;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read file");
    let fish: Vec<_> = input.split(",").map(|f| f.parse::<u32>().unwrap()).collect();

    let mut buckets: Vec<Bucket> = vec![];

    for i in 0..fish.len() {
        let f = fish[i];
        let existing_index = buckets.iter().position(|b| b.life == f);

        if existing_index.is_none() {
            buckets.push(Bucket::new(1, f));
        } else {
            let existing_index = existing_index.unwrap();
            let bucket = mem::replace(&mut buckets[existing_index], Bucket::new(0, 0));

            let new_count = bucket.count + 1;

            buckets[existing_index] = Bucket::new(
                new_count,
                f
            );
        }
    }


    for _ in 0..256 {
        let mut fish_to_add: u64 = 0;

        for i in 0..buckets.len() {
            let bucket = mem::replace(&mut buckets[i], Bucket::new(0, 0));

            if bucket.life == 0 {
                fish_to_add += bucket.count;
                buckets[i] = Bucket::new(bucket.count, 6);
            } else {
                buckets[i] = Bucket::new(bucket.count, bucket.life - 1);
            }
        }

        
        if fish_to_add > 0 {
            buckets.push(Bucket::new(
                fish_to_add,
                8
            ));
        }
    }

    let mut final_count = 0;

    for bucket in buckets {
        final_count += bucket.count;
    }

    println!("Answer = {}", final_count);
}

#[derive(Debug, Clone, Copy)]
struct Bucket {
    count: u64,
    life: u32,
}

impl Bucket {
    pub fn new(count: u64, life: u32) -> Self {
        Bucket {
            count,
            life,
        }
    }
}
