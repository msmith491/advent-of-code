use std::collections::{HashMap, HashSet};
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
    let result: usize = get_input().iter().map(|&x| count_answers_any(x)).sum();
    println!("{}", result);
}

fn part_two() {
    let result: usize = get_input().iter().map(|&x| count_answers_all(x)).sum();
    println!("{}", result);
}

fn count_answers_any(answers: &str) -> usize {
    let mut found: HashSet<char> = HashSet::new();
    answers
        .chars()
        .filter(|x| x.is_ascii_alphabetic())
        .for_each(|x| {
            found.insert(x);
        });
    found.len()
}

fn count_answers_all(answers: &str) -> usize {
    let mut found: HashMap<char, usize> = HashMap::new();
    let len = answers.lines().count();
    answers
        .chars()
        .filter(|x| x.is_ascii_alphabetic())
        .for_each(|x| {
            *found.entry(x).or_insert(0) += 1;
        });
    found.iter().filter(|(_k, &v)| v == len).count()
}

fn get_input() -> Vec<&'static str> {
    include_str!("../input.txt")
        .split("\n\n")
        .collect::<Vec<_>>()
}
