use std::env;
use std::fs::read_to_string;

// try searching special from numerics and then traversing numeric
// Fix somehow
fn main() {
    let mut path = env::current_dir().unwrap();
    path.push("input.txt");

    let parsed_array = match path.to_str() {
        None => panic!("path is not a valid UTF-8 sequence"),
        Some(s) => parse_file(&s),
    };

    let mut all_positions: Vec<(usize, usize)> = Vec::new();

    for (row_index, row) in parsed_array.iter().enumerate() {
        for (char_index, char) in row.char_indices() {
            // println!("Current pos: ({},{})", row_index, char_index);
            //top row
            if (char != '.') && char.is_ascii_digit() {
                if row_index > 0 {
                    let top_row = &parsed_array[row_index - 1];
                    if (top_row.chars().nth(char_index).unwrap() != '.')
                        && !(top_row.chars().nth(char_index).unwrap().is_ascii_digit())
                    {
                        all_positions.append(&mut vec![(row_index, char_index)])
                    }

                    if char_index > 0 {
                        if (top_row.chars().nth(char_index - 1).unwrap() != '.')
                            && !(top_row
                                .chars()
                                .nth(char_index - 1)
                                .unwrap()
                                .is_ascii_digit())
                        {
                            all_positions.append(&mut vec![(row_index, char_index)])
                        };
                    }
                    if char_index < row.len() - 1 {
                        if (top_row.chars().nth(char_index + 1).unwrap() != '.')
                            && !(top_row
                                .chars()
                                .nth(char_index + 1)
                                .unwrap()
                                .is_ascii_digit())
                        {
                            all_positions.append(&mut vec![(row_index, char_index)])
                        };
                    }
                }

                //middle row
                if char_index > 0 {
                    if (row.chars().nth(char_index - 1).unwrap() != '.')
                        && !(row.chars().nth(char_index - 1).unwrap().is_ascii_digit())
                    {
                        all_positions.append(&mut vec![(row_index, char_index)])
                    };
                }
                if char_index < row.len() - 1 {
                    if (row.chars().nth(char_index + 1).unwrap() != '.')
                        && !(row.chars().nth(char_index + 1).unwrap().is_ascii_digit())
                    {
                        all_positions.append(&mut vec![(row_index, char_index)])
                    };
                }

                //bottom row
                if row_index < parsed_array.len() - 1 {
                    let bottom_row = &parsed_array[row_index + 1];
                    if (bottom_row.chars().nth(char_index).unwrap() != '.')
                        && !(bottom_row.chars().nth(char_index).unwrap().is_ascii_digit())
                    {
                        all_positions.append(&mut vec![(row_index, char_index)])
                    }

                    if char_index > 0 {
                        if (bottom_row.chars().nth(char_index - 1).unwrap() != '.')
                            && !(bottom_row
                                .chars()
                                .nth(char_index - 1)
                                .unwrap()
                                .is_ascii_digit())
                        {
                            all_positions.append(&mut vec![(row_index, char_index)])
                        };
                    }
                    if char_index < row.len() - 1 {
                        if (bottom_row.chars().nth(char_index + 1).unwrap() != '.')
                            && !(bottom_row
                                .chars()
                                .nth(char_index + 1)
                                .unwrap()
                                .is_ascii_digit())
                        {
                            all_positions.append(&mut vec![(row_index, char_index)])
                        };
                    }
                }
            }
        }
    }

    let mut last_pos: &(usize, usize) = &all_positions[0];
    let mut positions_without_duplicates: Vec<(usize, usize)> = Vec::new();

    for position in &all_positions {
        if (last_pos.0 == position.0
            && !(last_pos.1 == position.1 - 1)) //|| last_pos.1 == position.1 - 2
            || (last_pos.0 != position.0)
        {
            positions_without_duplicates.append(&mut vec![position.clone()]);
            last_pos = position;
        }
    }

    // println!("{:?}", all_positions);
    // println!("{:?}", positions_without_duplicates);

    let mut all_numbers: Vec<u32> = Vec::new();
    let mut last_num = 0;

    for pos in &positions_without_duplicates {
        // println!("Current pos: ({},{})", pos.0, pos.1);
        let num = traverse_number(&parsed_array[pos.0].chars().collect(), pos.1);
        if last_num != num {
            all_numbers.append(&mut vec![num]);
        }

        last_num = num;
    }

    // println!("{:?}", all_numbers);

    let sum: u32 = all_numbers.iter().sum();
    println!("{}", sum);
}

fn parse_file(path: &str) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    for line in read_to_string(&path).unwrap().lines() {
        lines.append(&mut vec![line.to_owned()])
    }
    lines
}

fn traverse_number(row: &Vec<char>, char_index: usize) -> u32 {
    let mut digits = vec![row[char_index]];
    let mut start_index = char_index + 1;
    // println!("Current Row: {:?}\n", row);
    // print!("Chars: ");
    for i in (0..char_index).rev() {
        // traverse backwards to find beginning

        // print!("{}", row[i]);
        if row[i].is_ascii_digit() {
            digits[0] = row[i];
            start_index = i + 1;
        } else {
            break;
        }
    }
    // print!("\nStartindex: {}\n", start_index);
    // println!("");
    // print!("Traverse forward: ");
    for j in start_index..row.len() {
        // traverse forward to find end and collect all digits

        // print!("{}", row[j]);
        if row[j].is_ascii_digit() {
            digits.append(&mut vec![row[j]]);
        } else {
            break;
        }
    }
    // println!("");
    // println!("");
    digits.iter().collect::<String>().parse::<u32>().unwrap()
}
