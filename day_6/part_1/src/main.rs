use std::{env, fs::read_to_string, iter::zip};

fn main() {
    let mut path = env::current_dir()
        .unwrap()
        .parent()
        .unwrap()
        .canonicalize()
        .unwrap();
    path.push("input");

    let output = match path.to_str() {
        None => panic!("path is not a valid UTF-8 sequence"),
        Some(s) => parse_file(&s),
    };

    println!("\nPossible combinations: {}", output)
}

fn parse_file(path: &str) -> u64 {
    let binding = read_to_string(&path).unwrap();

    let mut times: Vec<u64> = Vec::new();
    let mut distances: Vec<u64> = Vec::new();

    for line in binding.lines() {
        if line.starts_with("Time:") {
            let line = line.strip_prefix("Time:").unwrap();
            let mut t_line: Vec<&str> = line.split(" ").collect();
            t_line.retain(|&x| x != "");
            times = t_line.iter().map(|x| x.parse::<u64>().unwrap()).collect();
        }

        if line.starts_with("Distance:") {
            let line = line.strip_prefix("Distance:").unwrap();
            let mut d_line: Vec<&str> = line.split(" ").collect();
            d_line.retain(|&x| x != "");
            distances = d_line.iter().map(|x| x.parse::<u64>().unwrap()).collect();
        }
    }

    let mut wins: u64 = 0;
    let mut win_vec: Vec<u64> = Vec::new();

    for (time, distance) in zip(times, distances) {
        wins = 0;
        for x in 1..time {
            if distance < ((time - x) * x) {
                wins += 1;
            }
        }
        win_vec.append(&mut vec![wins])
    }
    win_vec.iter().product()
}
