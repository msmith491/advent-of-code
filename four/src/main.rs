extern crate regex;
use regex::Regex;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Need either --part-one or --part-two as arg");
        return;
    }
    if args[1] == "--part-one" {
        part_one();
        return;
    }

    if args[1] == "--part-two" {
        part_two();
        return;
    }

    println!("Unknown args");
}

fn part_one() {
    let fields: Vec<&'static str> = vec!["byr:", "iyr:", "eyr:", "hgt:", "hcl:", "ecl:", "pid:"];

    let mut count = 0;
    let passports = get_input();
    for passport in passports {
        let mut valid = true;
        for field in &fields {
            if !passport.contains(field) {
                valid = false;
                break;
            }
        }
        if valid {
            count += 1;
        }
    }
    println!("{}", count);
}

fn part_two() {
    let byr = Regex::new(r#"byr:(\d{4})\b"#).unwrap();
    let iyr = Regex::new(r#"iyr:(\d{4})\b"#).unwrap();
    let eyr = Regex::new(r#"eyr:(\d{4})\b"#).unwrap();
    let hgt = Regex::new(r#"hgt:(\d\d\d?)(cm|in)\b"#).unwrap();
    let hcl = Regex::new(r#"hcl:#([0-9a-f]{6})\b"#).unwrap();
    let ecl = Regex::new(r#"ecl:(amb|blu|brn|gry|grn|hzl|oth)\b"#).unwrap();
    let pid = Regex::new(r#"pid:(\d{9})\b"#).unwrap();
    let passports = get_input();
    let mut count = 0;
    for passport in passports {
        match byr.captures(passport) {
            Some(m) => {
                let num = m[1].parse::<usize>().unwrap_or(0);
                if num < 1920 || num > 2002 {
                    continue;
                }
            }
            None => continue,
        };
        match iyr.captures(passport) {
            Some(m) => {
                let num = m[1].parse::<usize>().unwrap_or(0);
                if num < 2010 || num > 2020 {
                    continue;
                }
            }
            None => continue,
        };
        match eyr.captures(passport) {
            Some(m) => {
                let num = m[1].parse::<usize>().unwrap_or(0);
                if num < 2020 || num > 2030 {
                    continue;
                }
            }
            None => continue,
        };
        match hgt.captures(passport) {
            Some(m) => {
                let num = m[1].parse::<usize>().unwrap_or(0);
                let unit = &m[2];
                if unit == "in" && (num < 59 || num > 76) {
                    continue;
                } else if unit == "cm" && (num < 150 || num > 193) {
                    continue;
                }
            }
            None => continue,
        };
        match hcl.captures(passport) {
            Some(_) => {}
            None => continue,
        };
        match ecl.captures(passport) {
            Some(_) => {}
            None => continue,
        };
        match pid.captures(passport) {
            Some(_) => {}
            None => continue,
        };
        count += 1
    }
    println!("{}", count);
}

fn get_input() -> Vec<&'static str> {
    include_str!("../input.txt")
        .split("\n\n")
        .collect::<Vec<_>>()
}
