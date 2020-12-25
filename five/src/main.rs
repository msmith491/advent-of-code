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
    let result = get_input().iter().map(|&x| seat_id(x)).max().unwrap();
    println!("{}", result);
}

fn part_two() {
    let mut result = get_input().iter().map(|&x| seat_id(x)).collect::<Vec<_>>();
    result.sort();
    for (i, row) in result.iter().enumerate() {
        if row + 1 != result[i + 1] {
            println!("{}", row + 1);
            break;
        }
    }
}

fn seat_id(id: &str) -> usize {
    let mut row = 0;
    let mut col = 0;
    for (i, letter) in id.chars().enumerate() {
        match letter {
            'B' => row += 128 / 2_usize.pow(i as u32 + 1),
            'R' => col += 8 / 2_usize.pow(i as u32 - 6),
            _ => {}
        }
    }
    row * 8 + col
}

fn get_input() -> Vec<&'static str> {
    include_str!("../input.txt").lines().collect::<Vec<_>>()
}
