use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    fmt::{Display, Write as _},
};

use grid::Grid;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Cell {
    Start,
    Exit,
    Wall,
    Empty,
}

impl Cell {
    fn walkable(&self) -> bool {
        match self {
            Cell::Wall => false,
            _ => true,
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Cell::Start => 'S',
            Cell::Exit => 'E',
            Cell::Wall => '#',
            Cell::Empty => '.',
        })
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
    prev: Option<(usize, usize)>,
    cheat_start: Option<(usize, usize)>,
    cheat_end: Option<(usize, usize)>,
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

fn h_cost(pos: (usize, usize), goal: (usize, usize)) -> usize {
    pos.0.abs_diff(goal.0) + pos.1.abs_diff(goal.1)
}

fn shortest_path(grid: &Grid<Cell>) -> Vec<(usize, usize)> {
    let mut to_visit = BinaryHeap::new();
    let Some(start) = grid
        .iter_cells()
        .find(|c| *c.val == Cell::Start)
        .map(|c| (c.row, c.col))
    else {
        panic!("Could not find starting cell");
    };

    let Some(goal) = grid
        .iter_cells()
        .find(|c| *c.val == Cell::Exit)
        .map(|c| (c.row, c.col))
    else {
        panic!("Could not find exit cell");
    };
    to_visit.push(State {
        pos: start,
        g_cost: 0,
        h_cost: h_cost(start, goal),
        cheat_start: None,
        cheat_end: None,
        prev: None,
    });

    let mut visited: HashMap<(usize, usize), (usize, Option<(usize, usize)>)> = HashMap::new();
    while let Some(node) = to_visit.pop() {
        if node.pos == goal {
            let mut path = vec![node.pos];
            let mut step = node.prev;
            while let Some(s) = step {
                step = visited.get(&s).and_then(|n| n.1);
                path.insert(0, s);
            }
            return path;
        }
        if visited.contains_key(&(node.pos.0, node.pos.1)) {
            continue;
        }

        visited.insert((node.pos.0, node.pos.1), (node.g_cost, node.prev));

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
                .is_some_and(|c| c.0 <= new_cost)
            {
                continue;
            }

            to_visit.push(State {
                pos: new_pos,
                g_cost: new_cost,
                h_cost: h_cost(new_pos, goal),
                cheat_start: None,
                cheat_end: None,
                prev: Some(node.pos),
            });
        }
    }

    // We failed
    Vec::new()
}

fn skips(path: Vec<(usize, usize)>, skips_allowed: usize, savings: usize) -> usize {
    let mut ways = 0;
    for i in 0..path.len() {
        for j in (i + savings)..path.len() {
            let dist = h_cost(path[i], path[j]);
            if dist <= skips_allowed && j - (i + dist) >= savings {
                ways += 1;
            }
        }
    }

    ways
}

fn part1(grid: &Grid<Cell>) -> usize {
    let benchmark = shortest_path(grid);
    skips(benchmark, 2, 100)
}

fn part2(grid: &Grid<Cell>) -> usize {
    let benchmark = shortest_path(grid);
    skips(benchmark, 20, 100)
}

fn parse_input() -> Grid<Cell> {
    let input = include_str!("../inputs/20.txt");
    let mut grid = Grid::new();

    for line in input.lines() {
        grid.add_row(
            line.chars()
                .map(|ch| match ch {
                    '#' => Cell::Wall,
                    'E' => Cell::Exit,
                    'S' => Cell::Start,
                    '.' => Cell::Empty,
                    _ => panic!("unexpected char in grid: `{ch}`"),
                })
                .collect(),
        );
    }

    grid
}

pub fn main() {
    let grid = parse_input();
    // println!("{}", grid);
    println!("{}", part1(&grid));
    println!("{}", part2(&grid));
}
