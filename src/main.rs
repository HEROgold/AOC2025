#![deny(warnings)]

use std::ops::Range;

use crate::input::get_input;

mod input;


fn get_range(range: &str) -> Range<i128> {
    let bounds: Vec<&str> = range.split("-").collect();
    let start: i128 = bounds[0].trim_start_matches('0').parse().unwrap();
    let end: i128 = bounds[1].trim_start_matches('0').parse().unwrap();
    return start..end
}

fn is_invalid_id(n: i128) -> bool {
    let s = n.to_string();
    let len = s.len();

    // Try all possible pattern lengths that divide the total length
    for pat_len in 1..=len / 2 {
        if len % pat_len != 0 {
            continue;
        }

        let pattern = &s[..pat_len];
        if pattern
            .repeat(len / pat_len)
            == s
        {
            // Must be at least two repetitions
            if len / pat_len >= 2 {
                return true;
            }
        }
    }

    false
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
