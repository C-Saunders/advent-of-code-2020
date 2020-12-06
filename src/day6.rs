use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

#[aoc_generator(day6)]
pub fn get_values(input: &str) -> Vec<String> {
    lazy_static! {
        static ref SPLIT_EXPR: Regex = Regex::new(r"\n\n").unwrap();
    }
    SPLIT_EXPR.split(input).map(|l| l.to_string()).collect()
}

#[aoc(day6, part1)]
pub fn part1(answer_groups: &[String]) -> usize {
    let sets = answer_groups
        .iter()
        .map(|answer_group| {
            answer_group
                .chars()
                .filter(|c| *c != '\n')
                .collect::<HashSet<char>>()
        })
        .collect::<Vec<HashSet<char>>>();

    let mut total = 0;
    for answer_set in sets {
        total += answer_set.len();
    }

    total
}

#[aoc(day6, part2)]
pub fn part2(answer_groups: &[String]) -> usize {
    answer_groups
        .iter()
        .map(|group| {
            let mut lines = group.lines();
            let accumulator: HashSet<char> =
                lines.next().unwrap().chars().collect::<HashSet<char>>();
            // this might be nicer as: map to HashSet, then fold_first, but fold_first is nightly only
            lines.fold(accumulator, |acc, line| {
                let current = line.chars().collect::<HashSet<char>>();
                acc.intersection(&current).cloned().collect()
            })
        })
        .fold(0, |acc, set| acc + set.len())
}
