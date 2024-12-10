use std::collections::{HashMap, HashSet, VecDeque};

use grid::Grid;

fn parse_input() -> Grid<u8> {
    let input = include_str!("../inputs/10.txt");
    let mut grid = Grid::new();
    for line in input.lines() {
        grid.add_row(
            line.chars()
                .map(|ch| ch.to_digit(10).unwrap() as u8)
                .collect(),
        );
    }

    grid
}

fn part1(input: Grid<u8>) -> usize {
    let mut start_nodes = Vec::new();
    let mut exit_nodes = HashSet::new();
    let mut edges: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();
    let mut labels = HashMap::new();

    for cell in input.iter_cells() {
        labels.insert((cell.row, cell.col), *cell.val);
        if *cell.val == 0 {
            start_nodes.push((cell.row, cell.col));
        }

        if *cell.val == 9 {
            exit_nodes.insert((cell.row, cell.col));
            continue;
        }

        for (row, col) in input.straight_neighbours(cell.row, cell.col) {
            if let Some(n) = input.get(row, col) {
                if *cell.val + 1 == *n.val {
                    let entry = edges.entry((cell.row, cell.col)).or_default();
                    entry.push((n.row, n.col));
                }
            }
        }
    }

    let mut score = 0;
    for start in start_nodes {
        score += reach_nines(start, &exit_nodes, &edges, &labels);
    }

    score
}

fn reach_nines(
    start: (usize, usize),
    exit_nodes: &HashSet<(usize, usize)>,
    edges: &HashMap<(usize, usize), Vec<(usize, usize)>>,
    labels: &HashMap<(usize, usize), u8>,
) -> usize {
    let mut to_visit = Vec::new();
    to_visit.push(start);
    let mut reached = 0;
    let mut visited = HashSet::new();

    while let Some(node) = to_visit.pop() {
        if visited.contains(&node) {
            continue;
        }
        let label = labels.get(&node).unwrap();
        if *label == 9 {
            reached += 1;
        }

        visited.insert(node);

        if let Some(next_nodes) = edges.get(&node) {
            for edge in next_nodes {
                to_visit.push(*edge);
            }
        }
    }

    reached
}

fn part2(input: Grid<u8>) -> usize {
    let mut start_nodes = Vec::new();
    let mut exit_nodes = HashSet::new();
    let mut edges: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();
    let mut labels = HashMap::new();

    for cell in input.iter_cells() {
        labels.insert((cell.row, cell.col), *cell.val);
        if *cell.val == 0 {
            start_nodes.push((cell.row, cell.col));
        }

        if *cell.val == 9 {
            exit_nodes.insert((cell.row, cell.col));
            continue;
        }

        for (row, col) in input.straight_neighbours(cell.row, cell.col) {
            if let Some(n) = input.get(row, col) {
                if *cell.val + 1 == *n.val {
                    let entry = edges.entry((cell.row, cell.col)).or_default();
                    entry.push((n.row, n.col));
                }
            }
        }
    }

    let mut score = 0;
    for start in start_nodes {
        score += rating_nines(start, &exit_nodes, &edges, &labels);
    }

    score
}

fn rating_nines(
    start: (usize, usize),
    exit_nodes: &HashSet<(usize, usize)>,
    edges: &HashMap<(usize, usize), Vec<(usize, usize)>>,
    labels: &HashMap<(usize, usize), u8>,
) -> usize {
    let mut to_visit = Vec::new();
    to_visit.push(start);
    let mut reached = 0;

    while let Some(node) = to_visit.pop() {
        let label = labels.get(&node).unwrap();
        if *label == 9 {
            reached += 1;
        }

        if let Some(next_nodes) = edges.get(&node) {
            for edge in next_nodes {
                to_visit.push(*edge);
            }
        }
    }

    reached
}

pub fn main() {
    let input = parse_input();
    println!("Part 1: {}", part1(input.clone()));
    println!("Part 2: {}", part2(input.clone()));
}
