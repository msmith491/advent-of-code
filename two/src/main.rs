extern crate regex;

use regex::Regex;
use std::collections::HashSet;
use std::env;
use std::fs;

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
    let valid = get_input()
        .iter()
        .filter(|(low, high, letter, passwd)| {
            let count = passwd.matches(letter).count();
            count >= *low && count <= *high
        })
        .count();
    println!("{}", valid);
}

fn part_two() {
    let valid = get_input()
        .iter()
        .filter(|(first, second, letter, passwd)| {
            let letter = letter.chars().next().unwrap();
            let a = passwd.chars().nth(first - 1).unwrap_or('\0') == letter;
            let b = passwd.chars().nth(second - 1).unwrap_or('\0') == letter;
            let result = (a || b) && !(a && b);
            result
        })
        .count();
    println!("{}", valid);
}

fn get_input() -> Vec<(usize, usize, String, String)> {
    let input = fs::read_to_string("input.txt").expect("File not found");
    let input_re = Regex::new(r#"(\d+)-(\d+) (\w): (\w*)$"#).unwrap();
    // let input_re = Regex::new(r#"^(\d+)-(\d+) (.): (.+)$"#).unwrap();
    input
        .lines()
        .map(|x| {
            let cap = input_re.captures(x).unwrap();
            (
                (&cap[1]).parse().unwrap(),
                (&cap[2]).parse().unwrap(),
                (&cap[3]).to_owned(),
                (&cap[4]).to_owned(),
            )
        })
        .collect::<Vec<_>>()
}
