use std::fs::read_to_string;

#[derive(Debug)]
struct Pos {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Cell {
    pos: Pos,
    char: char,
}

#[derive(Debug)]
enum Direction {
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

    for (mut cell, mut from_direction) in starting_points {
        (cell, from_direction) = traverse(grid.clone(), cell, from_direction);
    }

    todo!()
}

fn traverse(
    grid: Vec<Vec<char>>,
    starting_cell: Cell,
    from_direction: Direction,
) -> (Cell, Direction) {
    /*
    TODO:
    - Implement actual traversing (only possible direction for each char)
     */
    todo!()
}

fn establish_grid(lines: Vec<&str>) -> (Vec<Vec<char>>, Pos) {
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
