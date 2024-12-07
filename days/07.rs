#[derive(Debug, Clone)]
struct Equation {
    result: u64,
    nums: Vec<u64>,
}

impl Equation {
    fn has_solution_1(&self) -> bool {
        if self.nums.is_empty() {
            return false;
        }

        if self.nums.len() == 1 {
            return *self.nums.first().unwrap() == self.result;
        }

        let mut new = self.nums.clone();

        let a = new.pop().unwrap();
        let b = new.pop().unwrap();

        new.push(a + b);

        let e = Equation {
            result: self.result,
            nums: new.clone(),
        };
        if e.has_solution_1() {
            return true;
        }

        new.pop();
        new.push(a * b);

        let e = Equation {
            result: self.result,
            nums: new,
        };

        e.has_solution_1()
    }

    fn has_solution_2(&self) -> bool {
        if self.nums.is_empty() {
            return false;
        }

        if self.nums.len() == 1 {
            return *self.nums.first().unwrap() == self.result;
        }

        let mut new = self.nums.clone();

        let a = new.pop().unwrap();
        let b = new.pop().unwrap();

        new.push(a + b);

        let e = Equation {
            result: self.result,
            nums: new.clone(),
        };
        if e.has_solution_2() {
            return true;
        }

        new.pop();
        new.push(a * b);

        let e = Equation {
            result: self.result,
            nums: new.clone(),
        };

        if e.has_solution_2() {
            return true;
        }

        new.pop();
        new.push(concat(a, b));

        let e = Equation {
            result: self.result,
            nums: new,
        };

        e.has_solution_2()
    }
}

fn concat(mut a: u64, b: u64) -> u64 {
    let mut bb = b;
    while bb > 0 {
        a *= 10;
        bb /= 10;
    }

    a + b
}

fn parse_input() -> Vec<Equation> {
    let input = include_str!("../inputs/07.txt");
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(": ");
            Equation {
                result: parts.next().unwrap().parse().unwrap(),
                nums: parts
                    .next()
                    .unwrap()
                    .split(' ')
                    .map(|n| n.parse().unwrap())
                    .rev()
                    .collect(),
            }
        })
        .collect()
}

fn part1(input: &[Equation]) -> u64 {
    input.iter().fold(0u64, |sum, e| {
        if e.has_solution_1() {
            sum + e.result
        } else {
            sum
        }
    })
}

fn part2(input: &[Equation]) -> u64 {
    input.iter().fold(0u64, |sum, e| {
        if e.has_solution_2() {
            sum + e.result
        } else {
            sum
        }
    })
}

pub fn main() {
    let input = parse_input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
