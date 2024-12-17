use core::panic;

use regex::Regex;

type Word = u64;

#[derive(Debug, Clone, Default)]
struct Computer {
    program: Vec<Word>,
    ip: Word,

    // registers
    a: Word,
    b: Word,
    c: Word,
}

impl Computer {
    fn literal(&self) -> Word {
        self.program[self.ip as usize + 1]
    }

    fn combo(&self) -> Word {
        match self.program[self.ip as usize + 1] {
            n @ 0..=3 => n,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            x => panic!("unexpected combo operand: `{x}` at {}", self.ip + 1),
        }
    }

    fn run(&mut self) -> Vec<Word> {
        let mut output = Vec::new();

        while let Some(instr) = self.program.get(self.ip as usize) {
            match instr {
                0 => self.a /= 2u64.pow(self.combo() as u32),
                1 => self.b ^= &self.literal(),
                2 => self.b = self.combo() % 8,
                3 => {
                    if self.a != 0 {
                        self.ip = self.literal();
                        continue;
                    }
                }
                4 => self.b ^= self.c,
                5 => output.push(self.combo() % 8),
                6 => self.b = self.a / 2u64.pow(self.combo() as u32),
                7 => self.c = self.a / 2u64.pow(self.combo() as u32),
                _ => panic!("unexpected opcode {instr} at {}", self.ip),
            };

            self.ip += 2;
        }

        output
    }

    fn disasm_combo(combo: Word) -> &'static str {
        match combo {
            0 => "0",
            1 => "1",
            2 => "2",
            3 => "3",
            4 => "a",
            5 => "b",
            6 => "c",
            x => panic!(),
        }
    }
    fn disasm(&self) {
        for op in self.program.chunks(2) {
            match op[0] {
                0 => println!("a = a / (2^{});", Self::disasm_combo(op[1])),
                1 => println!("b = b ^ {};", op[1]),
                2 => println!("b = {} % 8;", Self::disasm_combo(op[1])),
                3 => println!("if a != 0 {{ jump {} }};", op[1]),
                4 => println!("b = b ^ c;"),
                5 => println!("out({} % 8)", Self::disasm_combo(op[1])),
                6 => println!("b = a / (2^{})", Self::disasm_combo(op[1])),
                7 => println!("c = a / (2^{})", Self::disasm_combo(op[1])),
                _ => panic!(),
            }
        }
    }
}

fn parse_input() -> Computer {
    let input = include_str!("../inputs/17.txt");
    let register_re = Regex::new(r#"Register .: (\d+)"#).unwrap();
    let program_re = Regex::new(r#"Program: ((\d+,)*\d+)"#).unwrap();

    let mut registers = register_re
        .captures_iter(input)
        .map(|caps| caps[1].parse::<Word>().unwrap());

    let program = program_re.captures(input).unwrap()[1]
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();

    Computer {
        program,
        ip: 0,
        a: registers.next().unwrap(),
        b: registers.next().unwrap(),
        c: registers.next().unwrap(),
    }
}

fn part1(mut input: Computer) -> String {
    input
        .run()
        .into_iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

fn part2(input: Computer) -> Word {
    println!("Searching for {:?}", input.program);
    let mut candidates: Vec<(Word, usize)> = Vec::new();
    candidates.push((0, input.program.len()));
    let mut results: Vec<Word> = Vec::new();

    while let Some((num, left)) = candidates.pop() {
        if left == 0 {
            results.push(num);
            continue;
        }
        for i in 0..8u64 {
            let mut input = input.clone();
            let num_to_test = num * 8 + i;
            input.a = num_to_test;
            let output = input.run();

            let haystack = input.program[left - 1];
            if *output.first().unwrap() == haystack {
                candidates.push((num_to_test, left - 1))
            }
        }
    }

    results.into_iter().min().unwrap()
}

pub fn main() {
    let input = parse_input();
    println!("Part 1: {}", part1(input.clone()));
    input.disasm();
    println!("Part 2: {}", part2(input.clone()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        let mut computer = Computer {
            program: vec![2, 6],
            ip: 0,
            a: 0,
            b: 0,
            c: 9,
        };
        computer.run();
        assert_eq!(1, computer.b);
    }

    #[test]
    fn b() {
        let mut computer = Computer {
            program: vec![5, 0, 5, 1, 5, 4],
            ip: 0,
            a: 10,
            b: 0,
            c: 0,
        };
        let output = computer.run();
        assert_eq!(vec![0, 1, 2], output);
    }
}
