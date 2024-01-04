use std::fs::read_to_string;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct Cell {
    pub pos: Pos,
    pub char: char,
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub fn main(path: &str) -> i32 {
    let binding = read_to_string(&path).unwrap();
    let lines: Vec<&str> = binding.lines().collect();

    let (grid, start) = establish_grid(lines);

    let mut starting_points: Vec<(Cell, Direction)> = Vec::new();

    // Chech upwards
    if start.y > 0 && "|F7".contains(grid[start.y - 1][start.x]) {
        let cell = Cell {
            pos: Pos {
                x: start.x,
                y: start.y - 1,
            },
            char: grid[start.y - 1][start.x],
        };

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

        starting_points.push((cell, Direction::West));
    }
    let mut dist = 0;

    let distance = loop {
        dist += 1;
        let mut new_points: Vec<(Cell, Direction)> = Vec::new();
        for (cell, from_direction) in starting_points {
            let new = traverse(grid.clone(), cell, from_direction);
            new_points.push(new)
        }
        starting_points = new_points;

        if &starting_points[0].0.pos == &starting_points[1].0.pos {
            break dist;
        }
    };

    distance + 1
}

pub fn traverse(
    grid: Vec<Vec<char>>,
    starting_cell: Cell,
    from_direction: Direction,
) -> (Cell, Direction) {
    let x = starting_cell.pos.x;
    let y = starting_cell.pos.y;

    let char = starting_cell.char;

    let next: (Pos, Direction) = match from_direction {
        Direction::North => match char {
            '|' => Some((Pos { x, y: y + 1 }, Direction::North)),
            'L' => Some((Pos { x: x + 1, y }, Direction::West)),
            'J' => Some((Pos { x: x - 1, y }, Direction::East)),
            _ => None,
        },
        Direction::East => match char {
            '-' => Some((Pos { x: x - 1, y }, Direction::East)),
            'L' => Some((Pos { x, y: y - 1 }, Direction::South)),
            'F' => Some((Pos { x, y: y + 1 }, Direction::North)),
            _ => None,
        },
        Direction::South => match char {
            '|' => Some((Pos { x, y: y - 1 }, Direction::South)),
            '7' => Some((Pos { x: x - 1, y }, Direction::East)),
            'F' => Some((Pos { x: x + 1, y }, Direction::West)),
            _ => None,
        },
        Direction::West => match char {
            '-' => Some((Pos { x: x + 1, y }, Direction::West)),
            'J' => Some((Pos { x, y: y - 1 }, Direction::South)),
            '7' => Some((Pos { x, y: y + 1 }, Direction::North)),
            _ => None,
        },
    }
    .unwrap();

    let next_pos = next.0;
    let next_dir = next.1;

    let next_cell: Cell = Cell {
        pos: next_pos.clone(),
        char: grid[next_pos.y][next_pos.x],
    };

    (next_cell, next_dir)
}

pub fn establish_grid(lines: Vec<&str>) -> (Vec<Vec<char>>, Pos) {
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut start: Pos = Pos { x: 0, y: 0 }; // x, y
    for (y, line) in lines.iter().enumerate() {
        let mut l: Vec<char> = Vec::new();
        for (x, char) in line.chars().enumerate() {
            if char == 'S' {
                start.x = x;
                start.y = y;
            }
            l.push(char);
        }
        grid.push(l);
    }

    (grid, start)
}
