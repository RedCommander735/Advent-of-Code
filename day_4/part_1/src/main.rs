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
    let mut points = 0;
    for line in read_to_string(&path).unwrap().lines() {
        let line: Vec<&str> = line.split(":").collect();
        points += parse_line(&line[1]);
    }
    points
}

fn parse_line(string: &str) -> u32 {
    let mut points = 0;
    let card: Vec<&str> = string.split("|").collect();
    let mut winning_numbers: Vec<&str> = card[0].split(" ").collect();
    let mut local_numbers: Vec<&str> = card[1].split(" ").collect();
    winning_numbers.retain(|&x| x != "");
    local_numbers.retain(|&x| x != "");

    for number in local_numbers {
        if winning_numbers.contains(&number) {
            if points == 0 {
                points = 1
            } else {
                points *= 2
            }
        }
    }

    points
}
