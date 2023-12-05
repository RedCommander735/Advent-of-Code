use std::env;
use std::fs::read_to_string;

fn main() {
    let mut path = env::current_dir().unwrap();
    path.push("input.txt");

    let parsed_array = match path.to_str() {
        None => panic!("path is not a valid UTF-8 sequence"),
        Some(s) => parse_file(s),
    };

    let mut all_positions: Vec<Vec<(usize, usize)>> = Vec::new();

    for (row_index, row) in parsed_array.iter().enumerate() {
        for (char_index, char) in row.char_indices() {
            //top row
            if (char != '.') && !char.is_ascii_digit() && (char == '*') {
                let mut per_gear: Vec<(usize, usize)> = Vec::new();
                if row_index > 0 {
                    let top_row = &parsed_array[row_index - 1];
                    if (top_row.chars().nth(char_index).unwrap() != '.')
                        && (top_row.chars().nth(char_index).unwrap().is_ascii_digit())
                    {
                        per_gear.append(&mut vec![(row_index - 1, char_index)])
                    }

                    if char_index > 0
                        && (top_row.chars().nth(char_index - 1).unwrap() != '.')
                        && (top_row
                            .chars()
                            .nth(char_index - 1)
                            .unwrap()
                            .is_ascii_digit())
                    {
                        per_gear.append(&mut vec![(row_index - 1, char_index - 1)])
                    }
                    if char_index < row.len() - 1
                        && (top_row.chars().nth(char_index + 1).unwrap() != '.')
                        && (top_row
                            .chars()
                            .nth(char_index + 1)
                            .unwrap()
                            .is_ascii_digit())
                    {
                        per_gear.append(&mut vec![(row_index - 1, char_index + 1)])
                    }
                }

                //middle row
                if char_index > 0
                    && (row.chars().nth(char_index - 1).unwrap() != '.')
                    && (row.chars().nth(char_index - 1).unwrap().is_ascii_digit())
                {
                    per_gear.append(&mut vec![(row_index, char_index - 1)])
                }
                if char_index < row.len() - 1
                    && (row.chars().nth(char_index + 1).unwrap() != '.')
                    && (row.chars().nth(char_index + 1).unwrap().is_ascii_digit())
                {
                    per_gear.append(&mut vec![(row_index, char_index + 1)])
                }

                //bottom row
                if row_index < parsed_array.len() - 1 {
                    let bottom_row = &parsed_array[row_index + 1];
                    if (bottom_row.chars().nth(char_index).unwrap() != '.')
                        && (bottom_row.chars().nth(char_index).unwrap().is_ascii_digit())
                    {
                        per_gear.append(&mut vec![(row_index + 1, char_index)])
                    }

                    if char_index > 0
                        && (bottom_row.chars().nth(char_index - 1).unwrap() != '.')
                        && (bottom_row
                            .chars()
                            .nth(char_index - 1)
                            .unwrap()
                            .is_ascii_digit())
                    {
                        per_gear.append(&mut vec![(row_index + 1, char_index - 1)])
                    }
                    if char_index < row.len() - 1
                        && (bottom_row.chars().nth(char_index + 1).unwrap() != '.')
                        && (bottom_row
                            .chars()
                            .nth(char_index + 1)
                            .unwrap()
                            .is_ascii_digit())
                    {
                        per_gear.append(&mut vec![(row_index + 1, char_index + 1)])
                    }
                }
                all_positions.append(&mut vec![per_gear])
            }
        }
    }

    let mut all_number_pairs: Vec<Vec<u32>> = Vec::new();
    let mut num_starting_pos: Vec<(usize, usize)> = Vec::new();
    let mut last_num = 0;

    for pos in &all_positions {
        let mut num_vec: Vec<u32> = Vec::new();
        for p in pos {
            let (num, starting_index) = traverse_number(&parsed_array[p.0].chars().collect(), p.1);
            if !num_starting_pos.contains(&(p.0, starting_index)) {
                num_starting_pos.append(&mut vec![(p.0, starting_index)]);
                num_vec.append(&mut vec![num]);
            }

            last_num = num;
        }
        if num_vec.len() >= 2 {
            all_number_pairs.append(&mut vec![num_vec]);
        }
    }

    println!("{:?}", all_number_pairs);

    let mut nums: Vec<u32> = Vec::new();

    for pair in all_number_pairs {
        nums.append(&mut vec![pair[0] * pair[1]])
    }
    let sum: u32 = nums.iter().sum();
    println!("{}", sum);
}

fn parse_file(path: &str) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    for line in read_to_string(path).unwrap().lines() {
        lines.append(&mut vec![line.to_owned()])
    }
    lines
}

fn traverse_number(row: &Vec<char>, char_index: usize) -> (u32, usize) {
    let mut digits = vec![row[char_index]];
    let mut start_index = char_index + 1;
    for i in (0..char_index).rev() {
        // traverse backwards to find beginning
        if row[i].is_ascii_digit() {
            digits[0] = row[i];
            start_index = i + 1;
        } else {
            break;
        }
    }
    for j in start_index..row.len() {
        // traverse forward to find end and collect all digits
        if row[j].is_ascii_digit() {
            digits.append(&mut vec![row[j]]);
        } else {
            break;
        }
    }
    (
        digits.iter().collect::<String>().parse::<u32>().unwrap(),
        start_index - 1,
    )
}
