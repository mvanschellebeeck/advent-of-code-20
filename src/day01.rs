use std::collections::HashMap;
use std::fs;

pub fn solve(part: u8) -> Option<i64> {
    match part {
        1 => part_1(),
        2 => part_2(),
        _ => None,
    }
}

fn part_1() -> Option<i64> {
    const DESIRED_SUM: i64 = 2020;
    let amounts = parse_file("input/day01_p1");
    let mut seen_amounts: HashMap<i64, i64> = HashMap::new();

    for amount in amounts {
        match seen_amounts.get(&amount) {
            Some(val) => return Some(amount * val),
            _ => seen_amounts.insert(DESIRED_SUM - amount, amount),
        };
    }
    None
}

fn part_2() -> Option<i64> {
    const DESIRED_SUM: i64 = 2020;
    let amounts = parse_file("input/day01_p2");
    let mut seen_pair_amounts: HashMap<i64, (i64, i64)> = HashMap::new();

    // store all possible pairs - O(n^2)
    for a1 in &amounts {
        for a2 in &amounts {
            seen_pair_amounts.insert(DESIRED_SUM - (a1 + a2), (*a1, *a2));
            // check if final value of three-pair sum can be found
            match seen_pair_amounts.get(&a2) {
                Some((p1, p2)) => return Some(a2 * p1 * p2),
                _ => continue,
            };
        }
    }
    None
}

fn parse_file(filename: &str) -> Vec<i64> {
    fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
}
