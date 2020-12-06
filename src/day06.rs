use std::collections::HashMap;
use std::fs;

pub fn solve(part: u8) -> Option<i64> {
    match part {
        1 => part_1(),
        2 => part_2(),
        _ => None,
    }
}

fn parse_input() -> Vec<String> {
    fs::read_to_string("input/day06.txt")
        .unwrap()
        .split("\n\n")
        .map(|s| s.to_string())
        .collect()
}

fn count_unique(s: String) -> usize {
    let mut s: Vec<char> = s.chars().collect();
    s.sort();
    s.dedup();
    s.len()
}

fn count_common_unique(groups: Vec<String>) -> usize {
    // collect seen characters into a map
    let map = groups
        .iter()
        .flat_map(|group| group.chars())
        .fold(HashMap::new(), |mut map, ch| {
            *map.entry(ch).or_insert(0) += 1;
            map
        });

    // count entries that appear in all groups
    map.iter()
        .filter(|(_, &count)| count == groups.len())
        .count()
}

fn part_1() -> Option<i64> {
    let answer: usize = parse_input()
        .iter()
        .map(|s| count_unique(s.replace("\n", "")))
        .sum();

    Some(answer as i64)
}

fn part_2() -> Option<i64> {
    let answer: usize = parse_input()
        .iter()
        .map(|s| count_common_unique(s.split("\n").map(String::from).collect()))
        .sum();

    Some(answer as i64)
}
