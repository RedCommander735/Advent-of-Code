use std::fs::read_to_string;

pub fn main(path: &str) -> i32 {
    let binding = read_to_string(&path).unwrap();
    let lines: Vec<&str> = binding.lines().collect();

    let mut next_values = Vec::<i32>::new();

    for line in lines {
        next_values.push(extrapolate_next_value(line));
    }

    next_values.iter().sum()
}

fn extrapolate_next_value(line: &str) -> i32 {
    let mut sequence: Vec<i32> = line.split(' ').map(|n| n.parse::<i32>().unwrap()).collect();

    let mut sequences: Vec<Vec<i32>> = Vec::new();
    sequences.push(sequence.clone());

    while !sequence.iter().all(|n| *n == 0) {
        let mut diff: Vec<i32> = Vec::new();

        for index in 1..sequence.len() {
            diff.push(sequence[index] - sequence[index - 1]);
        }

        sequences.push(diff.clone());
        sequence = diff;
    }

    sequences.pop();
    sequences.reverse();

    let mut last_diff = 0;

    for seq in &sequences {
        last_diff += seq.last().unwrap()
    }

    last_diff
}
