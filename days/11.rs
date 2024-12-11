use std::collections::HashMap;

fn iter(freqs: HashMap<usize, usize>) -> HashMap<usize, usize> {
    let mut new = HashMap::with_capacity(freqs.capacity());

    for (num, count) in freqs.into_iter() {
        let digits = count_digits(num);
        match num {
            0 => inc(&mut new, 1, count),
            x if digits % 2 == 0 => {
                let div = 10usize.pow(digits as u32 / 2);
                inc(&mut new, x / div, count);
                inc(&mut new, x % div, count)
            }
            _ => inc(&mut new, num * 2024, count),
        };
    }

    new
}
#[inline]
fn inc(map: &mut HashMap<usize, usize>, k: usize, count: usize) {
    let entry = map.entry(k).or_default();
    *entry += count;
}

#[inline(always)]
fn count_digits(n: usize) -> usize {
    1 + n.abs_diff(0).checked_ilog10().unwrap_or_default() as usize
}

fn count(freqs: HashMap<usize, usize>) -> usize {
    freqs.values().sum()
}

fn part1(input: Vec<usize>) -> usize {
    let mut freqs: HashMap<usize, usize> = input.into_iter().map(|n| (n, 1)).collect();
    for _i in 0..25 {
        freqs = iter(freqs);
    }

    count(freqs)
}

fn part2(input: Vec<usize>) -> usize {
    let mut freqs: HashMap<usize, usize> = input.into_iter().map(|n| (n, 1)).collect();
    for _i in 0..75 {
        freqs = iter(freqs);
    }

    count(freqs)
}

fn parse_input() -> Vec<usize> {
    let input = include_str!("../inputs/11.txt");
    input
        .split_ascii_whitespace()
        .map(|n| n.parse().unwrap())
        .collect()
}

pub fn main() {
    let input = parse_input();
    println!("Part 1: {}", part1(input.clone()));
    println!("Part 2: {}", part2(input));
}
