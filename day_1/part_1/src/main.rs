use std::env;
use std::fs::read_to_string;

fn main() {
    let mut path = env::current_dir().unwrap();
    path.push("input.txt");

    let output = match path.to_str() {
        None => panic!("path is not a valid UTF-8 sequence"),
        Some(s) => parse_file(&s),
    };

    println!("{}", output);
}

fn parse_file(path: &str) -> u32 {
    let mut all_numbers: Vec<u32> = Vec::new();
    for line in read_to_string(&path).unwrap().lines() {
        let line = line.to_string();
        let numbers = parse_line(&line);
        let number: u32 = numbers.first().unwrap() * 10 + numbers.last().unwrap();
        all_numbers.append(&mut vec![number]);
    }
    let sum: u32 = all_numbers.iter().sum();
    sum
}

fn parse_line(string: &str) -> Vec<u32> {
    let mut numbers = Vec::new();

    for char in string.chars() {
        if char.is_numeric() {
            numbers.append(&mut vec![char.to_digit(10).unwrap()])
        }
    }
    numbers
}
