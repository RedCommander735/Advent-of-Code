use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;
use std::iter::zip;

// TODO: Fix
fn main() {
    let mut path = env::current_dir().unwrap();
    path.push("example.txt");

    let output = match path.to_str() {
        None => panic!("path is not a valid UTF-8 sequence"),
        Some(s) => parse_file(&s),
    };

    println!("{:?}", output)
}

fn parse_file(path: &str) -> Vec<u32> {
    let binding = read_to_string(&path).unwrap();
    let sections: Vec<&str> = binding.split("\r\n\r\n").collect();

    let mut maps: Vec<Vec<HashMap<u32, u32>>> = Vec::new();

    for section in &sections {
        maps.append(&mut vec![parse_section(section)])
    }

    let seeds_string: Vec<&str> = sections[0].split(":").collect();
    let mut seeds: Vec<&str> = seeds_string[1].split(" ").collect();
    seeds.retain(|&x| x != "");

    let seeds_parsed: Vec<u32> = seeds.iter().map(|x| x.parse::<u32>().unwrap()).collect();

    for mut seed in &seeds_parsed {
        for map_vec in &maps {
            for hash_map in map_vec {
                if hash_map.contains_key(&seed) {
                    seed = &hash_map.get(&seed).unwrap()
                }
            }
        }
    }

    seeds_parsed
}

fn parse_section(section: &str) -> Vec<HashMap<u32, u32>> {
    let mut map_vec: Vec<HashMap<u32, u32>> = Vec::new();

    for line in section.lines() {
        let mut map: HashMap<u32, u32> = HashMap::new();
        if line.contains(":") {
            continue;
        }

        let values: Vec<u32> = line.split(" ").map(|x| x.parse::<u32>().unwrap()).collect();

        for (input, output) in zip(
            values[0]..values[0] + 1 + values[2],
            values[1]..values[1] + 1 + values[2],
        ) {
            map.insert(input, output);
        }

        map_vec.append(&mut vec![map])
    }
    map_vec
}
