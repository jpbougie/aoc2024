use std::collections::HashSet;

use grid::Grid;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Cell {
    Free,
    Blocked,
}

#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
    N,
    S,
    E,
    W,
}

impl Dir {
    fn mv(&self, pos: &mut (isize, isize)) {
        match self {
            Dir::N => pos.0 -= 1,
            Dir::S => pos.0 += 1,
            Dir::E => pos.1 += 1,
            Dir::W => pos.1 -= 1,
        }
    }

    fn clockwise(&self) -> Dir {
        match self {
            Dir::N => Dir::E,
            Dir::S => Dir::W,
            Dir::E => Dir::S,
            Dir::W => Dir::N,
        }
    }
}

fn parse_input() -> (Grid<Cell>, (isize, isize), Dir) {
    let input = include_str!("../inputs/06.txt");
    let mut pos = (0, 0);
    let mut orientiation = Dir::N;

    let mut grid = Grid::new();

    for (row, line) in input.lines().enumerate() {
        grid.add_row(
            line.chars()
                .enumerate()
                .map(|(col, ch)| match ch {
                    '.' => Cell::Free,
                    '#' => Cell::Blocked,
                    '^' => {
                        pos = (row as isize, col as isize);
                        orientiation = Dir::N;
                        Cell::Free
                    }
                    '>' => {
                        pos = (row as isize, col as isize);
                        orientiation = Dir::E;
                        Cell::Free
                    }
                    'V' => {
                        pos = (row as isize, col as isize);
                        orientiation = Dir::S;
                        Cell::Free
                    }
                    '<' => {
                        pos = (row as isize, col as isize);
                        orientiation = Dir::W;
                        Cell::Free
                    }

                    _ => panic!("unkown ch `{ch}`"),
                })
                .collect(),
        );
    }

    (grid, pos, orientiation)
}

fn part1(grid: Grid<Cell>, mut pos: (isize, isize), mut dir: Dir) -> usize {
    let mut visited = HashSet::new();

    while pos.0 >= 0 && pos.1 >= 0 && grid.get(pos.0 as usize, pos.1 as usize).is_some() {
        visited.insert(pos);
        let mut new_pos = pos;
        dir.mv(&mut new_pos);
        if new_pos.0 < 0 || new_pos.1 < 0 {
            break;
        }

        let Some(c) = grid.get(new_pos.0 as usize, new_pos.1 as usize) else {
            break;
        };

        if *c.val == Cell::Free {
            pos = new_pos;
        } else {
            dir = dir.clockwise();
        }
    }

    visited.len()
}

enum Exit {
    OutOfBounds,
    Loop,
}
fn potentials(grid: Grid<Cell>, mut pos: (isize, isize), mut dir: Dir) -> HashSet<(isize, isize)> {
    let mut visited = HashSet::new();

    while pos.0 >= 0 && pos.1 >= 0 && grid.get(pos.0 as usize, pos.1 as usize).is_some() {
        visited.insert(pos);
        let mut new_pos = pos;
        dir.mv(&mut new_pos);
        if new_pos.0 < 0 || new_pos.1 < 0 {
            break;
        }

        let Some(c) = grid.get(new_pos.0 as usize, new_pos.1 as usize) else {
            break;
        };

        if *c.val == Cell::Free {
            pos = new_pos;
        } else {
            dir = dir.clockwise();
        }
    }

    visited
}

fn test(grid: Grid<Cell>, mut pos: (isize, isize), mut dir: Dir) -> Exit {
    let mut visited: HashSet<((isize, isize), Dir)> = HashSet::new();

    while pos.0 >= 0 && pos.1 >= 0 && grid.get(pos.0 as usize, pos.1 as usize).is_some() {
        if visited.contains(&(pos, dir)) {
            return Exit::Loop;
        }
        visited.insert((pos, dir));

        let mut new_pos = pos;
        dir.mv(&mut new_pos);
        if new_pos.0 < 0 || new_pos.1 < 0 {
            break;
        }

        let Some(c) = grid.get(new_pos.0 as usize, new_pos.1 as usize) else {
            break;
        };

        if *c.val == Cell::Free {
            pos = new_pos;
        } else {
            dir = dir.clockwise();
        }
    }

    Exit::OutOfBounds
}

fn part2(grid: Grid<Cell>, pos: (isize, isize), dir: Dir) -> usize {
    let potentials = potentials(grid.clone(), pos, dir);
    let mut count = 0;
    for (i, j) in potentials {
        if i == pos.0 && j == pos.1 {
            continue;
        }
        if grid
            .get(i as usize, j as usize)
            .is_some_and(|c| *c.val == Cell::Free)
        {
            let mut grid = grid.clone();
            grid.set(i as usize, j as usize, Cell::Blocked);
            if matches!(test(grid, pos, dir), Exit::Loop) {
                count += 1;
            }
        }
    }

    count
}

pub fn main() {
    let (grid, pos, dir) = parse_input();
    println!("Part 1: {}", part1(grid.clone(), pos, dir));
    println!("Part 2: {}", part2(grid.clone(), pos, dir));
}
