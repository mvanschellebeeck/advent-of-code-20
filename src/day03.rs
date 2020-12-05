use std::fs;

pub fn solve(part: u8) -> Option<i64> {
    match part {
        1 => Some(part_1(3, 1)),
        2 => Some(part_2()),
        _ => None,
    }
}

pub fn part_1(right: i64, down: i64) -> i64 {
    let input: Vec<Vec<char>> = fs::read_to_string("input/day03_p1")
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let height = input.len() as i64;
    let width = input.get(0).unwrap().len() as i64;

    let mut row = 0i64;
    let mut column = 0i64;
    let mut trees = 0i64;

    while (row + down) < height {
        column += right;
        row += down;
        let pos: &char = input
            .get(row as usize)
            .unwrap()
            .get((column % width) as usize)
            .unwrap();
        if *pos == '#' {
            trees += 1;
        }
    }
    trees
}

pub fn part_2() -> i64 {
    part_1(1, 1) * part_1(3, 1) * part_1(5, 1) * part_1(7, 1) * part_1(1, 2)
}
