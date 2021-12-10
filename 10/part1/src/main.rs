use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let lines: Vec<_> = input.lines().collect();

    let mut syntax_error_score = 0;

    for line in &lines {
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

            syntax_error_score += get_char_value(c);
            break;
        }
    }

    println!("Answer = {}", syntax_error_score);
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

fn get_char_value(c: char) -> u32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("unexpected character"),
    }
}
