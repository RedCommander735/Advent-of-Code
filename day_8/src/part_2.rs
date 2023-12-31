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

    while !current_nodes.iter().all(|x| x.ends_with('Z')) {
        let mut new_nodes = Vec::new();

        for index in 0..current_nodes.len() {
            let node = &current_nodes[index];
            let intersection = *nodes_map.get(*node).unwrap();

            for direction in instructions.chars() {
                let next_node = if direction == 'L' {
                    &intersection.0
                } else {
                    &intersection.1
                };

                // Assuming ends_with('Z') check needs to be done here
                if next_node.ends_with('Z') {
                    // Handle the case when the node ends with 'Z'
                }

                new_nodes.push(next_node.to_string());
            }
        }

        let new_new_nodes: Vec<&&str> = new_nodes.iter().map(|node| node.as_str()).collect();

        // Update current_nodes with the new values
        current_nodes = new_new_nodes;
        counter += 1;
    }

    counter
}
