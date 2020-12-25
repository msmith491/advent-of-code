use std::env;
use std::str::Chars;

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
    let slope = get_input();
    let len = slope[0].len();
    let (mut i, mut j, mut trees) = (0, 0, 0);
    while i < slope.len() {
        // println!("{}, {}, {}", i, j, slope.len());
        match slope[i].chars().nth(j).unwrap() {
            '#' => trees += 1,
            _ => {}
        };
        i += 1;
        if j + 3 >= len {
            j = j + 3 - len
        } else {
            j += 3;
        }
    }
    println!("{}", trees);
}

fn part_two() {
    let slope = get_input();
    let result: usize = vec![
        helper(&slope, 1, 1),
        helper(&slope, 1, 3),
        helper(&slope, 1, 5),
        helper(&slope, 1, 7),
        helper(&slope, 2, 1),
    ]
    .iter()
    .product();
    println!("{}", result);
}

fn helper(slope: &Vec<&'static str>, dy: usize, dx: usize) -> usize {
    let len = slope[0].len();
    let (mut i, mut j, mut trees) = (0, 0, 0);
    while i < slope.len() {
        match slope[i].chars().nth(j).unwrap() {
            '#' => trees += 1,
            _ => {}
        };
        i += dy;
        if j + dx >= len {
            j = j + dx - len
        } else {
            j += dx;
        }
    }
    println!("{}", trees);
    trees
}

fn get_input() -> Vec<&'static str> {
    include_str!("../input.txt").lines().collect::<Vec<_>>()
}
