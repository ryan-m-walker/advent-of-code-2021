use std::fs;

fn main() {
    let numbers = fs::read_to_string("numbers.txt").unwrap();
    let numbers: Vec<_> = numbers.split(",").map(|n| n.parse::<u32>().unwrap()).collect();

    let boards = fs::read_to_string("boards.txt").unwrap();
    let boards: Vec<_> = boards.split("\n\n").collect();
    let boards = 
}

fn parse_boards(input: Vec<&str>) -> Vec<u32> {
    

    unimplemented!();
}
