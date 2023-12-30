use std::{collections::HashMap, fs::read_to_string};

pub fn part_1(path: &str) -> i32 {
    let binding = read_to_string(&path).unwrap();
    let lines: Vec<&str> = binding.lines().collect();

    let instructions = lines[0];
    let nodes = &lines[2..];

    let nodes_map = nodes_to_map(nodes.to_vec());

    let mut current_node = "AAA";

    let mut counter = 0;

    while current_node != "ZZZ" {
        for direction in instructions.chars() {
            counter += 1;
            let intersection = nodes_map.get(current_node).unwrap();

            if direction == 'L' {
                current_node = intersection.0
            } else {
                current_node = intersection.1
            }
        }
    }

    counter
}

pub fn nodes_to_map(nodes: Vec<&str>) -> HashMap<&str, (&str, &str)> {
    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();

    for node in nodes {
        let content: Vec<&str> = node.split('=').map(|c| c.trim()).collect();
        let key = content[0];

        let directions: Vec<&str> = content[1]
            .trim_matches(|d| d == '(' || d == ')')
            .split(',')
            .map(|s| s.trim())
            .collect();
        let val: (&str, &str) = (directions[0], directions[1]);
        map.insert(key, val);
    }

    map
}
