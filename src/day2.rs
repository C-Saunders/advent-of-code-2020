use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

pub struct PasswordRecord {
    minimum_count: usize,
    maximum_count: usize,
    letter: char,
    password: String,
}

#[aoc_generator(day2)]
pub fn get_values(input: &str) -> Vec<PasswordRecord> {
    lazy_static! {
        static ref PARSING_EXPR: Regex = Regex::new(
            r"^(?P<minCount>\d+)-(?P<maxCount>\d+) (?P<letter>[a-z]): (?P<password>[a-z]+)$"
        )
        .unwrap();
    }
    input
        .lines()
        .map(|line| {
            let caps = PARSING_EXPR.captures(line).unwrap();
            PasswordRecord {
                minimum_count: caps["minCount"].parse::<usize>().unwrap(),
                maximum_count: caps["maxCount"].parse::<usize>().unwrap(),
                letter: caps["letter"].chars().next().unwrap(),
                password: caps["password"].to_string(),
            }
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn part1(inputs: &[PasswordRecord]) -> usize {
    inputs
        .iter()
        .filter(|record| {
            let count_of_letter = record
                .password
                .chars()
                .filter(|l| *l == record.letter)
                .collect::<Vec<char>>()
                .len();
            return count_of_letter >= record.minimum_count
                && count_of_letter <= record.maximum_count;
        })
        .collect::<Vec<&PasswordRecord>>()
        .len()
}

#[aoc(day2, part2)]
pub fn part2(inputs: &[PasswordRecord]) -> usize {
    inputs
        .iter()
        .filter(|record| {
            let first_index_is_letter = record
                .password
                .chars()
                .skip(record.minimum_count - 1)
                .next()
                .unwrap()
                == record.letter;
            let last_index_is_letter = record
                .password
                .chars()
                .skip(record.maximum_count - 1)
                .next()
                .unwrap()
                == record.letter;
            // ^ is xor
            return first_index_is_letter ^ last_index_is_letter;
        })
        .collect::<Vec<&PasswordRecord>>()
        .len()
}
