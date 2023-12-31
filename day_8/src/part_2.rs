use std::{collections::HashMap, fs::read_to_string};

pub fn part_2(path: &str) -> i64 {
    let binding = read_to_string(&path).unwrap();
    let lines: Vec<&str> = binding.lines().collect();

    let instructions = lines[0];
    let nodes = &lines[2..];

    // gleiche map wie eben plus alle starting nodes
    let (nodes_map, current_nodes) = nodes_to_map(nodes.to_vec());

    let mut counters: Vec<i64> = Vec::new();
    // part 1 für alle start nodes
    for mut node in current_nodes {
        let mut counter = 0;
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
        counters.push(counter) // alle counter sammeln
    }

    counters.iter().fold(1, |acc, x| lcm(acc, *x)) // lcm von allen values --> return value
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

    nodes_list.retain(|x| x.ends_with('A'));
    (map, nodes_list)
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    let mut _r = 0;
    while a % b > 0 {
        _r = a % b;
        a = b;
        b = _r;
    }
    return b;
}

fn lcm(a: i64, b: i64) -> i64 {
    (a * b) / gcd(a, b)
}
