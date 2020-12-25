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
    let data = get_input();
    let mut checked: HashSet<i64> = HashSet::new();
    for item in data {
        let tmp = 2020 - item;
        if checked.contains(&tmp) {
            println!("{}, {}, {}", item, tmp, tmp * item);
            break;
        }
        checked.insert(item);
    }
}

fn part_two() {
    let data = get_input();
    for item in &data {
        for orig in &data {
            for third in &data {
                if item + orig + third == 2020 {
                    println!(
                        "{}, {}, {}, {}, {}",
                        item,
                        orig,
                        third,
                        item + orig + third,
                        item * orig * third,
                    );
                    return;
                }
            }
        }
    }
}

fn get_input() -> Vec<i64> {
    let input = fs::read_to_string("input.txt").expect("File not found");
    input
        .split("\n")
        .filter(|&x| x != "")
        .map(|x| {
            let num: i64 = x.parse().unwrap();
            num
        })
        .collect::<Vec<_>>()
}
