use std::fs;

pub fn solve(part: u8) -> Option<i64> {
    match part {
        1 => Some(part_1()),
        2 => Some(part_2()),
        _ => None,
    }
}

fn part_1() -> i64 {
    let mut valid_passwords: i64 = 0;

    for line in fs::read_to_string("input/day02_p1").unwrap().lines() {
        let mut splitter = line.split_whitespace();

        let policy_count_range: Vec<i32> = splitter
            .next()
            .unwrap()
            .split("-")
            .map(|i| i.parse().unwrap())
            .collect();
        let policy_count_start = *policy_count_range.get(0).unwrap() as usize;
        let policy_count_end = *policy_count_range.get(1).unwrap() as usize;

        let policy_letter = splitter.next().unwrap().chars().next().unwrap();
        let password = splitter.next().unwrap();

        let count = password.chars().filter(|&ch| ch == policy_letter).count();

        if count >= policy_count_start && count <= policy_count_end {
            valid_passwords += 1;
        }
    }
    valid_passwords
}

fn part_2() -> i64 {
    let mut valid_passwords: i64 = 0;

    for line in fs::read_to_string("input/day02_p1").unwrap().lines() {
        let mut splitter = line.split_whitespace();

        let policy_count_range: Vec<i32> = splitter
            .next()
            .unwrap()
            .split("-")
            .map(|i| i.parse().unwrap())
            .collect();
        let policy_count_first = *policy_count_range.get(0).unwrap() as usize;
        let policy_count_second = *policy_count_range.get(1).unwrap() as usize;

        let policy_letter = splitter.next().unwrap().chars().next().unwrap();
        let password = splitter.next().unwrap();

        let valid_chars: usize = password
            .chars()
            .enumerate()
            .filter(|(_, ch)| *ch == policy_letter)
            .filter(|(index, _)| {
                index + 1 == policy_count_first || index + 1 == policy_count_second
            })
            .count();

        if valid_chars == 1 {
            valid_passwords += 1;
        }
    }
    valid_passwords
}
