use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;
use std::iter::zip;

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

fn parse_file(path: &str) -> Vec<u32> {
    let binding = read_to_string(&path).unwrap();
    let sections: Vec<&str> = binding.split("\r\n\r\n").collect();

    let mut maps: Vec<Vec<HashMap<u32, u32>>> = Vec::new();

    println!("\nParsing {} sections.", sections.len());

    for (section_index, section) in sections.iter().enumerate() {
        maps.append(&mut vec![parse_section(section, section_index)])
    }

    println!("Done parsing sections!");

    // println!("{:?}", maps);

    let seeds_string: Vec<&str> = sections[0].split(":").collect();
    let mut seeds: Vec<&str> = seeds_string[1].split(" ").collect();
    seeds.retain(|&x| x != "");

    let seeds_parsed: Vec<u32> = seeds.iter().map(|x| x.parse::<u32>().unwrap()).collect();

    let mut parsed_values: Vec<u32> = Vec::new();

    println!("\nParsing {} seeds.", seeds_parsed.len());

    for (index, seed) in seeds_parsed.iter().enumerate() {
        let mut s: &u32 = seed;
        for map_vec in &maps {
            for hash_map in map_vec {
                if hash_map.contains_key(&s) {
                    s = &hash_map.get(&s).unwrap();
                    break;
                }
            }
        }
        println!("Done parsing seed {}.", index + 1);
        parsed_values.append(&mut vec![*s]);
    }
    println!("Done parsing seeds!");
    parsed_values
}

fn parse_section(section: &str, section_index: usize) -> Vec<HashMap<u32, u32>> {
    let mut map_vec: Vec<HashMap<u32, u32>> = Vec::new();

    for line in section.lines() {
        let mut map: HashMap<u32, u32> = HashMap::new();
        if line.contains(":") {
            continue;
        }

        let values: Vec<u32> = line.split(" ").map(|x| x.parse::<u32>().unwrap()).collect();

        println!("Parsing {} indecies.", values[2]);

        for (index, (output, input)) in zip(
            values[0]..values[0] + values[2],
            values[1]..values[1] + values[2],
        )
        .enumerate()
        {
            map.insert(input, output);
            print!("\rCurrent line: {}", index + 1);
        }
        println!();

        map_vec.append(&mut vec![map])
    }
    println!("Done parsing section {}!", section_index + 1);
    map_vec
}


