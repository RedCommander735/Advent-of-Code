use std::fs::read_to_string;
use std::iter::successors;
use std::{cmp, env};

// try searching special from numerics and then traversing numeric
// Fix somehow
fn main() {
    let mut path = env::current_dir().unwrap();
    path.push("example.txt");

    let parsed_array = match path.to_str() {
        None => panic!("path is not a valid UTF-8 sequence"),
        Some(s) => parse_file(&s),
    };

    let mut all_numbers: Vec<u32> = Vec::new();

    let mut skip = 0;

    for (row_index, row) in parsed_array.iter().enumerate() {
        for (char_index, char) in row.iter().enumerate() {
            if (char.to_string() != ".") && (char.is_numeric()) {
                if skip > 0 {
                    skip -= 1;
                    continue;
                }
                let (return_value, to_subtract) =
                    check_surrounding(&parsed_array, row_index, char_index);

                skip = return_value
                    .to_string()
                    .len()
                    .checked_sub(to_subtract)
                    .unwrap_or(0);

                all_numbers.append(&mut vec![return_value])
            }
        }
    }
    println!("{:?}", all_numbers);

    let sum: u32 = all_numbers.iter().sum();
    println!("{}", sum);
}

fn check_surrounding(array: &Vec<Vec<char>>, row_index: usize, char_index: usize) -> (u32, usize) {
    println!("Indicator: {}", array[row_index][char_index]);

    let top_row = row_index.checked_sub(1).unwrap_or(0);
    let bottom_row = cmp::min(row_index + 1, row_index);
    let left = char_index.checked_sub(1).unwrap_or(0);
    let right = cmp::min(char_index + 1, char_index);
    let middle = char_index;

    if !(array[top_row][left].is_numeric() && array[top_row][left].to_string() == ".")
        || !(array[top_row][middle].is_numeric() && array[top_row][middle].to_string() == ".")
        || !(array[top_row][right].is_numeric() && array[top_row][right].to_string() == ".")
        || !(array[row_index][left].is_numeric() && array[row_index][left].to_string() == ".")
        || !(array[row_index][right].is_numeric() && array[row_index][right].to_string() == ".")
        || !(array[bottom_row][left].is_numeric() && array[bottom_row][left].to_string() == ".")
        || !(array[bottom_row][middle].is_numeric() && array[bottom_row][middle].to_string() == ".")
        || !(array[bottom_row][right].is_numeric() && array[bottom_row][right].to_string() == ".")
    {
        return traverse_number(&array[row_index], char_index);
    }

    println!("--------------------------------------------------------");

    (0, 0)
}

fn traverse_number(row: &Vec<char>, char_index: usize) -> (u32, usize) {
    let mut digits = vec![row[char_index]];
    let mut start_index = 0;
    println!("Current Row: {:?}\n", row);
    print!("Chars: ");
    for i in (0..char_index.checked_sub(1).unwrap_or(0)).rev() {
        // traverse backwards to find beginning

        print!("{}", row[i]);
        if row[i].is_numeric() {
            digits[0] = row[i];
            start_index = i + 1;
        } else {
            break;
        }
    }
    print!("\n");
    println!("");
    print!("Traverse forward: ");
    for j in start_index..row.len() {
        // traverse forward to find end and collect all digits

        print!("{}", row[j]);
        if row[j].is_numeric() {
            digits.append(&mut vec![row[j]]);
        } else {
            break;
        }
    }
    println!("");
    println!("");
    (
        digits.iter().collect::<String>().parse::<u32>().unwrap(),
        char_index - start_index,
    )
}

fn parse_file(path: &str) -> Vec<Vec<char>> {
    let mut lines: Vec<Vec<char>> = Vec::new();
    for line in read_to_string(&path).unwrap().lines() {
        lines.append(&mut vec![parse_line(line)])
    }
    lines
}

fn parse_line(string: &str) -> Vec<char> {
    let line_chars: Vec<char> = string.chars().collect();
    line_chars
}
