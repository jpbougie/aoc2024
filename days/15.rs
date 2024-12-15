use std::{
    collections::HashSet,
    fmt::{Display, Write},
    hash::Hash,
};

use grid::Grid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Cell {
    Wall,
    Box,
    Empty,
    Robot,
    BoxLeft,
    BoxRight,
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Cell::Wall => '#',
            Cell::Box => 'O',
            Cell::Empty => '.',
            Cell::Robot => '@',
            Cell::BoxLeft => '[',
            Cell::BoxRight => ']',
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Move {
    U,
    D,
    L,
    R,
}

impl Move {
    fn ap(&self, v: (usize, usize)) -> (usize, usize) {
        match self {
            Move::U => (v.0 - 1, v.1),
            Move::D => (v.0 + 1, v.1),
            Move::L => (v.0, v.1 - 1),
            Move::R => (v.0, v.1 + 1),
        }
    }

    fn can_push_two(&self) -> bool {
        match self {
            Move::U | Move::D => true,
            Move::L | Move::R => false,
        }
    }
}

#[derive(Clone)]
struct Game {
    robot: (usize, usize),
    grid: Grid<Cell>,
    moves: Vec<Move>,
}

impl Game {
    fn apply_simple_moves(&mut self) {
        'skip_move: for mv in self.moves.iter() {
            let mut boxes_to_move = Vec::new();
            let mut to_check = mv.ap(self.robot);
            while let Some(cell) = self.grid.get(to_check.0, to_check.1) {
                if cell.val == &Cell::Box {
                    boxes_to_move.push((cell.row, cell.col));
                    to_check = mv.ap(to_check);
                }

                if cell.val == &Cell::Empty {
                    break;
                }

                if cell.val == &Cell::Wall {
                    continue 'skip_move;
                }
            }

            self.grid.set(self.robot.0, self.robot.1, Cell::Empty);
            self.robot = mv.ap(self.robot);
            self.grid.set(self.robot.0, self.robot.1, Cell::Robot);
            if !boxes_to_move.is_empty() {
                self.grid.set(to_check.0, to_check.1, Cell::Box);
            }
        }
    }

    fn sum(&self) -> usize {
        self.grid
            .iter_cells()
            .filter_map(|c| {
                if *c.val == Cell::Box || *c.val == Cell::BoxLeft {
                    Some(100 * c.row + c.col)
                } else {
                    None
                }
            })
            .sum()
    }

    fn check_moves(
        grid: &Grid<Cell>,
        mv: &Move,
        to_move: &mut HashSet<(usize, usize, Cell)>,
        pos: (usize, usize),
    ) -> bool {
        let Some(c) = grid.get(pos.0, pos.1) else {
            return false;
        };

        if to_move.contains(&(pos.0, pos.1, *c.val)) {
            return true;
        }

        if *c.val == Cell::Empty {
            return true;
        }

        if *c.val == Cell::Wall {
            return false;
        }

        if *c.val == Cell::BoxLeft {
            to_move.insert((pos.0, pos.1, *c.val));
            to_move.insert((pos.0, pos.1 + 1, Cell::BoxRight));

            Self::check_moves(grid, mv, to_move, (pos.0, pos.1 + 1))
                && Self::check_moves(grid, mv, to_move, mv.ap((pos.0, pos.1)))
                && Self::check_moves(grid, mv, to_move, mv.ap((pos.0, pos.1 + 1)))
        } else if *c.val == Cell::BoxRight {
            to_move.insert((pos.0, pos.1, *c.val));
            to_move.insert((pos.0, pos.1 - 1, Cell::BoxLeft));

            Self::check_moves(grid, mv, to_move, (pos.0, pos.1 - 1))
                && Self::check_moves(grid, mv, to_move, mv.ap((pos.0, pos.1)))
                && Self::check_moves(grid, mv, to_move, mv.ap((pos.0, pos.1 - 1)))
        } else {
            panic!("Should not get here");
        }
    }

    fn apply_moves(&mut self) {
        'skip_move: for mv in self.moves.iter() {
            if mv.can_push_two() {
                let mut to_move = HashSet::new();
                if Self::check_moves(&self.grid, mv, &mut to_move, mv.ap(self.robot)) {
                    // HACK: don't overwrite things to empty cells if we've just visited them. This should really be done in order instead
                    let mut visited = HashSet::new();

                    for (row, col, c) in to_move.into_iter() {
                        if !visited.contains(&(row, col)) {
                            self.grid.set(row, col, Cell::Empty);
                        }
                        let new_pos = mv.ap((row, col));
                        self.grid.set(new_pos.0, new_pos.1, c);
                        visited.insert(new_pos);
                    }
                    self.grid.set(self.robot.0, self.robot.1, Cell::Empty);
                    self.robot = mv.ap(self.robot);
                    self.grid.set(self.robot.0, self.robot.1, Cell::Robot);
                }
            } else {
                let mut boxes_to_move = Vec::new();
                let mut to_check = mv.ap(self.robot);
                while let Some(cell) = self.grid.get(to_check.0, to_check.1) {
                    if matches!(*cell.val, Cell::BoxLeft | Cell::BoxRight) {
                        boxes_to_move.push((cell.row, cell.col, *cell.val));
                        to_check = mv.ap(to_check);
                    }

                    if cell.val == &Cell::Empty {
                        break;
                    }

                    if cell.val == &Cell::Wall {
                        continue 'skip_move;
                    }
                }

                for b in boxes_to_move.into_iter().rev() {
                    let new_pos = mv.ap((b.0, b.1));
                    self.grid.set(new_pos.0, new_pos.1, b.2);
                }
                self.grid.set(self.robot.0, self.robot.1, Cell::Empty);
                self.robot = mv.ap(self.robot);
                self.grid.set(self.robot.0, self.robot.1, Cell::Robot);
            }
        }
    }
}

fn parse_input(expand: bool) -> Game {
    let input = include_str!("../inputs/15.txt");
    let mut grid = Grid::new();
    let mut parts = input.split("\n\n");
    for line in parts.next().unwrap().lines() {
        grid.add_row(
            line.chars()
                .flat_map(|ch| {
                    if !expand {
                        vec![match ch {
                            '#' => Cell::Wall,
                            '.' => Cell::Empty,
                            'O' => Cell::Box,
                            '@' => Cell::Robot,
                            _ => panic!("unexpected `{ch}` in grid"),
                        }]
                    } else {
                        match ch {
                            '#' => vec![Cell::Wall, Cell::Wall],
                            '.' => vec![Cell::Empty, Cell::Empty],
                            'O' => vec![Cell::BoxLeft, Cell::BoxRight],
                            '@' => vec![Cell::Robot, Cell::Empty],
                            _ => panic!("unexpected `{ch}` in grid"),
                        }
                    }
                })
                .collect(),
        );
    }

    let moves = parts
        .next()
        .unwrap()
        .chars()
        .filter(|ch| *ch != '\n')
        .map(|ch| match ch {
            '<' => Move::L,
            '>' => Move::R,
            '^' => Move::U,
            'v' => Move::D,
            _ => panic!("unexpected `{ch}` in moves"),
        })
        .collect();

    // find the robot
    let robot = grid
        .iter_cells()
        .find(|c| *c.val == Cell::Robot)
        .map(|c| (c.row, c.col))
        .unwrap();
    Game { robot, grid, moves }
}

fn part1(mut input: Game) -> usize {
    input.apply_simple_moves();
    input.sum()
}

fn part2(mut input: Game) -> usize {
    input.apply_moves();
    println!("{}", input.grid);
    input.sum()
}

pub fn main() {
    let input = parse_input(false);
    println!("Part 1: {}", part1(input));
    let input = parse_input(true);
    println!("Part 2: {}", part2(input));
}
