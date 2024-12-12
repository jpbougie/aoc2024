use std::collections::{HashMap, HashSet};

#[derive(Debug, Default)]
struct Region {
    plant: char,
    vertices: Vec<(usize, usize)>,
    edges: HashMap<(usize, usize), Vec<(usize, usize)>>,
}

impl Region {
    fn price(&self) -> usize {
        self.vertices.len()
            * self.vertices.iter().fold(0, |s, v| {
                s + 4
                    - self
                        .edges
                        .get(v)
                        .map(|edges| edges.len())
                        .unwrap_or_default()
            })
    }

    fn price2(&self) -> usize {
        self.vertices.len() * self.sides()
    }

    fn sides(&self) -> usize {
        // Reddit made me discover that you can count corners and you'll get the number of sides
        let mut corners = 0;
        let points: HashSet<_> = self
            .vertices
            .iter()
            .map(|(row, col)| (*row as isize, *col as isize))
            .collect();

        for pos in points.iter().cloned() {
            // TOP-LEFT corner
            if !points.contains(&Side::L.ap(pos)) && !points.contains(&Side::T.ap(pos)) {
                corners += 1;
            }

            if points.contains(&Side::L.ap(pos))
                && points.contains(&Side::T.ap(pos))
                && !points.contains(&Side::TL.ap(pos))
            {
                corners += 1;
            }

            // TOP-RIGHT corner
            if !points.contains(&Side::R.ap(pos)) && !points.contains(&Side::T.ap(pos)) {
                corners += 1;
            }

            if points.contains(&Side::R.ap(pos))
                && points.contains(&Side::T.ap(pos))
                && !points.contains(&Side::TR.ap(pos))
            {
                corners += 1;
            }

            // BOTTOM-RIGHT corner
            if !points.contains(&Side::R.ap(pos)) && !points.contains(&Side::B.ap(pos)) {
                corners += 1;
            }

            if points.contains(&Side::R.ap(pos))
                && points.contains(&Side::B.ap(pos))
                && !points.contains(&Side::BR.ap(pos))
            {
                corners += 1;
            }

            // BOTTOM-LEFT corner
            if !points.contains(&Side::L.ap(pos)) && !points.contains(&Side::B.ap(pos)) {
                corners += 1;
            }

            if points.contains(&Side::L.ap(pos))
                && points.contains(&Side::B.ap(pos))
                && !points.contains(&Side::BL.ap(pos))
            {
                corners += 1;
            }
        }

        corners
    }

    // This crawls the walls, which gives the correct count for polygons without holes
    fn broken_sides(&self) -> usize {
        let mut turns = 0;
        let start_pos = *self.vertices.first().unwrap();
        let mut pos = start_pos;
        let start_dir = Dir::East;
        let mut dir = Dir::East;

        loop {
            if let Some(fwd) = dir
                .forward(pos)
                .filter(|p| self.vertices.iter().any(|v| v == p))
            {
                if let Some(fl) = dir
                    .left(fwd)
                    .filter(|p| self.vertices.iter().any(|v| v == p))
                {
                    turns += 1;
                    dir = dir.turn_left();
                    pos = fl;
                } else {
                    pos = fwd;
                }
            } else {
                turns += 1;
                dir = dir.turn_right();
            }
            if pos == start_pos && dir == start_dir {
                break;
            }
        }

        turns
    }
}

enum Side {
    T,
    B,
    L,
    R,
    TL,
    TR,
    BL,
    BR,
}

impl Side {
    fn ap(&self, pos: (isize, isize)) -> (isize, isize) {
        match self {
            Side::T => (pos.0 - 1, pos.1),
            Side::B => (pos.0 + 1, pos.1),
            Side::L => (pos.0, pos.1 - 1),
            Side::R => (pos.0, pos.1 + 1),
            Side::TL => (pos.0 - 1, pos.1 - 1),
            Side::TR => (pos.0 - 1, pos.1 + 1),
            Side::BL => (pos.0 + 1, pos.1 - 1),
            Side::BR => (pos.0 + 1, pos.1 + 1),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Dir {
    North,
    South,
    East,
    West,
}

impl Dir {
    fn forward(&self, pos: (usize, usize)) -> Option<(usize, usize)> {
        match self {
            Dir::East => Some((pos.0, pos.1 + 1)),
            Dir::North => {
                if pos.0 > 0 {
                    Some((pos.0 - 1, pos.1))
                } else {
                    None
                }
            }
            Dir::South => Some((pos.0 + 1, pos.1)),
            Dir::West => {
                if pos.1 > 0 {
                    Some((pos.0, pos.1 - 1))
                } else {
                    None
                }
            }
        }
    }

    fn left(&self, pos: (usize, usize)) -> Option<(usize, usize)> {
        self.turn_left().forward(pos)
    }

    fn turn_left(&self) -> Dir {
        match self {
            Dir::North => Dir::West,
            Dir::South => Dir::East,
            Dir::East => Dir::North,
            Dir::West => Dir::South,
        }
    }

    fn turn_right(&self) -> Dir {
        match self {
            Dir::North => Dir::East,
            Dir::South => Dir::West,
            Dir::East => Dir::South,
            Dir::West => Dir::North,
        }
    }
}

fn parse_input() -> Vec<Region> {
    let input = include_str!("../inputs/12.txt");
    let mut grid = grid::Grid::new();
    for line in input.lines() {
        grid.add_row(line.chars().collect());
    }

    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut res = Vec::new();

    for cell in grid.iter_cells() {
        let pt = (cell.row, cell.col);
        if visited.contains(&pt) {
            continue;
        }

        let mut region: Region = Region {
            plant: *cell.val,
            ..Default::default()
        };

        let mut potential_neighbours = vec![pt];
        while let Some(n) = potential_neighbours.pop() {
            if visited.contains(&n) {
                continue;
            }

            visited.insert(n);

            region.vertices.push(n);
            let neighbours: Vec<(usize, usize)> = grid
                .straight_neighbours(n.0, n.1)
                .iter()
                .filter_map(|nn| grid.get(nn.0, nn.1))
                .filter(|cell| *cell.val == region.plant)
                .map(|cell| (cell.row, cell.col))
                .collect();

            for neigh in neighbours.iter() {
                potential_neighbours.push(*neigh);
            }

            region.edges.insert(n, neighbours);
        }

        res.push(region);
    }

    res
}

fn part1(input: &[Region]) -> usize {
    input.iter().fold(0, |s, r| s + r.price())
}

fn part2(input: &[Region]) -> usize {
    input.iter().fold(0, |s, r| s + r.price2())
}

pub fn main() {
    let regions = parse_input();
    println!("Part 1: {}", part1(&regions));
    println!("Part 1: {}", part2(&regions));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sides() {
        let region = Region {
            plant: 'a',
            vertices: vec![(0, 0)],
            ..Default::default()
        };

        assert_eq!(4, region.sides());

        let region = Region {
            plant: 'a',
            vertices: vec![(0, 0), (0, 1)],
            ..Default::default()
        };

        assert_eq!(4, region.sides());

        let region = Region {
            plant: 'a',
            vertices: vec![(0, 0), (0, 1), (1, 0)],
            ..Default::default()
        };

        assert_eq!(6, region.sides());
    }
}
