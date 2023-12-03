use fancy_regex::Regex;
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
        // println!("{:?}", numbers);
        let number: u32 = numbers.first().unwrap() * 10 + numbers.last().unwrap();
        // println!("{:?}", number);
        all_numbers.append(&mut vec![number]);
    }
    let sum: u32 = all_numbers.iter().sum();
    sum
}

fn parse_line(string: &str) -> Vec<u32> {
    let mut numbers = Vec::new();
    let preprocessed = &string
        .replace("one", "onee")
        .replace("three", "threee")
        .replace("five", "fivee")
        .replace("nine", "ninee")
        .replace("eight", "eightt")
        .replace("seven", "sevenn")
        .replace("two", "twoo");

    let regex = Regex::new("(\\d|one|two|three|four|five|six|seven|eight|nine)").unwrap();

    for _match in regex.find_iter(&preprocessed) {
        // match all matches to actual numbers, return
        // println!("{:?}", _match);

        let current = match _match.unwrap().as_str() {
            "one" | "1" => 1,
            "two" | "2" => 2,
            "three" | "3" => 3,
            "four" | "4" => 4,
            "five" | "5" => 5,
            "six" | "6" => 6,
            "seven" | "7" => 7,
            "eight" | "8" => 8,
            "nine" | "9" => 9,
            &_ => unreachable!(),
        };
        numbers.append(&mut vec![current])
    }
    numbers
}
