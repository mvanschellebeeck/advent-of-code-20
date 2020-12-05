use std::fs;
use std::str::Chars;

pub fn solve(part: u8) -> Option<i64> {
    match part {
        1 => part_1(),
        2 => part_2(),
        _ => None,
    }
}

fn mid(line: &mut Chars, lower: u8, upper: u8) -> u8 {
    let mut lower = lower as f64;
    let mut upper = upper as f64;
    while lower != upper {
        let ch = line.next().unwrap();
        let mid = (lower + upper) / 2f64;
        match ch {
            'F' => upper = mid.floor(),
            'B' => lower = mid.ceil(),
            'L' => upper = mid.floor(),
            'R' => lower = mid.ceil(),
            _ => panic!("{} isn't a bound identifier", ch),
        }
    }
    lower as u8
}

fn get_seat_ids() -> Vec<i64> {
    let mut seat_ids: Vec<i64> = Vec::new();
    for line in fs::read_to_string("input/day05.txt").unwrap().lines() {
        let row = mid(&mut line[..7].chars(), 0, 127) as f64;
        let column = mid(&mut line[7..10].chars(), 0, 7) as f64;
        seat_ids.push(((row * 8f64) + column) as i64);
    }
    seat_ids
}

fn part_1() -> Option<i64> {
    let seat_ids = get_seat_ids();
    Some(*seat_ids.iter().max().unwrap())
}

fn part_2() -> Option<i64> {
    let mut seat_ids = get_seat_ids();
    seat_ids.sort();

    for pair in seat_ids.chunks(2) {
        if pair[1] - pair[0] == 2 {
            return Some(pair[0] + 1);
        }
    }
    Some(-1)
}
