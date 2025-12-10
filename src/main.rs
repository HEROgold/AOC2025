#![deny(warnings)]

use std::{env, fmt::Debug};

#[derive(Debug)]
enum Direction {
    L, //Lower
    R, // Higher
}

fn rotate(mut current: i32, direction: &Direction, distance: i32) -> i32 {
    current = match direction {
        Direction::L => current - distance,
        Direction::R => current + distance,
    };

    return current % 100;
}

fn get_direction(char: char) -> &'static Direction {
    return match char {
        'L' => &Direction::L,
        'R' => &Direction::R,
        _ => panic!("Invalid direction"),
    };
}

fn as_i32(s: &str) -> i32 {
    s.trim().parse::<i32>().unwrap()
}

fn one_one(input: String) -> i32 {
    let mut current: i32 = 50;
    let mut direction: &Direction;
    let mut amount_zero: i32 = 0;
    for line in input.lines().into_iter() {
        let char = line.chars().next().unwrap();
        direction = get_direction(char);
        dbg!(char);
        dbg!(direction);

        let amount = line.get(1..).unwrap_or("0");
        let result = rotate(current, direction, as_i32(amount));
        current = result;
        dbg!(current);
        if current == 0 {
            amount_zero += 1;
        }
    }
    return amount_zero;
}

fn main() {
    let session = env::var("AOC_SESSION").ok();
    let res = aoc_input_lib::get_puzzle_input(2025, 1, session);
    let input = res.unwrap();
    println!("{}", one_one(input));
}

// answer format: level=1&answer=997
// to url: https://adventofcode.com/2025/day/1/answer
