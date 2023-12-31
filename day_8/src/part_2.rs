use std::{collections::HashMap, fs::read_to_string};

pub fn part_2(path: &str) -> i32 {
    let binding = read_to_string(&path).unwrap();
    let lines: Vec<&str> = binding.lines().collect();

    let instructions = lines[0];
    let nodes = &lines[2..];

    let (nodes_map, mut current_nodes) = nodes_to_map(nodes.to_vec());

    current_nodes.retain(|x| x.ends_with('A'));

    let mut counter = 0;
    for mut node in current_nodes {
        while !node.ends_with('Z') {
            for direction in instructions.chars() {
                counter += 1;

                let intersection = *nodes_map.get(node).unwrap();

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

fn nodes_to_map(nodes: Vec<&str>) -> (HashMap<&str, (&str, &str)>, Vec<&str>) {
    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
    let mut nodes_list: Vec<&str> = Vec::new();

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
        nodes_list.push(key)
    }

    (map, nodes_list)
}
