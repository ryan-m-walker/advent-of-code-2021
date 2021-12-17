use std::collections::HashMap;

const STEPS: u8 = 10;

fn main() {
    let mut template: Vec<_> = parse_template(include_str!("./template.txt"));
    let insertions = parse_insertions(include_str!("./insertions.txt"));

    for step in 0..STEPS {
        let mut new_template = vec![];

        dbg!(step);

        for window in template.windows(2) {
            let first = window[0];
            let pair = window.join("");
            let insertion = insertions.iter().find(|x| x.0 == pair);

            new_template.push(first);

            if insertion.is_some() {
                let (_, to_insert) = insertion.unwrap();
                new_template.push(to_insert);
            }
        }

        let last = template[template.len() - 1];
        new_template.push(last);

        template = new_template;
    }

    let counts = get_counts(&template);
    let (highest, lowest) = get_highest_and_lowest(&counts);
    println!("Answer = {}", highest - lowest);
}

fn parse_template(input: &str) -> Vec<&str> {
    input.split("").filter(|x| !x.is_empty()).collect()
}

fn parse_insertions(input: &str) -> Vec<(&str, &str)> {
    input
        .lines()
        .map(|l| {
            let components: Vec<_> = l.split(" -> ").collect();
            let pair = components[0];
            let insertion = components[1];

            (pair, insertion)
        })
        .collect()
}

fn get_counts(input: &Vec<&str>) -> HashMap<String, u32> {
    let mut counts: HashMap<String, u32> = HashMap::new();

    for c in input {
        if counts.get(&c.to_string()).is_none() {
            counts.insert(c.to_string(), 0);
        }

        let count = counts.get_mut(&c.to_string()).unwrap();
        *count += 1;
    }

    counts
}

fn get_highest_and_lowest(input: &HashMap<String, u32>) -> (u32, u32) {
    let mut highest = (&String::from(""), &0);
    let mut lowest = (&String::from(""), &u32::MAX);

    for (key, count) in input.into_iter() {
        if count > highest.1 {
            highest = (key, count);
        }

        if count < lowest.1 {
            lowest = (key, count);
        }
    }

    (highest.1.to_owned(), lowest.1.to_owned())
}
