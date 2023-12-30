use crate::part_1::nodes_to_map;
use std::fs::read_to_string;

pub fn part_2(path: &str) -> i32 {
    let binding = read_to_string(&path).unwrap();
    let lines: Vec<&str> = binding.lines().collect();

    let instructions = lines[0];
    let nodes = &lines[2..];

    let nodes_map = nodes_to_map(nodes.to_vec());

    let mut current_nodes: Vec<&&str> = nodes_map.keys().clone().collect();

    current_nodes.retain(|x| x.ends_with('A'));

    let mut counter = 0;

    while !(&current_nodes).iter().all(|x| x.ends_with('Z')) {
        for direction in instructions.chars() {
            counter += 1;
            for mut node in &current_nodes {
                let intersection = *nodes_map.get(*node).unwrap();

                if direction == 'L' {
                    node = &&intersection.0
                } else {
                    node = &&intersection.1
                }
            }
        }
    }

    counter
}
