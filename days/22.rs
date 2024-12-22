use std::collections::HashMap;

#[inline]
fn mix(x: u64, y: u64) -> u64 {
    x ^ y
}

#[inline]
fn prune(x: u64) -> u64 {
    x % 16777216
}

fn gen(n: u64) -> u64 {
    let n2 = n * 64;
    let n = prune(mix(n, n2));

    let n2 = n / 32;
    let n = prune(mix(n, n2));

    let n2 = n * 2048;
    prune(mix(n, n2))
}

fn gen_n(n: u64, times: usize) -> u64 {
    let mut n = n;
    for _i in 0..times {
        n = gen(n);
    }

    n
}

fn gen_2000(mut n: u64) -> [u8; 2001] {
    let mut x = [0u8; 2001];
    x[0] = (n % 10) as u8;
    (1..=2000).for_each(|i| {
        n = gen(n);
        x[i] = (n % 10) as u8;
    });

    x
}

fn changes(a: [u8; 2001]) -> [i8; 2000] {
    let mut x = [0; 2000];
    for i in 0..2000 {
        x[i] = a[i + 1] as i8 - a[i] as i8;
    }

    x
}

fn part1(input: &[u64]) -> u64 {
    input.iter().map(|n| gen_n(*n, 2000)).sum()
}

fn part2(input: &[u64]) -> u64 {
    let mut scores: HashMap<[i8; 4], HashMap<u64, u8>> = HashMap::new();

    for input in input {
        let prices = gen_2000(*input);
        let changes = changes(prices);
        changes.windows(4).enumerate().for_each(|(i, window)| {
            let mut ch = [0i8; 4];
            ch.copy_from_slice(window);

            let entry = scores.entry(ch).or_default();
            entry.entry(*input).or_insert(prices[i + 4]);
        });
    }

    scores
        .values()
        .map(|score| score.values().map(|x| *x as u64).sum::<u64>())
        .max()
        .unwrap_or_default()
}

fn parse_input() -> Vec<u64> {
    let input = include_str!("../inputs/22.txt");
    input.lines().map(|l| l.parse().unwrap()).collect()
}

pub fn main() {
    let input = parse_input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let n = gen(123);
        assert_eq!(15887950, n);

        let n = gen(n);
        assert_eq!(16495136, n);
    }

    #[test]
    fn test_n() {
        assert_eq!(8685429, gen_n(1, 2000));
    }
}
