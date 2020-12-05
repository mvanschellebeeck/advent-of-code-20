use regex::Regex;
use std::fs;
use std::str::FromStr;

pub fn solve(part: u8) -> Option<i64> {
    match part {
        1 => Some(part_1()),
        2 => Some(part_2()),
        _ => None,
    }
}

fn part_1() -> i64 {
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
        .count() as i64
}

fn part_2() -> i64 {
    let byr_re = Regex::new(r"byr:(\d{4})\b").unwrap();
    let iyr_re = Regex::new(r"iyr:(\d{4})\b").unwrap();
    let eyr_re = Regex::new(r"eyr:(\d{4})\b").unwrap();
    let hgt_re = Regex::new(r"hgt:(\d+)(cm|in)\b").unwrap();
    let hcl_re = Regex::new(r"hcl:(#[0-9a-f]{6})\b").unwrap();
    let ecl_re = Regex::new(r"ecl:(amb|blu|brn|gry|grn|hzl|oth)\b").unwrap();
    let pid_re = Regex::new(r"pid:([0-9]{9})\b").unwrap();
    // let cid_re = Regex::new(r"cid:(\d+)\b").unwrap();

    let passports: Vec<String> = fs::read_to_string("input/day04.txt")
        .unwrap()
        .split("\n\n")
        .map(|s| s.to_string())
        .collect();

    let mut valid_passports = 0i64;
    for passport in &passports {
        let byr = get_capture::<u64>(&byr_re, passport, 1);
        let iyr = get_capture::<u64>(&iyr_re, passport, 1);
        let eyr = get_capture::<u64>(&eyr_re, passport, 1);
        let hgt_value = get_capture::<u64>(&hgt_re, passport, 1);
        let hgt_unit = get_capture::<String>(&hgt_re, passport, 2);
        let hcl = get_capture::<String>(&hcl_re, passport, 1);
        let ecl = get_capture::<String>(&ecl_re, passport, 1);
        let pid = get_capture::<String>(&pid_re, passport, 1);
        // let cid = get_capture::<u64>(&cid_re, passport, 1);

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

fn get_capture<T: FromStr>(regex: &Regex, s: &str, n: usize) -> Option<T> {
    if let Some(captures) = regex.captures(&*s) {
        return Some(captures[n].parse::<T>().ok().unwrap());
    }
    None
}

fn validate_hgt(value: Option<u64>, unit: Option<String>) -> bool {
    match unit.as_deref() {
        Some("cm") => value.map_or_else(|| false, |v| v >= 150 && v <= 193),
        Some("in") => value.map_or_else(|| false, |v| v >= 59 && v <= 76),
        _ => false,
    }
}
