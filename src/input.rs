use std::env;

pub fn get_input(year: u32, day: u32) -> String {
    let session = env::var("AOC_SESSION").ok();
    return aoc_input_lib::get_puzzle_input(year, day, session).unwrap();
}
