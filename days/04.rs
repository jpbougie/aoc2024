use grid::Grid;

fn parse_input() -> Grid<char> {
    let mut grid = Grid::new();
    let input = include_str!("../inputs/04.txt");
    for line in input.lines() {
        grid.add_row(line.chars().collect::<Vec<char>>());
    }

    grid
}

const CHARS: [char; 4] = ['X', 'M', 'A', 'S'];
const DIRS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

struct Candidate {
    offset: usize,
    r: usize,
    c: usize,
    dir: (isize, isize),
}

fn part1(input: Grid<char>) -> usize {
    let mut candidates: Vec<Candidate> = Vec::new();

    for cell in input.iter_cells() {
        if *cell.val == 'X' {
            for dir in DIRS {
                candidates.push(Candidate {
                    offset: 1,
                    r: cell.row,
                    c: cell.col,
                    dir,
                });
            }
        }
    }

    let mut found = 0;

    while let Some(candidate) = candidates.pop() {
        if candidate.offset == 4 {
            found += 1;
            continue;
        }

        let new_row: isize = candidate.r as isize + candidate.dir.0;
        let new_col: isize = candidate.c as isize + candidate.dir.1;
        if new_row >= 0 && new_col >= 0 {
            let new_row = new_row as usize;
            let new_col = new_col as usize;

            if let Some(cell) = input.get(new_row, new_col) {
                if *cell.val == CHARS[candidate.offset] {
                    candidates.push(Candidate {
                        offset: candidate.offset + 1,
                        r: new_row,
                        c: new_col,
                        dir: candidate.dir,
                    })
                }
            }
        }
    }

    found
}

fn part2(input: Grid<char>) -> usize {
    let mut found = 0;
    for cell in input.iter_cells() {
        if *cell.val != 'A' {
            continue;
        }
        if cell.row == 0 || cell.col == 0 {
            continue;
        }

        let Some(a) = input.get(cell.row - 1, cell.col - 1).map(|c| *c.val) else {
            continue;
        };

        let Some(b) = input.get(cell.row - 1, cell.col + 1).map(|c| *c.val) else {
            continue;
        };

        let Some(c) = input.get(cell.row + 1, cell.col - 1).map(|c| *c.val) else {
            continue;
        };

        let Some(d) = input.get(cell.row + 1, cell.col + 1).map(|c| *c.val) else {
            continue;
        };

        if !((a == 'M' && d == 'S') || (a == 'S' && d == 'M')) {
            continue;
        }

        if !((c == 'M' && b == 'S') || (c == 'S' && b == 'M')) {
            continue;
        }

        found += 1;
    }

    found
}

pub fn main() {
    let input = parse_input();
    println!("Part 1: {}", part1(input.clone()));
    println!("Part 1: {}", part2(input.clone()));
}
