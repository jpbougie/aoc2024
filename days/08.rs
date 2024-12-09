use std::{
    collections::{HashMap, HashSet},
    iter,
};

use grid::Grid;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
enum Cell {
    Empty,
    Freq(char),
}

fn parse_input() -> Grid<Cell> {
    let input = include_str!("../inputs/08.txt");
    let mut grid = Grid::new();

    for line in input.lines() {
        grid.add_row(
            line.chars()
                .map(|ch| match ch {
                    '.' => Cell::Empty,
                    x => Cell::Freq(x),
                })
                .collect(),
        );
    }

    grid
}

fn pairs(input: &Grid<Cell>) -> HashMap<char, Vec<(isize, isize)>> {
    let mut res: HashMap<char, Vec<(isize, isize)>> = HashMap::new();

    for cell in input.iter_cells() {
        if let Cell::Freq(f) = cell.val {
            let entry = res.entry(*f).or_default();
            entry.push((cell.row as isize, cell.col as isize));
        }
    }

    res
}

pub fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m %= n;
    }
    n
}
fn add_antinodes(
    positions: &[(isize, isize)],
    res: &mut HashSet<(isize, isize)>,
    max: (isize, isize),
) {
    if positions.len() <= 1 {
        return;
    }
    let base = positions.first().unwrap();

    for other in positions.iter().skip(1) {
        let d = diff(*base, *other);

        let before = (base.0 - d.0, base.1 - d.1);
        if before.0 >= 0 && before.0 <= max.0 && before.1 >= 0 && before.1 <= max.1 {
            res.insert(before);
        }

        let after = (other.0 + d.0, other.1 + d.1);
        if after.0 >= 0 && after.0 <= max.0 && after.1 >= 0 && after.1 <= max.1 {
            res.insert(after);
        }
    }
}

fn diff(base: (isize, isize), other: (isize, isize)) -> (isize, isize) {
    (other.0 - base.0, other.1 - base.1)
}

fn part1(input: &Grid<Cell>) -> usize {
    let pairs = pairs(input);
    let mut res = HashSet::new();
    let max = (
        input.row_count() as isize - 1,
        input.col_count() as isize - 1,
    );

    for (_freq, positions) in pairs {
        for i in 0..(positions.len() - 1) {
            add_antinodes(&positions[i..], &mut res, max);
        }
    }

    res.len()
}

fn add_all_antinodes(
    positions: &[(isize, isize)],
    res: &mut HashSet<(isize, isize)>,
    max: (isize, isize),
) {
    if positions.len() <= 1 {
        return;
    }
    let base = positions.first().unwrap();

    for other in positions.iter().skip(1) {
        let d = diff(*base, *other);

        let div = gcd(d.0.unsigned_abs() as u64, d.1.unsigned_abs() as u64) as isize;

        let d = (d.0 / div, d.1 / div);

        let mut before = *base;
        while before.0 >= 0 && before.0 <= max.0 && before.1 >= 0 && before.1 <= max.1 {
            res.insert(before);
            before = (before.0 - d.0, before.1 - d.1);
        }

        let mut after = *base;
        while after.0 >= 0 && after.0 <= max.0 && after.1 >= 0 && after.1 <= max.1 {
            res.insert(after);
            after = (after.0 + d.0, after.1 + d.1);
        }
    }
}

fn part2(input: &Grid<Cell>) -> usize {
    let pairs = pairs(input);
    let mut res = HashSet::new();
    let max = (
        input.row_count() as isize - 1,
        input.col_count() as isize - 1,
    );

    for (_freq, positions) in pairs {
        for i in 0..(positions.len() - 1) {
            add_all_antinodes(&positions[i..], &mut res, max);
        }
    }

    res.len()
}

pub fn main() {
    let input = parse_input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
