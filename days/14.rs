use std::{
    collections::HashSet,
    ops::{Add, AddAssign},
};

use regex::Regex;

const WIDTH: isize = 101;
const HEIGHT: isize = 103;
// const WIDTH: isize = 11;
// const HEIGHT: isize = 7;

#[derive(Debug, Clone, Copy)]
struct Bot {
    p: V,
    v: V,
}

impl Bot {
    fn adv(&mut self) {
        self.p += self.v;
        self.p.x = (WIDTH + self.p.x) % WIDTH;
        self.p.y = (HEIGHT + self.p.y) % HEIGHT;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct V {
    x: isize,
    y: isize,
}

impl Add for V {
    type Output = V;

    fn add(self, rhs: Self) -> Self::Output {
        V {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for V {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

fn parse_input() -> Vec<Bot> {
    let input = include_str!("../inputs/14.txt");
    let regex = Regex::new(r#"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)"#).unwrap();
    input
        .lines()
        .map(|l| {
            let caps = regex.captures(l).unwrap();
            Bot {
                p: V {
                    x: caps[1].parse().unwrap(),
                    y: caps[2].parse().unwrap(),
                },
                v: V {
                    x: caps[3].parse().unwrap(),
                    y: caps[4].parse().unwrap(),
                },
            }
        })
        .collect()
}

fn quadrants(bots: &[Bot]) -> usize {
    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;
    for bot in bots {
        match bot.p.x.cmp(&(WIDTH / 2)) {
            std::cmp::Ordering::Less => match bot.p.y.cmp(&(HEIGHT / 2)) {
                std::cmp::Ordering::Less => {
                    q1 += 1;
                }
                std::cmp::Ordering::Greater => {
                    q3 += 1;
                }
                std::cmp::Ordering::Equal => {}
            },
            std::cmp::Ordering::Greater => match bot.p.y.cmp(&(HEIGHT / 2)) {
                std::cmp::Ordering::Less => {
                    q2 += 1;
                }
                std::cmp::Ordering::Greater => {
                    q4 += 1;
                }
                std::cmp::Ordering::Equal => {}
            },
            std::cmp::Ordering::Equal => {}
        };
    }

    q1 * q2 * q3 * q4
}

fn part1(mut input: Vec<Bot>) -> usize {
    for _i in 0..100 {
        for bot in input.iter_mut() {
            bot.adv();
        }
    }

    quadrants(&input)
}

fn part2(mut input: Vec<Bot>) -> usize {
    let mut iters = 0;
    loop {
        if is_potentially_christmas_tree(&input) {
            display_grid(&input);
            break;
        }
        iters += 1;
        for bot in input.iter_mut() {
            bot.adv();
        }
    }
    iters
}

fn is_potentially_christmas_tree(bots: &[Bot]) -> bool {
    let pos: HashSet<V> = bots.iter().map(|b| b.p).collect();
    let mut grid = vec![vec![0usize; 9]; 9];
    // try to find a pattern that has a very high density
    const W: isize = WIDTH / 8;
    const H: isize = HEIGHT / 8;
    for pos in pos {
        grid[(pos.x / W) as usize][(pos.y / H) as usize] += 1;
    }
    // panic!();

    for row in grid {
        for col in row {
            if col > 40 {
                return true;
            }
        }
    }
    false
}

fn display_grid(bots: &[Bot]) {
    let pos: HashSet<V> = bots.iter().map(|b| b.p).collect();
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            if pos.contains(&V { x, y }) {
                print!("*");
            } else {
                print!(" ")
            }
        }
        println!();
    }
}

pub fn main() {
    let input = parse_input();
    println!("Part 1: {}", part1(input.clone()));
    println!("Part 2: {}", part2(input.clone()));
}
