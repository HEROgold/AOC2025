#![deny(warnings)]

use std::{env, fmt::Debug};

#[derive(Debug)]
enum Direction {
    L, //Lower
    R, // Higher
}

#[allow(dead_code)]
fn rotate(mut current: i32, direction: &Direction, distance: i32) -> i32 {
    current = match direction {
        Direction::L => current - distance,
        Direction::R => current + distance,
    };

    return current % 100;
}

struct RotateResult {
    current: i32,
    total_zeros: i32,
}

fn rotate_two(mut current: i32, direction: &Direction, distance: i32) -> RotateResult {
    let mut total_zeros = 0;
    match direction {
        Direction::L => {
            for _ in 0..distance {
                current -= 1;
                check_and_increment_zero(current, &mut total_zeros);
            }
        },
        Direction::R => {
            for _ in 0..distance {
                current += 1;
                check_and_increment_zero(current, &mut total_zeros);
            }
        },
    };
    return RotateResult { current, total_zeros };
}

fn check_and_increment_zero(current: i32, total_zeros: &mut i32) {
    if current % 100 == 0 {
        *total_zeros += 1;
    }
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
        let (result, zeros) = {
            let res = rotate_two(current, direction, as_i32(amount));
            (res.current, res.total_zeros)
        };
        amount_zero += zeros;
        current = result;
        dbg!(current);
        dbg!(amount_zero);
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
