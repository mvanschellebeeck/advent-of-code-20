use std::collections::HashSet;
use std::fs;

pub fn solve(part: u8) -> Option<i64> {
    match part {
        1 => part_1(),
        2 => part_2(),
        _ => None,
    }
}

fn get_instructions() -> Vec<(String, i64)> {
    fs::read_to_string("input/day08.txt")
        .unwrap()
        .lines()
        .map(|s| {
            let mut splitter = s.split_whitespace();
            let instruction = splitter.next().unwrap().to_string();
            let value = splitter.next().unwrap().parse::<i64>().unwrap();
            (instruction, value)
        })
        .collect()
}

fn part_1() -> Option<i64> {
    let instructions = get_instructions();

    let mut acc = 0i64;
    let mut ptr = 0i64;
    let mut seen_ptrs: HashSet<i64> = HashSet::new();

    loop {
        let (instruction, offset) = &instructions[ptr as usize];
        match instruction.as_ref() {
            "acc" => {
                acc += offset;
                ptr = ptr + 1;
            }
            "jmp" => ptr = ptr + offset,
            "nop" => ptr = ptr + 1,
            _ => panic!("Unexpected instruction"),
        }
        if seen_ptrs.contains(&ptr) {
            break;
        }
        seen_ptrs.insert(ptr);
    }
    Some(acc)
}

fn does_execute(swap_index: usize, new_ins: String) -> (bool, i64) {
    let mut instructions = get_instructions();
    instructions[swap_index].0 = new_ins;

    let mut acc = 0i64;
    let mut ptr = 0i64;
    let mut seen_ptrs: HashSet<i64> = HashSet::new();

    loop {
        let (instruction, offset) = &instructions[ptr as usize];
        match instruction.as_ref() {
            "acc" => {
                acc += offset;
                ptr = ptr + 1;
            }
            "jmp" => ptr = ptr + offset,
            "nop" => ptr = ptr + 1,
            _ => panic!("Unexpected instruction"),
        }
        if seen_ptrs.contains(&ptr) {
            return (false, acc);
        }
        if ptr == instructions.len() as i64 {
            break;
        }
        seen_ptrs.insert(ptr);
    }
    (true, acc)
}

fn part_2() -> Option<i64> {
    let instructions = get_instructions();
    let indices_to_swap: Vec<(usize, String)> = instructions
        .iter()
        .enumerate()
        .map(|(index, pair)| match pair.0.as_ref() {
            "jmp" => (index, "nop".to_string()),
            "nop" => (index, "jmp".to_string()),
            _ => (index, pair.0.to_string()),
        })
        .collect();

    for (index, s) in indices_to_swap {
        let (success, acc) = does_execute(index, s);
        if success {
            return Some(acc);
        }
    }
    Some(-1)
}
