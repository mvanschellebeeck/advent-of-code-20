use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::str::FromStr;

fn main() {
    println!("Day 1 part 1: {}", day01_p1().unwrap());
    println!("Day 1 part 2: {}", day01_p2().unwrap());

    println!("Day 2 part 1: {}", day02_p1());
    println!("Day 2 part 2: {}", day02_p2());

    println!("Day 3 part 1: {}", day03_p1(3, 1));
    println!("Day 3 part 2: {}", day03_p2());

    println!("Day 4 part 1: {}", day04_p1());
    println!("Day 4 part 2: {}", day04_p2());
}

fn read_day01_file(filename: &str) -> Vec<i32> {
    fs::read_to_string(filename)
        .unwrap()
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
            _ => continue,
        };
    }
    None
}

fn day02_p1() -> u32 {
    let mut valid_passwords: u32 = 0;

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

fn day02_p2() -> u32 {
    let mut valid_passwords: u32 = 0;

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

fn day03_p1(right: u64, down: u64) -> u64 {
    let input: Vec<Vec<char>> = fs::read_to_string("input/day03_p1")
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let height = input.len() as u64;
    let width = input.get(0).unwrap().len() as u64;

    let mut row = 0u64;
    let mut column = 0u64;
    let mut trees = 0u64;

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

fn day03_p2() -> u64 {
    day03_p1(1, 1) * day03_p1(3, 1) * day03_p1(5, 1) * day03_p1(7, 1) * day03_p1(1, 2)
}

#[derive(Debug, PartialEq)]
enum HeightUnit {
    Cm,
    Inches,
}

impl FromStr for HeightUnit {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cm" => Ok(HeightUnit::Cm),
            "in" => Ok(HeightUnit::Inches),
            _ => Err(format!("'{}' is not a valid value for HeightUnit", s)),
        }
    }
}

#[derive(Debug)]
enum EyeColour {
    Amber,
    Blue,
    Brown,
    Green,
    Grey,
    Hazel,
    Other,
}

impl FromStr for EyeColour {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "amb" => Ok(EyeColour::Amber),
            "blu" => Ok(EyeColour::Blue),
            "brn" => Ok(EyeColour::Brown),
            "gry" => Ok(EyeColour::Grey),
            "grn" => Ok(EyeColour::Green),
            "hzl" => Ok(EyeColour::Hazel),
            "oth" => Ok(EyeColour::Other),
            _ => Err(format!("'{}' is not a valid value for EyeColour", s)),
        }
    }
}

fn get_capture<T: FromStr>(regex: &Regex, s: &str, n: usize) -> Option<T> {
    if let Some(captures) = regex.captures(&*s) {
        return Some(captures[n].parse::<T>().ok().unwrap());
    }
    None
}

fn validate_hgt(value: Option<u64>, unit: Option<HeightUnit>) -> bool {
    match unit {
        Some(HeightUnit::Cm) => value.map_or_else(|| false, |v| v >= 150 && v <= 193),
        Some(HeightUnit::Inches) => value.map_or_else(|| false, |v| v >= 59 && v <= 76),
        None => false,
    }
}

fn day04_p1() -> usize {
    fs::read_to_string("input/day04.txt")
        .unwrap()
        .split("\n\n")
        .filter(|s| {
            s.contains("byr:")
                && s.contains("iyr:")
                && s.contains("eyr:")
                && s.contains("hgt:")
                && s.contains("hcl:")
                && s.contains("ecl:")
                && s.contains("pid:")
        })
        .count()
}

fn day04_p2() -> u64 {
    let byr_re = Regex::new(r"byr:(\d{4})\b").unwrap();
    let iyr_re = Regex::new(r"iyr:(\d{4})\b").unwrap();
    let eyr_re = Regex::new(r"eyr:(\d{4})\b").unwrap();
    let hgt_re = Regex::new(r"hgt:(\d+)(cm|in)\b").unwrap();
    let hcl_re = Regex::new(r"hcl:(#[0-9a-f]{6})\b").unwrap();
    let ecl_re = Regex::new(r"ecl:(amb|blu|brn|gry|grn|hzl|oth)\b").unwrap();
    let pid_re = Regex::new(r"pid:([0-9]{9})\b").unwrap();
    let cid_re = Regex::new(r"cid:(\d+)\b").unwrap();

    let passports: Vec<String> = fs::read_to_string("input/day04.txt")
        .unwrap()
        .split("\n\n")
        .map(|s| s.to_string())
        .collect();

    let mut valid_passports = 0u64;
    for passport in &passports {
        let byr = get_capture::<u64>(&byr_re, passport, 1);
        let iyr = get_capture::<u64>(&iyr_re, passport, 1);
        let eyr = get_capture::<u64>(&eyr_re, passport, 1);
        let hgt_value = get_capture::<u64>(&hgt_re, passport, 1);
        let hgt_unit = get_capture::<HeightUnit>(&hgt_re, passport, 2);
        let hcl = get_capture::<String>(&hcl_re, passport, 1);
        let ecl = get_capture::<EyeColour>(&ecl_re, passport, 1);
        let pid = get_capture::<String>(&pid_re, passport, 1);
        let cid = get_capture::<u64>(&cid_re, passport, 1);

        if (byr.is_some() && (1920..=2003u64).contains(&byr.unwrap()))
            && (iyr.is_some() && (2010..=2020u64).contains(&iyr.unwrap()))
            && (eyr.is_some() && (2020..=2030u64).contains(&eyr.unwrap()))
            && validate_hgt(hgt_value, hgt_unit)
            && hcl.is_some()
            && ecl.is_some()
            && pid.is_some()
        {
            valid_passports += 1;
        }
    }
    valid_passports
}
