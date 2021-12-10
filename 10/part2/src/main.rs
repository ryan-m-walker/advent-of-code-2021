use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let lines: Vec<_> = input.lines().collect();
    let lines: Vec<_> = lines.iter().filter(|l| filter_syntax_errors(l)).collect();

    let mut total_scores = vec![];

    for line in &lines {
        let mut score = 0;
        let mut to_find_stack: Vec<char> = vec![];

        for c in line.chars() {
            let closing_to_find = if to_find_stack.len() > 0 {
                to_find_stack.get(to_find_stack.len() - 1)
            } else {
                None
            };
            if is_opening_tag(c) {
                to_find_stack.push(get_closing_tag(c));
                continue;
            }
            if closing_to_find.is_some() && closing_to_find.unwrap() == &c {
                to_find_stack.pop();
                continue;
            }
        }

        to_find_stack.reverse();

        for c in to_find_stack {
            score *= 5;
            score += get_char_value(c);
        }

        total_scores.push(score);
    }

    total_scores.sort();
    let answer = total_scores[total_scores.len() / 2];

    println!("Answer = {}", answer);
}

fn filter_syntax_errors(line: &str) -> bool {
    let mut to_find_stack: Vec<char> = vec![];

    for c in line.chars() {
        let closing_to_find = if to_find_stack.len() > 0 {
            to_find_stack.get(to_find_stack.len() - 1)
        } else {
            None
        };

        if is_opening_tag(c) {
            to_find_stack.push(get_closing_tag(c));
            continue;
        }

        if closing_to_find.is_some() && closing_to_find.unwrap() == &c {
            to_find_stack.pop();
            continue;
        }

        return false;
    }

    true
}

fn is_opening_tag(c: char) -> bool {
    match c {
        '(' | '[' | '{' | '<' => true,
        _ => false,
    }
}

fn get_closing_tag(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!("unexpected character"),
    }
}

fn get_char_value(c: char) -> u64 {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => panic!("unexpected character"),
    }
}
