use std::fs;
use std::collections::HashMap;

fn main() {
    let result = day01_p1().unwrap_or_else(|| panic!("No solution found for day1 p1"));
    println!("{}", result);

    let result2 = day01_p2().unwrap_or_else(|| panic!("No solution found for day1 p2"));
    println!("{}", result2);
}


fn read_file(filename: &str) -> Vec<i32> {
    fs::read_to_string(filename).unwrap()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
}

fn day01_p1() -> Option<i32> {
    const DESIRED_SUM: i32 = 2020;
    let amounts = read_file("input/day01_p1");
    let mut seen_amounts: HashMap<i32, i32> = HashMap::new();

    for amount in amounts {
        if seen_amounts.contains_key(&amount) {
            return Some(amount * seen_amounts.get(&amount).unwrap());
        }
        seen_amounts.insert(DESIRED_SUM - amount, amount);
    }
    None
}

fn day01_p2() -> Option<i32> {
    const DESIRED_SUM: i32 = 2020;
    let amounts = read_file("input/day01_p2");
    let mut seen_pair_amounts: HashMap<i32, (i32, i32)> = HashMap::new();

    // store all possible pairs - On^2
    for a1 in &amounts {
        for a2 in &amounts {
            seen_pair_amounts.insert(DESIRED_SUM - (a1 + a2), (*a1, *a2));
        }
    }

    // iterate once more to find final value for three-pair sum - O(n)
    for a3 in amounts {
        if seen_pair_amounts.contains_key(&a3) {
            let (p1, p2) = seen_pair_amounts.get(&a3).unwrap();
            return Some(a3 * p1 * p2);
        }
    }
    None
}