use regex::Regex;

#[derive(Clone, Debug)]
struct Mul {
    a: u64,
    b: u64,
}

impl Mul {
    fn total(&self) -> u64 {
        self.a * self.b
    }
}

#[derive(Clone, Debug)]
enum Instr {
    Mul(Mul),
    Do,
    Dont,
}

fn parse_input() -> Vec<Instr> {
    let re = Regex::new(r#"(?:(mul)\((\d{1,3}),(\d{1,3})\))|(do\(\))|(don't\(\))"#).unwrap();
    let input = include_str!("../inputs/03.txt");

    re.captures_iter(input)
        .map(|caps| match &caps[0] {
            "do()" => Instr::Do,
            "don't()" => Instr::Dont,
            _ if caps[0].starts_with("mul") => Instr::Mul(Mul {
                a: caps[2].parse::<u64>().unwrap(),
                b: caps[3].parse::<u64>().unwrap(),
            }),
            _ => panic!("unknown cap {}", &caps[0]),
        })
        .collect()
}

fn part1(input: Vec<Instr>) -> u64 {
    input
        .into_iter()
        .filter_map(|a| {
            if let Instr::Mul(mul) = a {
                Some(mul.total())
            } else {
                None
            }
        })
        .sum()
}

fn part2(input: Vec<Instr>) -> u64 {
    let mut enabled = true;
    let mut sum = 0;

    for instr in input {
        match instr {
            Instr::Mul(mul) => {
                if enabled {
                    sum += mul.total()
                }
            }
            Instr::Do => {
                enabled = true;
            }
            Instr::Dont => enabled = false,
        }
    }

    sum
}
pub fn main() {
    let input = parse_input();
    println!("Part 1: {}", part1(input.clone()));
    println!("Part 1: {}", part2(input.clone()));
}
