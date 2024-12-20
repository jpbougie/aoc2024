use std::collections::HashMap;

use regex::Regex;

struct Input {
    patterns: Vec<String>,
    towels: Vec<String>,
}

fn parse_input() -> Input {
    let input = include_str!("../inputs/19.txt");
    let mut parts = input.split("\n\n");
    let patterns = parts
        .next()
        .unwrap()
        .split(", ")
        .map(|x| x.to_string())
        .collect();
    let towels = parts
        .next()
        .unwrap()
        .lines()
        .map(|x| x.to_string())
        .collect();

    Input { patterns, towels }
}

fn part1(input: &Input) -> usize {
    let mut pats = input.patterns.clone();
    for pat in pats.iter_mut() {
        *pat = format!("({pat})");
    }
    let re_str = format!("^({})+$", pats.join("|"));
    let re = Regex::new(&re_str).unwrap();
    input.towels.iter().filter(|t| re.is_match(t)).count()
}

fn waysto(input: &str, patterns: &Vec<String>, cache: &mut HashMap<String, usize>) -> usize {
    if input.is_empty() {
        return 1;
    }

    if let Some(cached) = cache.get(input) {
        return *cached;
    }

    let mut matches = 0;
    for pattern in patterns {
        if let Some(rest) = input.strip_prefix(pattern) {
            matches += waysto(rest, patterns, cache);
        }
    }

    cache.insert(input.to_string(), matches);
    matches
}

fn part2(input: &Input) -> usize {
    let mut count = 0;

    let mut pats = input.patterns.clone();
    pats.sort_by_key(|f| f.len());

    let mut cache: HashMap<String, usize> = HashMap::new();

    for towel in input.towels.iter() {
        println!("Checking {towel}");
        count += waysto(towel, &input.patterns, &mut cache);
    }

    count
}

pub fn main() {
    let input = parse_input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
