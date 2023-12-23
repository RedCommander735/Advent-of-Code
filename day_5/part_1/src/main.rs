use std::env;
use std::fs::read_to_string;
use std::ops::Range;

/*
create ranges from start to end of source values

check if range .contains value

if range contains value, add difference between source and destination values

done

*/

// TODO: Fix
fn main() {
    let mut path = env::current_dir().unwrap();
    path.push("input.txt");

    let mut output = match path.to_str() {
        None => panic!("path is not a valid UTF-8 sequence"),
        Some(s) => parse_file(&s),
    };

    output.sort();

    println!("\nLowest location number: {}", output.first().unwrap())
}

fn parse_file(path: &str) -> Vec<i64> {
    let binding = read_to_string(&path).unwrap();
    let sections: Vec<&str> = binding.split("\r\n\r\n").collect();

    let mut maps: Vec<Vec<(Range<i64>, i64)>> = Vec::new();

    for (section_index, section) in sections.iter().enumerate() {
        maps.append(&mut vec![parse_section(section, section_index)])
    }

    let seeds_string: Vec<&str> = sections[0].split(":").collect();
    let mut seeds: Vec<&str> = seeds_string[1].split(" ").collect();
    seeds.retain(|&x| x != "");

    let seeds_parsed: Vec<i64> = seeds.iter().map(|x| x.parse::<i64>().unwrap()).collect();

    let mut parsed_values: Vec<i64> = Vec::new();

    for (index, seed) in seeds_parsed.iter().enumerate() {
        let mut s: i64 = *seed;
        for range_vec in &maps {
            for (range, diff) in range_vec {
                if range.contains(&s) {
                    s += diff;
                    break;
                }
            }
        }
        parsed_values.append(&mut vec![s]);
    }
    parsed_values
}

fn parse_section(section: &str, section_index: usize) -> Vec<(Range<i64>, i64)> {
    let mut range_vec: Vec<(Range<i64>, i64)> = Vec::new();

    for line in section.lines() {
        if line.contains(":") {
            continue;
        }

        let values: Vec<i64> = line.split(" ").map(|x| x.parse::<i64>().unwrap()).collect();
        // destination, source, range

        let range = values[1]..values[1] + values[2];
        let diff = values[0] - values[1];

        range_vec.append(&mut vec![(range, diff)]);
    }
    range_vec
}
