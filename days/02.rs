fn parse_input() -> Vec<Vec<i64>> {
    let input = include_str!("../inputs/02.txt");
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|num| num.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect()
}

fn part1_safe(level: &&Vec<i64>) -> bool {
    let diffs = level
        .iter()
        .zip(level.iter().skip(1))
        .map(|(a, b)| b - a)
        .collect::<Vec<i64>>();

    diffs.iter().all(|d| *d >= 1 && *d <= 3) || diffs.iter().all(|d| *d >= -3 && *d <= -1)
}

fn part1(input: Vec<Vec<i64>>) -> usize {
    input.iter().filter(part1_safe).count()
}

fn part2_safe(level: &&Vec<i64>) -> bool {
    if part1_safe(level) {
        return true;
    }

    for i in 0..level.len() {
        let mut l = level.to_vec();
        l.remove(i);
        if part1_safe(&&l) {
            return true;
        }
    }

    false
}

fn part2(input: Vec<Vec<i64>>) -> usize {
    input.iter().filter(part2_safe).count()
}

pub fn main() {
    let input = parse_input();
    println!("Part1: {}", part1(input.clone()));
    println!("Part1: {}", part2(input.clone()));
}
