use regex::Regex;

#[derive(Debug, Clone, Copy)]
struct Game {
    a: Button,
    b: Button,
    prize: Prize,
}

// n_a * a.x + n_b * b.x = p.x
// n_a * a.y + n_b * b.y = p.y

// n_a * a.x + n_b * b.x = p.x
// n_a = (p.x - n_b * b.x) / a.x

// ((p.x - n_b * b.x) / a.x) * a.y + n_b * b.y = p.y
// (p.x - n_b * b.x) * a.y + n_b * b.y * a.x = p.y * a.x
// p.x - n_b * b.x + n_b * b.y * a.x / a.y = p.y * a.x / a.y
// p.x + n_b * b.y * a.x / a.y - n_b * b.x = p.y * a.x / a.y
// n_b * b.y * a.x / a.y - n_b * b.x = p.y * a.x / a.y - p.x
// n_b * (b.y * a.x / a.y - b.x) = p.y * a.x / a.y - p.x
// n_b = (p.y * a.x / a.y - p.x) / (b.y * a.x / a.y - b.x)

// n_a = (p.x - n_b * b.x) / a.x
// n_b = (p.y * a.x / a.y - p.x) / (b.y * a.x / a.y - b.x)

const EPSILON: f64 = 0.0001;

fn n_b(Game { a, b, prize: p }: Game) -> Option<usize> {
    let p0 = p.y as f64 * a.x as f64 / a.y as f64 - p.x as f64;
    let p1 = b.y as f64 * a.x as f64 / a.y as f64 - b.x as f64;

    let n_b = p0 / p1;
    let n_b_rounded = n_b.round();

    if (n_b - n_b_rounded).abs() < EPSILON {
        Some(n_b_rounded as usize)
    } else {
        None
    }
}

fn n_a(Game { a, b, prize: p }: Game, n_b: usize) -> Option<usize> {
    // n_a = (p.x - n_b * b.x) / a.x
    let a = (p.x as f64 - n_b as f64 * b.x as f64) / a.x as f64;
    let a_rounded = a.round();

    if (a - a_rounded).abs() < EPSILON {
        Some(a_rounded as usize)
    } else {
        None
    }
}

impl Game {
    fn cost(&self) -> Option<usize> {
        let n_b = n_b(*self)?;

        let n_a = n_a(*self, n_b)?;

        Some(cost(n_a, n_b))
    }
    fn min_cost(&self) -> Option<usize> {
        // find all values of (n_a, n_b) that satisfy the 2 equations:
        // n_a * a.x + n_b * b.x = p.x
        // n_a * a.y + n_b * b.y = p.y
        let mut best_cost = usize::MAX;
        let mut n_a = 0;
        loop {
            if n_a * self.a.x > self.prize.x || n_a * self.a.y > self.prize.y {
                break;
            }

            let rem_x = self.prize.x - n_a * self.a.x;
            let rem_y = self.prize.y - n_a * self.a.y;

            if rem_x % self.b.x == 0
                && rem_y % self.b.y == 0
                && rem_x / self.b.x == rem_y / self.b.y
            {
                let n_b = rem_x / self.b.x;

                let c = cost(n_a, n_b);

                if c < best_cost {
                    best_cost = c;
                }
            }

            n_a += 1;
        }
        // and find the (n_a, n_b) that minimizes the cost
        if best_cost < usize::MAX {
            Some(best_cost)
        } else {
            None
        }
    }
}

fn cost(n_a: usize, n_b: usize) -> usize {
    3 * n_a + n_b
}

#[derive(Debug, Clone, Copy)]
struct Button {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy)]
struct Prize {
    x: usize,
    y: usize,
}

fn parse_input() -> Vec<Game> {
    let input = include_str!("../inputs/13.txt");
    let button_regex = Regex::new(r#"Button .: X\+(\d+), Y\+(\d+)"#).unwrap();
    let prize_regex = Regex::new(r#"Prize: X=(\d+), Y=(\d+)"#).unwrap();
    let mut res = Vec::new();

    for block in input.split("\n\n") {
        let mut lines = block.lines();
        let line_a = lines.next().unwrap();
        let matches_a = button_regex.captures(line_a).unwrap();
        let a = Button {
            x: matches_a[1].parse().unwrap(),
            y: matches_a[2].parse().unwrap(),
        };
        let line_b = lines.next().unwrap();
        let matches_b = button_regex.captures(line_b).unwrap();
        let b = Button {
            x: matches_b[1].parse().unwrap(),
            y: matches_b[2].parse().unwrap(),
        };

        let line_prize = lines.next().unwrap();
        let matches = prize_regex.captures(line_prize).unwrap();
        let prize = Prize {
            x: matches[1].parse().unwrap(),
            y: matches[2].parse().unwrap(),
        };

        res.push(Game { a, b, prize });
    }

    res
}

fn part1(input: &[Game]) -> usize {
    input.iter().filter_map(|g| g.cost()).sum()
}

fn part2(mut input: Vec<Game>) -> usize {
    for g in input.iter_mut() {
        g.prize.x += 10000000000000;
        g.prize.y += 10000000000000;
    }
    input.iter().filter_map(|g| g.cost()).sum()
}

pub fn main() {
    let input = parse_input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_best_cost() {
        let game = Game {
            a: Button { x: 94, y: 34 },
            b: Button { x: 22, y: 67 },
            prize: Prize { x: 8400, y: 5400 },
        };

        assert_eq!(Some(280), game.min_cost());
    }

    #[test]
    fn test_formula() {
        let game = Game {
            a: Button { x: 94, y: 34 },
            b: Button { x: 22, y: 67 },
            prize: Prize { x: 8400, y: 5400 },
        };

        assert_eq!(Some(40), n_b(game));
        assert_eq!(Some(80), n_a(game, 40));

        let game = Game {
            a: Button { x: 26, y: 66 },
            b: Button { x: 67, y: 21 },
            prize: Prize { x: 12748, y: 12176 },
        };
        assert_eq!(None, n_b(game));
    }
}
