use std::collections::{HashMap, HashSet};
use std::fs;
use std::iter::FromIterator;

pub fn solve(part: u8) -> Option<i64> {
    match part {
        1 => part_1(),
        2 => part_2(),
        _ => None,
    }
}

fn get_input() -> Vec<i64> {
    fs::read_to_string("input/day10.txt")
        .unwrap()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
}

fn part_1() -> Option<i64> {
    let mut input = get_input();
    input.insert(0, 0);
    input.push(*input.iter().max().unwrap() + 3);
    input.sort();

    let mut one_jolts = 0;
    let mut three_jolts = 0;
    for index in 1..input.len() {
        let diff = input.get(index).unwrap() - input.get(index - 1).unwrap();
        if diff == 1 {
            one_jolts += 1;
        } else if diff == 3 {
            three_jolts += 1;
        }
    }
    Some(one_jolts * three_jolts)
}

fn part_2() -> Option<i64> {
    let input = get_input();
    let mut adapters: HashSet<i64> = HashSet::from_iter(input.iter().cloned());
    let last_adapter = *adapters.iter().max().unwrap();
    adapters.insert(0);
    adapters.insert(last_adapter);

    let mut seen_adapters: HashMap<i64, i64> = HashMap::new();
    Some(count_ways_to_n(&adapters, &mut seen_adapters, last_adapter))
}

fn count_ways_to_n(adapters: &HashSet<i64>, memo: &mut HashMap<i64, i64>, n: i64) -> i64 {
    if let Some(val) = memo.get(&n) {
        return *val;
    }
    if n == 0 {
        return 1;
    }
    if !adapters.contains(&n) {
        return 0;
    }

    let ways_to_children = count_ways_to_n(adapters, memo, n - 1)
        + count_ways_to_n(&adapters, memo, n - 2)
        + count_ways_to_n(&adapters, memo, n - 3);

    memo.insert(n, ways_to_children);
    return ways_to_children;
}
