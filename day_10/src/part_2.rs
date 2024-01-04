/*
for each line, get left most and right most coordinate (when gap then multiple) xoox---xooox
add all, subtract the number of tiles in a row when 2 get difference when more split into pairs of 2


guter ansatz, funktioniert nicht;
*/

use crate::part_1::{establish_grid, traverse, Cell, Direction, Pos};
use std::{collections::HashMap, fs::read_to_string};

pub fn main(path: &str) -> i32 {
    let binding = read_to_string(&path).unwrap();
    let lines: Vec<&str> = binding.lines().collect();

    let (grid, start) = establish_grid(lines);

    let mut starting_points: Vec<(Cell, Direction)> = Vec::new();
    let mut loop_coords: HashMap<usize, Vec<usize>> = HashMap::new();

    loop_coords.insert(start.y, vec![start.x]);

    // Chech upwards
    if start.y > 0 && "|F7".contains(grid[start.y - 1][start.x]) {
        let cell = Cell {
            pos: Pos {
                x: start.x,
                y: start.y - 1,
            },
            char: grid[start.y - 1][start.x],
        };

        if loop_coords.contains_key(&cell.pos.y) {
            loop_coords.get_mut(&cell.pos.y).unwrap().push(cell.pos.x);
        } else {
            loop_coords.insert(cell.pos.y, vec![cell.pos.x]);
        }

        starting_points.push((cell, Direction::South));
        // downwards
    }
    if start.y < grid.len() - 1 && "|LJ".contains(grid[start.y + 1][start.x]) {
        let cell = Cell {
            pos: Pos {
                x: start.x,
                y: start.y + 1,
            },
            char: grid[start.y + 1][start.x],
        };

        if loop_coords.contains_key(&cell.pos.y) {
            loop_coords.get_mut(&cell.pos.y).unwrap().push(cell.pos.x);
        } else {
            loop_coords.insert(cell.pos.y, vec![cell.pos.x]);
        }

        starting_points.push((cell, Direction::North));
        // left
    }
    if start.x > 0 && "-FL".contains(grid[start.y][start.x - 1]) {
        let cell = Cell {
            pos: Pos {
                x: start.x - 1,
                y: start.y,
            },
            char: grid[start.y][start.x - 1],
        };

        if loop_coords.contains_key(&cell.pos.y) {
            loop_coords.get_mut(&cell.pos.y).unwrap().push(cell.pos.x);
        } else {
            loop_coords.insert(cell.pos.y, vec![cell.pos.x]);
        }

        starting_points.push((cell, Direction::East));
        // right
    }
    if start.x < grid.len() - 1 && "-7J".contains(grid[start.y][start.x + 1]) {
        let cell = Cell {
            pos: Pos {
                x: start.x + 1,
                y: start.y,
            },
            char: grid[start.y][start.x + 1],
        };

        if loop_coords.contains_key(&cell.pos.y) {
            loop_coords.get_mut(&cell.pos.y).unwrap().push(cell.pos.x);
        } else {
            loop_coords.insert(cell.pos.y, vec![cell.pos.x]);
        }

        starting_points.push((cell, Direction::West));
    }

    loop {
        let mut new_points: Vec<(Cell, Direction)> = Vec::new();
        for (cell, from_direction) in starting_points {
            let new = traverse(grid.clone(), cell, from_direction);
            new_points.push(new.clone());

            if loop_coords.contains_key(&new.0.pos.y) {
                if !loop_coords
                    .get(&new.0.pos.y)
                    .unwrap()
                    .contains(&new.0.pos.x)
                {
                    loop_coords.get_mut(&new.0.pos.y).unwrap().push(new.0.pos.x);
                }
            } else {
                loop_coords.insert(new.0.pos.y, vec![new.0.pos.x]);
            }
        }
        starting_points = new_points;

        if &starting_points[0].0.pos == &starting_points[1].0.pos {
            break;
        }
    }

    let tiles_number = find_containing(loop_coords);

    println!("{:?}", tiles_number);
    todo!()
}

fn find_containing(loop_coords: HashMap<usize, Vec<usize>>) -> i32 {
    let mut ranges: Vec<(usize, usize)> = Vec::new();

    for (_key, value) in loop_coords {
        let mut start_index = 0;
        let mut start = value[0];

        let a = (value[0]..=*value.last().unwrap()).collect::<Vec<usize>>();
        if (value[0]..=*value.last().unwrap()).collect::<Vec<usize>>() == value {
            ranges.push((value[0], *value.last().unwrap()));
            continue;
        }

        for index in 0..value.len() {
            let _b = (value[start_index..=index].to_vec());
            let _c = ((start..=value[index]).collect::<Vec<usize>>());
            if (start..=value[index]).collect::<Vec<usize>>() != value[start_index..=index].to_vec()
            {
                ranges.push((start, value[index - 1]));
                start = value[index];
                start_index = index;
            }

            if (start..=value[index]).collect::<Vec<usize>>()
                == value[start_index..value.len()].to_vec()
            {
                ranges.push((start, value[index]));
            }
        }
    }
    let mut accu = 0;

    for range in ranges {
        accu += range.1 - range.0 + 1;
    }

    accu as i32
}
