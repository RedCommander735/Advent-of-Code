use std::env;
use std::fs::read_to_string;
use std::ops::Range;

fn main() {
    let mut path = env::current_dir().unwrap();
    path.push("example.txt");

    let output = match path.to_str() {
        None => panic!("path is not a valid UTF-8 sequence"),
        Some(s) => parse_file(&s),
    };

    let mut parsed: Vec<i64> = Vec::new();

    for range in output {
        parsed.append(&mut range.collect())
    }

    parsed.sort();

    println!("\nLowest location number: {}", parsed.first().unwrap())
}

fn parse_file(path: &str) -> Vec<Range<i64>> {
    let binding = read_to_string(&path).unwrap();
    let sections: Vec<&str> = binding.split("\r\n\r\n").collect();

    let mut conversion_maps: Vec<Vec<(Range<i64>, i64)>> = Vec::new();

    for section in &sections {
        conversion_maps.append(&mut vec![parse_section(section)])
    }

    let seeds_string: Vec<&str> = sections[0].split(":").collect();
    let mut seeds: Vec<&str> = seeds_string[1].split(" ").collect();
    seeds.retain(|&x| x != "");

    let seed_indecies_parsed: Vec<i64> = seeds.iter().map(|x| x.parse::<i64>().unwrap()).collect();

    let mut seeds_parsed: Vec<Range<i64>> = Vec::new();

    println!("{:?}", seed_indecies_parsed.len());

    for i in (0..seed_indecies_parsed.len()).step_by(2) {
        let seed_range =
            seed_indecies_parsed[i]..seed_indecies_parsed[i] + seed_indecies_parsed[i + 1];
        seeds_parsed.append(&mut vec![seed_range])
    }

    let mut parsed_values: Vec<Range<i64>> = Vec::new();

    for seed in seeds_parsed {
        let mut s: Vec<Range<i64>> = vec![seed];
        for range_vec in &conversion_maps {
            for (range, diff) in range_vec {
                s = process_conversion_range(&s, range, diff)
            }
        }
        parsed_values.append(&mut s);
    }
    println!();
    parsed_values
}

fn parse_section(section: &str) -> Vec<(Range<i64>, i64)> {
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

fn process_conversion_range(
    seed_ranges: &Vec<Range<i64>>,
    conversion_range: &Range<i64>,
    diff: &i64,
) -> Vec<Range<i64>> {
    /*
    Check overlap
    add non matching back to vec, process matching
    */
    let mut parsed: Vec<Range<i64>> = Vec::new();

    for range in seed_ranges {
        let s_start = range.start;
        let s_end = range.end;

        // check if there is overlap
        if s_start <= conversion_range.end && conversion_range.start <= s_end {
            let mut start = conversion_range.start;
            let mut end = conversion_range.end;

            if conversion_range.start <= s_start {
                start = s_start;
            }

            if s_end <= conversion_range.end {
                end = s_end
            }
            parsed.append(&mut vec![(start + diff..end + diff)])
        }

        if s_start < conversion_range.start {
            parsed.append(&mut vec![(s_start..conversion_range.start)])
        }

        if conversion_range.end < s_end {
            parsed.append(&mut vec![(conversion_range.end..s_end)])
        }
    }

    parsed
}
