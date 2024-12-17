use core::panic;
use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    fmt::{Display, Write},
};

use grid::Grid;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Move {
    Forward,
    TurnLeft,
    TurnRight,
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy)]
enum Dir {
    N,
    E,
    S,
    W,
}

impl Dir {
    fn ap(&self, pos: (usize, usize), mv: Move) -> ((usize, usize), Dir) {
        (self.new_pos(mv, pos), self.new_dir(mv))
    }

    fn left(&self) -> Self {
        match self {
            Dir::N => Dir::W,
            Dir::E => Dir::N,
            Dir::S => Dir::E,
            Dir::W => Dir::S,
        }
    }

    fn right(&self) -> Self {
        match self {
            Dir::N => Dir::E,
            Dir::E => Dir::S,
            Dir::S => Dir::W,
            Dir::W => Dir::N,
        }
    }

    fn new_pos(&self, mv: Move, pos: (usize, usize)) -> (usize, usize) {
        match mv {
            Move::Forward => match self {
                Dir::N => (pos.0 - 1, pos.1),
                Dir::E => (pos.0, pos.1 + 1),
                Dir::S => (pos.0 + 1, pos.1),
                Dir::W => (pos.0, pos.1 - 1),
            },
            Move::TurnLeft => pos,
            Move::TurnRight => pos,
        }
    }
    fn new_dir(&self, mv: Move) -> Self {
        match mv {
            Move::Forward => *self,
            Move::TurnLeft => self.left(),
            Move::TurnRight => self.right(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct State {
    pos: (usize, usize),
    dir: Dir,
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
            .then(other.dir.cmp(&self.dir))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Move {
    fn cost(&self) -> usize {
        match self {
            Move::Forward => 1,
            _ => 1000,
        }
    }
}

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

fn h_cost(pos: (usize, usize), goal: (usize, usize)) -> usize {
    pos.0.abs_diff(goal.0) + pos.1.abs_diff(goal.1)
}

fn part1(input: &Grid<Cell>) -> usize {
    let mut to_visit = BinaryHeap::new();
    let Some(start) = input
        .iter_cells()
        .find(|c| *c.val == Cell::Start)
        .map(|c| (c.row, c.col))
    else {
        panic!("Could not find starting cell");
    };

    let Some(goal) = input
        .iter_cells()
        .find(|c| *c.val == Cell::Exit)
        .map(|c| (c.row, c.col))
    else {
        panic!("Could not find exit cell");
    };
    to_visit.push(State {
        pos: start,
        dir: Dir::E,
        g_cost: 0,
        h_cost: h_cost(start, goal),
    });

    let mut visited: HashMap<(usize, usize, Dir), usize> = HashMap::new();
    while let Some(node) = to_visit.pop() {
        if node.pos == goal {
            return node.g_cost;
        }

        visited.insert((node.pos.0, node.pos.1, node.dir), node.g_cost);

        let neighbours = vec![Move::TurnLeft, Move::TurnRight, Move::Forward];

        for mv in neighbours {
            let (new_pos, new_dir) = node.dir.ap(node.pos, mv);
            let new_cost = node.g_cost + mv.cost();

            if input
                .get(new_pos.0, new_pos.1)
                .is_none_or(|c| !c.val.walkable())
            {
                continue;
            }

            if visited
                .get(&(new_pos.0, new_pos.1, new_dir))
                .is_some_and(|c| *c <= new_cost)
            {
                continue;
            }

            to_visit.push(State {
                pos: new_pos,
                dir: new_dir,
                g_cost: new_cost,
                h_cost: h_cost(new_pos, goal),
            });
        }
    }

    // We failed
    0
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct StateWithPath {
    pos: (usize, usize),
    dir: Dir,
    g_cost: usize,
    h_cost: usize,
    path: Vec<(usize, usize)>,
}

impl StateWithPath {
    fn f_cost(&self) -> usize {
        self.g_cost + self.h_cost
    }
}

impl Ord for StateWithPath {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .f_cost()
            .cmp(&self.f_cost())
            .then(other.pos.cmp(&self.pos))
            .then(other.dir.cmp(&self.dir))
    }
}

impl PartialOrd for StateWithPath {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn part2(input: &Grid<Cell>) -> usize {
    let mut to_visit = BinaryHeap::new();
    let mut best_goal = usize::MAX;
    let Some(start) = input
        .iter_cells()
        .find(|c| *c.val == Cell::Start)
        .map(|c| (c.row, c.col))
    else {
        panic!("Could not find starting cell");
    };

    let Some(goal) = input
        .iter_cells()
        .find(|c| *c.val == Cell::Exit)
        .map(|c| (c.row, c.col))
    else {
        panic!("Could not find exit cell");
    };
    to_visit.push(StateWithPath {
        pos: start,
        dir: Dir::E,
        g_cost: 0,
        h_cost: h_cost(start, goal),
        path: Vec::new(),
    });

    let mut visited: HashMap<(usize, usize, Dir), (usize, Vec<StateWithPath>)> = HashMap::new();
    while let Some(node) = to_visit.pop() {
        let entry = visited
            .entry((node.pos.0, node.pos.1, node.dir))
            .or_insert_with(|| (usize::MAX, Vec::new()));

        if node.g_cost < entry.0 {
            *entry = (node.g_cost, vec![node.clone()]);
        } else if entry.0 == node.g_cost {
            entry.1.push(node.clone());
        }

        if node.pos == goal && node.g_cost < best_goal {
            best_goal = node.g_cost;
        }

        if node.g_cost > best_goal {
            continue;
        }

        let neighbours = vec![Move::TurnLeft, Move::TurnRight, Move::Forward];

        for mv in neighbours {
            let (new_pos, new_dir) = node.dir.ap(node.pos, mv);
            let new_cost = node.g_cost + mv.cost();

            if input
                .get(new_pos.0, new_pos.1)
                .is_none_or(|c| !c.val.walkable())
            {
                continue;
            }

            if visited
                .get(&(new_pos.0, new_pos.1, new_dir))
                .is_some_and(|(c, _)| *c < new_cost)
            {
                continue;
            }

            let new_path = if mv == Move::Forward {
                let mut path = node.path.clone();
                path.push(new_pos);
                path
            } else {
                node.path.clone()
            };
            to_visit.push(StateWithPath {
                pos: new_pos,
                dir: new_dir,
                g_cost: new_cost,
                h_cost: h_cost(new_pos, goal),
                path: new_path,
            });
        }
    }

    let mut seats = HashSet::new();
    seats.insert(goal);
    seats.insert(start);
    for dir in [Dir::N, Dir::S, Dir::E, Dir::W] {
        if let Some((c, states)) = visited.get(&(goal.0, goal.1, dir)) {
            if *c == best_goal {
                for state in states {
                    for pos in state.path.iter() {
                        seats.insert(*pos);
                    }
                }
            }
        }
    }

    seats.len()
}

fn parse_input() -> Grid<Cell> {
    let input = include_str!("../inputs/16.txt");
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
    let input = parse_input();
    println!("{}", input);
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
