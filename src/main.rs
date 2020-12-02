use std::fs;
use std::collections::HashMap;

fn main() {
    let result = day01_p1().unwrap_or_else(|| panic!("No solution found for day1 p1"));
    println!("{}", result);

    let result2 = day01_p2().unwrap_or_else(|| panic!("No solution found for day1 p2"));
    println!("{}", result2);

    let result3 = day02_p1();
    println!("{}", result3);

    let result4 = day02_p2();
    println!("{}", result4);
}


fn read_day01_file(filename: &str) -> Vec<i32> {
    fs::read_to_string(filename).unwrap()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
}

fn day01_p1() -> Option<i32> {
    const DESIRED_SUM: i32 = 2020;
    let amounts = read_day01_file("input/day01_p1");
    let mut seen_amounts: HashMap<i32, i32> = HashMap::new();

    for amount in amounts {
        match seen_amounts.get(&amount) {
            Some(val) => return Some(amount * val),
            _ => seen_amounts.insert(DESIRED_SUM - amount, amount),
        };
    }
    None
}

fn day01_p2() -> Option<i32> {
    const DESIRED_SUM: i32 = 2020;
    let amounts = read_day01_file("input/day01_p2");
    let mut seen_pair_amounts: HashMap<i32, (i32, i32)> = HashMap::new();

    // store all possible pairs - On^2
    for a1 in &amounts {
        for a2 in &amounts {
            seen_pair_amounts.insert(DESIRED_SUM - (a1 + a2), (*a1, *a2));
        }
    }

    // iterate once more to find final value for three-pair sum - O(n)
    for a3 in amounts {
        match seen_pair_amounts.get(&a3) {
            Some((p1, p2)) => return Some(a3 * p1 * p2),
            _ => continue
        };
    }
    None
}


fn day02_p1() -> u32 {

    let mut valid_passwords: u32 = 0;

    for line in fs::read_to_string("input/day02_p1").unwrap().lines() {
        let mut splitter = line.split_whitespace();

        let policy_count_range: Vec<i32> = splitter.next().unwrap()
            .split("-").map(|i| i.parse().unwrap()).collect();
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

fn day02_p2() -> u32 {

    let mut valid_passwords: u32 = 0;

    for line in fs::read_to_string("input/day02_p1").unwrap().lines() {
        let mut splitter = line.split_whitespace();

        let policy_count_range: Vec<i32> = splitter.next().unwrap()
            .split("-").map(|i| i.parse().unwrap()).collect();
        let policy_count_first = *policy_count_range.get(0).unwrap() as usize;
        let policy_count_second = *policy_count_range.get(1).unwrap() as usize;

        let policy_letter = splitter.next().unwrap().chars().next().unwrap();
        let password = splitter.next().unwrap();

        let valid_chars : usize = password.chars().enumerate()
            .filter(|(_, ch)| *ch == policy_letter)
            .filter(|(index, _)| index + 1 == policy_count_first || index + 1 == policy_count_second)
            .count();

        if valid_chars == 1 {
            valid_passwords += 1;
        }
    }
    valid_passwords
}










