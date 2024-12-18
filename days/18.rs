use std::collections::{BinaryHeap, HashMap};

use grid::Grid;

fn parse_input() -> Vec<(usize, usize)> {
    let input = include_str!("../inputs/18.txt");

    input
        .lines()
        .map(|l| {
            let mut parts = l.split(',').map(|d| d.parse().unwrap());
            (parts.next().unwrap(), parts.next().unwrap())
        })
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Free,
    Blocked,
}

impl Cell {
    fn walkable(&self) -> bool {
        *self == Cell::Free
    }
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy)]
enum Dir {
    N,
    E,
    S,
    W,
}

impl Dir {
    fn ap(&self, pos: (usize, usize)) -> Option<(usize, usize)> {
        match self {
            Dir::N => pos.0.checked_add_signed(-1).map(|x| (x, pos.1)),
            Dir::E => Some((pos.0, pos.1 + 1)),
            Dir::S => Some((pos.0 + 1, pos.1)),
            Dir::W => pos.1.checked_add_signed(-1).map(|y| (pos.0, y)),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct State {
    pos: (usize, usize),
    g_cost: usize,
    h_cost: usize,
}

impl State {
    fn f_cost(&self) -> usize {
        self.g_cost + self.h_cost
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .f_cost()
            .cmp(&self.f_cost())
            .then(other.pos.cmp(&self.pos))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn part1(falls: &[(usize, usize)]) -> usize {
    let mut grid: Grid<Cell> = Grid::with_capacity(71);
    for _i in 0..=70 {
        grid.add_row(vec![Cell::Free; 71]);
    }

    for f in &falls[0..1024] {
        grid.set(f.0, f.1, Cell::Blocked);
    }

    shortest_path(&grid)
}

fn part2(falls: &[(usize, usize)]) -> (usize, usize) {
    let mut grid: Grid<Cell> = Grid::with_capacity(71);
    for _i in 0..=70 {
        grid.add_row(vec![Cell::Free; 71]);
    }

    for f in &falls[0..1024] {
        grid.set(f.0, f.1, Cell::Blocked);
    }

    let mut i = 1023;

    while shortest_path(&grid) > 0 {
        i += 1;
        grid.set(falls[i].0, falls[i].1, Cell::Blocked);
    }

    falls[i]
}

fn h_cost(pos: (usize, usize), goal: (usize, usize)) -> usize {
    pos.0.abs_diff(goal.0) + pos.1.abs_diff(goal.1)
}

fn shortest_path(grid: &Grid<Cell>) -> usize {
    let mut to_visit = BinaryHeap::new();
    let start = (0, 0);
    let goal = (70, 70);
    to_visit.push(State {
        pos: start,
        g_cost: 0,
        h_cost: h_cost(start, goal),
    });

    let mut visited: HashMap<(usize, usize), usize> = HashMap::new();
    while let Some(node) = to_visit.pop() {
        if node.pos == goal {
            return node.g_cost;
        }
        if visited.contains_key(&(node.pos.0, node.pos.1)) {
            continue;
        }

        visited.insert((node.pos.0, node.pos.1), node.g_cost);

        let neighbours = vec![Dir::N, Dir::S, Dir::E, Dir::W];

        for dir in neighbours {
            let Some(new_pos) = dir.ap(node.pos) else {
                continue;
            };
            let new_cost = node.g_cost + 1;

            if grid
                .get(new_pos.0, new_pos.1)
                .is_none_or(|c| !c.val.walkable())
            {
                continue;
            }

            if visited
                .get(&(new_pos.0, new_pos.1))
                .is_some_and(|c| *c <= new_cost)
            {
                continue;
            }

            to_visit.push(State {
                pos: new_pos,
                g_cost: new_cost,
                h_cost: h_cost(new_pos, goal),
            });
        }
    }

    // We failed
    0
}
pub fn main() {
    let input = parse_input();
    println!("Part 1: {}", part1(&input));
    let p2 = part2(&input);
    println!("Part 2: {},{}", p2.0, p2.1);
}
