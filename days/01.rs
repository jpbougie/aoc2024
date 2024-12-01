use std::collections::HashMap;

fn parse_input() -> (Vec<i32>, Vec<i32>) {
    let input = include_str!("../inputs/01.txt");
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    input.lines().for_each(|line| {
        let mut num = line.split_ascii_whitespace();
        left.push(num.next().unwrap().parse().unwrap());
        right.push(num.next().unwrap().parse().unwrap());
    });

    (left, right)
}

fn part1(input: (Vec<i32>, Vec<i32>)) -> i32 {
    let mut left = input.0;
    left.sort();
    let mut right = input.1;
    right.sort();

    left.into_iter()
        .zip(right)
        .fold(0, |s, (a, b)| s + a.abs_diff(b) as i32)
}

fn part2(input: (Vec<i32>, Vec<i32>)) -> i32 {
    let freqs: HashMap<i32, i32> = input.1.into_iter().fold(HashMap::new(), |mut h, n| {
        let entry = h.entry(n).or_default();
        *entry += 1;
        h
    });

    input
        .0
        .into_iter()
        .fold(0i32, |s, n| s + n * *freqs.get(&n).unwrap_or(&0))
}

pub fn main() {
    let input = parse_input();
    println!("Part 1: {}", part1(input.clone()));
    println!("Part 2: {}", part2(input.clone()));
}
