use std::collections::HashMap;
use std::fs::read_to_string;
use std::{cmp, env};

fn main() {
    let mut path = env::current_dir().unwrap();
    path.push("input.txt");

    let output = match path.to_str() {
        None => panic!("path is not a valid UTF-8 sequence"),
        Some(s) => parse_file(&s),
    };

    let mut points = 0;

    for (_, value) in &output {
        points += value;
    }

    println!("Points: {}", points);
}

fn parse_file(path: &str) -> HashMap<usize, u32> {
    let mut map: HashMap<usize, u32> = HashMap::new();
    let binding = read_to_string(&path).unwrap();
    let lines: Vec<&str> = binding.lines().collect();
    for (line_index, line) in lines.iter().enumerate() {
        let line: Vec<&str> = line.split(":").collect();
        parse_card(&line[1], line_index, &mut map, lines.len() - 1);
    }
    map
}

fn parse_card(
    string: &str,
    line_index: usize,
    map: &mut HashMap<usize, u32>,
    max_line_index: usize,
) -> () {
    if map.contains_key(&line_index) {
        *map.get_mut(&line_index).unwrap() += 1;
    } else {
        map.insert(line_index, 1);
    }
    let mut points = 0;
    let card: Vec<&str> = string.split("|").collect();
    let mut winning_numbers: Vec<&str> = card[0].split(" ").collect();
    let mut local_numbers: Vec<&str> = card[1].split(" ").collect();
    winning_numbers.retain(|&x| x != "");
    local_numbers.retain(|&x| x != "");

    for number in local_numbers {
        if winning_numbers.contains(&number) {
            points += 1;
        }
    }

    for i in line_index + 1..cmp::min(line_index + points + 1, max_line_index + 1) {
        *map.entry(i).or_insert(0) += 1 * map.get(&line_index).unwrap_or_else(|| -> &u32 { &1 });
    }
}
