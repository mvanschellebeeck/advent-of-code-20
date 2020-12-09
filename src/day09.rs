use std::collections::HashSet;
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
    fs::read_to_string("input/day09.txt")
        .unwrap()
        .lines()
        .map(|s| s.parse::<i64>().unwrap())
        .collect()
}

fn is_sum_of_two(window: &[i64], number: &i64, window_set: &HashSet<&i64>) -> bool {
    for prev_number in window {
        if window_set.contains(&(number - prev_number)) {
            return true;
        }
    }
    false
}

fn part_1() -> Option<i64> {
    let numbers = get_input();
    let mut window;
    let mut window_start = 0;
    let mut window_end = 25;
    // remove and add  1 number as the window slides
    let mut window_set = HashSet::from_iter(&numbers[window_start..window_end]);

    for index in 25..numbers.len() {
        window_set.insert(&numbers[window_end]);
        window = &numbers[window_start..window_end];
        let number = numbers.get(index).unwrap();
        if !is_sum_of_two(window, number, &window_set) {
            return Some(*number);
        }
        window_start += 1;
        window_end += 1;
        window_set.remove(&numbers[window_start]);
    }
    Some(-1)
}

fn part_2() -> Option<i64> {
    let numbers = get_input();
    let desired_sum = 104054607;

    let mut window_start = 0;
    let mut window_end = 1;
    let mut window;

    loop {
        window = &numbers[window_start..window_end];
        let sum: i64 = window.iter().sum();
        if sum < desired_sum {
            window_end += 1;
        } else if sum > desired_sum {
            window_start += 1;
        } else {
            return Some(window.iter().max().unwrap() + window.iter().min().unwrap());
        }
    }
}
