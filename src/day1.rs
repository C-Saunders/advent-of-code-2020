use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;

#[aoc_generator(day1)]
pub fn get_values(input: &str) -> Result<Vec<i32>, ParseIntError> {
    input.lines().map(|l| l.parse::<i32>()).collect()
}

#[aoc(day1, part1)]
pub fn part1(inputs: &[i32]) -> i32 {
    let mut result: Option<i32> = None;
    for (index, i) in inputs.iter().enumerate() {
        for j in inputs.iter().skip(index + 1) {
            if i + j == 2020 {
                result = Some(i * j);
            }
        }
    }

    result.unwrap()
}

#[aoc(day1, part2)]
pub fn part2(inputs: &[i32]) -> i32 {
    let mut result: Option<i32> = None;
    for (i_index, i) in inputs.iter().enumerate() {
        for (j_index, j) in inputs.iter().skip(i_index + 1).enumerate() {
            for k in inputs.iter().skip(j_index) {
                if i + j + k == 2020 {
                    result = Some(i * j * k);
                }
            }
        }
    }

    result.unwrap()
}
