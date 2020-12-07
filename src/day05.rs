use std::fs;
use std::str::Chars;

pub fn solve(part: u8) -> Option<i64> {
    match part {
        1 => part_1(),
        2 => part_2(),
        _ => None,
    }
}

fn binary_to_decimal(chars: Chars) -> i64 {
    chars
        .map(|c| match c {
            'R' | 'B' => 1,
            'F' | 'L' => 0,
            _ => panic!("Unexpected character: {}", c),
        })
        .rev()
        .enumerate()
        .fold(0, |s, (bit, result)| s + (result << bit))
}

fn get_seat_ids() -> Vec<i64> {
    let mut seat_ids: Vec<i64> = Vec::new();
    for line in fs::read_to_string("input/day05.txt").unwrap().lines() {
        let row = binary_to_decimal(line[..7].chars());
        let column = binary_to_decimal(line[7..10].chars());
        seat_ids.push((row * 8) + column)
    }
    seat_ids
}

fn part_1() -> Option<i64> {
    Some(*get_seat_ids().iter().max().unwrap())
}

fn part_2() -> Option<i64> {
    let mut seat_ids = get_seat_ids();
    seat_ids.sort();

    for i in 1..seat_ids.len() {
        if seat_ids[i] - 1 != seat_ids[i - 1] {
            return Some(seat_ids[i] - 1);
        }
    }
    Some(-1)
}
