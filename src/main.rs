#![feature(test)]
extern crate test;

mod day1;
mod day2;
mod day3;

use std::env;
use std::io::Write;
use std::str::FromStr;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() < 2 {
        handle_invalid_input(b"Usage: aoc [day] [part]");
    }

    let day = u64::from_str(&args[0]).expect("error parsing argument");
    let part = u64::from_str(&args[1]).expect("error parsing argument");

    let result = exercise_picker(day, part);

    match result {
        Some(output) => println!("Exercise day {} result: {}", day, output),
        None => println!("Exercise day {} part: {} failed", day, part),
    }
}

fn exercise_picker(day: u64, part: u64) -> Option<String> {
    return match (day, part) {
        (1, 1) => day1::part1(),
        (1, 2) => day1::part2(),
        (2, 1) => day2::part1(),
        (2, 2) => day2::part2(),
        (3, 1) => day3::part1(),
        (3, 2) => day3::part2(),
        _ => panic!("invalid day, part input"),
    };
}

fn handle_invalid_input(error_message: &[u8]) {
    std::io::stderr().write(error_message).unwrap();
    std::process::exit(1);
}
