#![deny(warnings)]

use std::ops::Range;

use crate::input::get_input;

mod input;


fn get_range(range: &str) -> Range<i128> {
    let bounds: Vec<&str> = range.split("-").collect();
    let start: i128 = bounds[0].trim_start_matches('0').parse().unwrap();
    let end: i128 = bounds[1].trim_start_matches('0').parse().unwrap();
    dbg!(start..end);
    return start..end
}

fn is_invalid_id(n: i128) -> bool {
    let s = n.to_string();
    if s.len() % 2 != 0 {
        return false;
    }
    let half = s.len() / 2;
    let (first, last) = s.split_at(half);
    first == last
}

fn sum_invalid_in_range(range: Range<i128>) -> i128 {
    range
        .filter(|&n| is_invalid_id(n))
        .sum()
}

fn find_invalid(range: Range<i128>) -> i128 {
    sum_invalid_in_range(range)
}

fn main() {
    let input = get_input(2025, 2);
    let mut total = 0;
    for line in input.lines() {
        for number in line.split(",") {
            let range = get_range(number);
            let invalid = find_invalid(range);
            total += invalid;
        }
    }
    print!("Total invalid numbers: {}", total);
}
