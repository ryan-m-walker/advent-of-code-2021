use std::fs;
use std::mem;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read file");
    let fish: Vec<_> = input.split(",").map(|f| f.parse::<u32>().unwrap()).collect();

    let mut buckets: Vec<Bucket> = vec![];

    for i in 0..fish.len() {
        let f = fish[i];
        let existing_bucket = buckets.iter().find(|b| b.count == f);

        if existing_bucket.is_none() {
            buckets.push(Bucket::new(1, f));
        } else {
            let mut existing_bucket = existing_bucket.unwrap();
            let new_count = existing_bucket.count + 1;
            mem::replace(&mut existing_bucket, &Bucket::new(
                new_count,
                f
            ),);
        }
    }

    println!("BUCKETS {:?}", buckets.len());

    for gen in 0..80 {
        let mut fish_to_add: u32 = 0;

        for i in 0..buckets.len() {
            let mut bucket = &buckets[i];

            if bucket.life == 0 {
                fish_to_add += bucket.count;
                let bucket_count = bucket.count;

                mem::replace(&mut bucket, &Bucket::new(
                    bucket_count,
                    6
                ));
            } else {
                let bucket_count = bucket.count;
                let new_life = bucket.life - 1;

                mem::replace(&mut bucket, &Bucket::new(
                    bucket_count,
                    new_life,
                ));
            }
        }

        buckets.push(Bucket::new(
            fish_to_add,
            8
        ));
    }

    // let mut final_count = 0;

    println!("BUCKETS {:?}", buckets);

    // for bucket in buckets {
    //     final_count += bucket.count;
    // }

    // println!("Answer = {}", final_count);
}

#[derive(Debug)]
struct Bucket {
    count: u32,
    life: u32,
}

impl Bucket {
    pub fn new(count: u32, life: u32) -> Self {
        Bucket {
            count,
            life,
        }
    }
}