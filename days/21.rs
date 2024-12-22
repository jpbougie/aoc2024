use std::{
    collections::HashMap,
    fmt::{Display, Write},
};

use grid::Grid;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Button {
    Num(u8),
    Up,
    Down,
    Left,
    Right,
    Activate,
    Empty,
}

impl Button {
    fn ap(&self, pos: (usize, usize)) -> (usize, usize) {
        match self {
            Button::Up => (pos.0 - 1, pos.1),
            Button::Down => (pos.0 + 1, pos.1),
            Button::Left => (pos.0, pos.1 - 1),
            Button::Right => (pos.0, pos.1 + 1),
            _ => pos,
        }
    }
}

impl Display for Button {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Button::Num(u) => write!(f, "{u}"),
            Button::Up => f.write_char('^'),
            Button::Down => f.write_char('v'),
            Button::Left => f.write_char('<'),
            Button::Right => f.write_char('>'),
            Button::Activate => f.write_char('A'),
            Button::Empty => f.write_char(' '),
        }
    }
}

#[derive(Debug, Clone)]
struct Keypad {
    pos: (usize, usize),
    grid: Grid<Button>,
}

fn keypads() -> Vec<Keypad> {
    vec![
        Keypad {
            grid: numeric(),
            pos: (3, 2),
        },
        Keypad {
            grid: directional(),
            pos: (0, 2),
        },
        Keypad {
            grid: directional(),
            pos: (0, 2),
        },
    ]
}

fn keypads_n(n: usize) -> Vec<Keypad> {
    let mut pads = Vec::with_capacity(1 + n);

    pads.push(Keypad {
        grid: numeric(),
        pos: (3, 2),
    });

    for _i in 0..n {
        pads.push(Keypad {
            grid: directional(),
            pos: (0, 2),
        });
    }
    pads
}

fn directional() -> Grid<Button> {
    let mut grid = Grid::new();
    grid.add_row(vec![Button::Empty, Button::Up, Button::Activate]);
    grid.add_row(vec![Button::Left, Button::Down, Button::Right]);

    grid
}

fn numeric() -> Grid<Button> {
    let mut grid = Grid::new();
    grid.add_row(vec![Button::Num(7), Button::Num(8), Button::Num(9)]);
    grid.add_row(vec![Button::Num(4), Button::Num(5), Button::Num(6)]);
    grid.add_row(vec![Button::Num(1), Button::Num(2), Button::Num(3)]);
    grid.add_row(vec![Button::Empty, Button::Num(0), Button::Activate]);

    grid
}

fn find_button(grid: &Grid<Button>, button: Button) -> Option<(usize, usize)> {
    grid.iter_cells()
        .find(|c| *c.val == button)
        .map(|c| (c.row, c.col))
}

fn moves_between(
    start: (usize, usize),
    goal: (usize, usize),
    avoid: (usize, usize),
) -> ((usize, Button), (usize, Button)) {
    let v_dist: isize = goal.0 as isize - start.0 as isize;
    let h_dist: isize = goal.1 as isize - start.1 as isize;

    let vertical = if v_dist < 0 {
        (v_dist.unsigned_abs(), Button::Up)
    } else {
        (v_dist as usize, Button::Down)
    };

    let horizontal = if h_dist < 0 {
        (h_dist.unsigned_abs(), Button::Left)
    } else {
        (h_dist as usize, Button::Right)
    };

    reorder(start, avoid, (vertical, horizontal))
}

fn reorder(
    start: (usize, usize),
    avoid: (usize, usize),
    moves: ((usize, Button), (usize, Button)),
) -> ((usize, Button), (usize, Button)) {
    // Test if taking the first button first goes
    if test_avoid(start, moves.0 .1, moves.0 .0, avoid)
        .is_none_or(|start| test_avoid(start, moves.1 .1, moves.1 .0, avoid).is_none())
    {
        return (moves.1, moves.0);
    }

    if test_avoid(start, moves.1 .1, moves.1 .0, avoid)
        .is_none_or(|start| test_avoid(start, moves.0 .1, moves.0 .0, avoid).is_none())
    {
        return moves;
    }

    // If left, prefer going left-right to up-down
    if moves.1 .1 == Button::Left {
        return (moves.1, moves.0);
    }

    // Otherwise, prefer going updown then left-right
    moves
}

fn test_avoid(
    start: (usize, usize),
    button: Button,
    times: usize,
    avoid: (usize, usize),
) -> Option<(usize, usize)> {
    let mut pos = start;
    for _i in 0..times {
        pos = button.ap(pos);
        if pos == avoid {
            return None;
        }
    }

    Some(pos)
}

fn part1(input: &Vec<Vec<Button>>) -> usize {
    let mut score = 0;

    for code in input {
        let mut keypads = keypads();
        let mut target = code.clone();
        for kp in keypads.iter_mut() {
            target = expand(target, kp);
        }
        println!("{}", target.len());
        score += target.len() * val(code);
    }

    score
}

fn part1_opt(input: &Vec<Vec<Button>>) -> usize {
    let mut score = 0;

    let mut cache = HashMap::new();
    for code in input {
        let keypads = keypads();
        let res = expand_count(code, 0, &keypads, &mut cache);
        println!("{res}");
        score += res * val(code);
    }

    score
}

fn part2(input: &Vec<Vec<Button>>) -> usize {
    let mut score = 0;

    let mut cache = HashMap::new();
    for code in input {
        let keypads = keypads_n(26);
        let res = expand_count(code, 0, &keypads, &mut cache);
        score += res * val(code);
    }

    score
}

type Pos = (usize, usize);
type MoveCache = HashMap<(Pos, Pos, Pos, usize), usize>;

fn expand_count(
    target: &[Button],
    depth: usize,
    keypads: &[Keypad],
    move_cache: &mut MoveCache,
) -> usize {
    if keypads.is_empty() {
        return target.len();
    }

    let keypad = &keypads[0];

    let mut sum = 0;
    let avoid = find_button(&keypad.grid, Button::Empty).unwrap();
    let mut pos = keypad.pos;
    for button in target.iter() {
        let mut moves = vec![];

        let goal = find_button(&keypad.grid, *button).unwrap();
        if let Some(cached) = move_cache.get(&(pos, goal, avoid, depth)) {
            sum += *cached;
        } else {
            let (mv_a, mv_b) = moves_between(pos, goal, avoid);
            for _i in 0..mv_a.0 {
                moves.push(mv_a.1);
            }
            for _i in 0..mv_b.0 {
                moves.push(mv_b.1);
            }

            moves.push(Button::Activate);

            let count = expand_count(&moves, depth + 1, &keypads[1..], move_cache);
            move_cache.insert((pos, goal, avoid, depth), count);

            sum += count;
        }
        pos = goal;
    }

    sum
}

fn expand(target: Vec<Button>, keypad: &mut Keypad) -> Vec<Button> {
    let mut moves = vec![];
    let init = keypad.pos;
    let avoid = find_button(&keypad.grid, Button::Empty).unwrap();
    for button in target {
        let goal = find_button(&keypad.grid, button).unwrap();
        let (mv_a, mv_b) = moves_between(keypad.pos, goal, avoid);
        for _i in 0..mv_a.0 {
            moves.push(mv_a.1);
        }
        for _i in 0..mv_b.0 {
            moves.push(mv_b.1);
        }

        moves.push(Button::Activate);
        keypad.pos = goal;
    }

    assert_eq!(init, keypad.pos);

    moves
}

fn parse_input() -> Vec<Vec<Button>> {
    let input = include_str!("../inputs/21.txt");

    input.lines().map(parse_buttons).collect()
}

fn val(buttons: &[Button]) -> usize {
    let mut score = 0;
    for b in buttons {
        if let Button::Num(n) = b {
            score *= 10;
            score += *n as usize;
        }
    }

    score
}

fn parse_buttons(input: &str) -> Vec<Button> {
    input
        .chars()
        .map(|ch| match ch {
            '0'..='9' => Button::Num(ch.to_digit(10).unwrap() as u8),
            'A' => Button::Activate,
            'v' => Button::Down,
            '^' => Button::Up,
            '<' => Button::Left,
            '>' => Button::Right,
            _ => panic!(),
        })
        .collect()
}

pub fn main() {
    let input = parse_input();
    println!("Part 1: {}", part1_opt(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_moves_between() {
        let grid = numeric();
        let start = (3, 2);
        let goal_0 = find_button(&grid, Button::Num(0)).unwrap();
        let avoid = find_button(&grid, Button::Empty).unwrap();

        assert_eq!(
            ((0, Button::Down), (1, Button::Left)),
            moves_between(start, goal_0, avoid)
        );

        let goal_2 = find_button(&grid, Button::Num(2)).unwrap();
        assert_eq!(
            ((1, Button::Up), (0, Button::Right)),
            moves_between(goal_0, goal_2, avoid)
        );

        let goal_2 = find_button(&grid, Button::Num(2)).unwrap();
        assert_eq!(
            ((1, Button::Up), (0, Button::Right)),
            moves_between(goal_0, goal_2, avoid)
        );

        let goal_9 = find_button(&grid, Button::Num(9)).unwrap();
        assert_eq!(
            ((2, Button::Up), (1, Button::Right)),
            moves_between(goal_2, goal_9, avoid)
        );

        assert_eq!(
            ((3, Button::Down), (0, Button::Right)),
            moves_between(goal_9, start, avoid)
        );
    }

    #[test]
    fn test_expand() {
        let mut keypad_a = Keypad {
            grid: numeric(),
            pos: (3, 2),
        };

        let target = expand(
            vec![
                Button::Num(0),
                Button::Num(2),
                Button::Num(9),
                Button::Activate,
            ],
            &mut keypad_a,
        );

        assert_eq!(parse_buttons("<A^A^^>AvvvA"), target);
    }
}
