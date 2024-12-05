use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug)]
struct Input {
    rules: Vec<(u32, u32)>,
    updates: Vec<Vec<u32>>,
}

fn parse_input() -> Input {
    let input = include_str!("../inputs/05.txt");
    let mut splits = input.split("\n\n");
    let mut rules = Vec::new();

    for line in splits.next().unwrap().lines() {
        let mut parts = line.split("|");
        rules.push((
            parts.next().unwrap().parse().unwrap(),
            parts.next().unwrap().parse().unwrap(),
        ));
    }

    let mut updates = Vec::new();
    for line in splits.next().unwrap().lines() {
        updates.push(
            line.split(",")
                .map(|page| page.parse())
                .collect::<Result<Vec<u32>, _>>()
                .unwrap(),
        );
    }

    Input { rules, updates }
}

fn respects_part1(update: &[u32], rules: &[(u32, u32)]) -> bool {
    let mut rules_h: HashMap<u32, HashSet<u32>> = HashMap::new();
    for (before, after) in rules {
        let entry = rules_h.entry(*after).or_default();
        entry.insert(*before);
    }

    for (i, num) in update.iter().enumerate() {
        let Some(must_be_before) = rules_h.get(num) else {
            continue;
        };

        for x in &update[i + 1..] {
            if must_be_before.contains(x) {
                return false;
            }
        }
    }

    true
}

fn part1(input: Input) -> u32 {
    input
        .updates
        .iter()
        .filter_map(|update| {
            if respects_part1(update, &input.rules) {
                Some(update[update.len() / 2])
            } else {
                None
            }
        })
        .sum()
}

fn part2(input: Input) -> u32 {
    let bad_updates = input
        .updates
        .iter()
        .filter(|update| !respects_part1(update, &input.rules));

    let mut sum = 0;
    let mut rules_h: HashMap<u32, HashSet<u32>> = HashMap::new();
    for (before, after) in input.rules.iter() {
        let entry = rules_h.entry(*after).or_default();
        entry.insert(*before);
    }
    'outer: for update in bad_updates {
        let mut new = Vec::new();
        let mut candidates = update.to_vec();

        while !candidates.is_empty() {
            if let Some(i) = (0..candidates.len()).find(|i| {
                rules_h.get(&candidates[*i]).is_none_or(|before| {
                    before
                        .iter()
                        .all(|bf| new.iter().any(|x| bf == x) || !candidates.contains(bf))
                })
            }) {
                let num = candidates.remove(i);
                if new.len() == update.len() / 2 {
                    sum += num;
                    continue 'outer;
                }
                new.push(num);
            } else {
                panic!("{:?}", candidates);
            }
        }
    }

    sum
}

pub fn main() {
    let input = parse_input();
    println!("Part 1: {}", part1(input.clone()));
    println!("Part 2: {}", part2(input.clone()));
}
