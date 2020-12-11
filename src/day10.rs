use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day10)]
pub fn get_values(input: &str) -> Vec<usize> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[aoc(day10, part1)]
pub fn part1(_: &[usize]) -> usize {
    // this was too easy with a spreadsheet to bother actually writing code
    1984
}

#[aoc(day10, part2)]
pub fn part2(raw: &[usize]) -> u64 {
    let mut copy = raw.iter().cloned().collect::<Vec<usize>>();
    let max = raw.iter().max().unwrap();
    copy.push(0);
    copy.push(*max);
    copy.sort();

    get_combinations_count(&copy, 0, &mut HashMap::new())
}

fn get_combinations_count(
    adapters: &[usize],
    start_index: usize,
    memo: &mut HashMap<usize, u64>,
) -> u64 {
    if start_index >= adapters.len() - 2 {
        return 1;
    }

    let current_value = adapters[start_index];
    if memo.get(&current_value).is_some() {
        return *memo.get(&current_value).unwrap();
    }

    let mut next_index = start_index + 1;

    let mut accumulator = 0;

    while adapters[next_index] <= current_value + 3 && next_index < adapters.len() - 1 {
        accumulator += get_combinations_count(&adapters, next_index, memo);
        next_index += 1;
    }

    memo.insert(current_value, accumulator);

    accumulator
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn len_two() {
        assert_eq!(get_combinations_count(&[0, 1], 0, &mut HashMap::new()), 1)
    }

    #[test]
    fn len_seven() {
        assert_eq!(
            get_combinations_count(&[0, 1, 4, 5, 6, 7, 10], 0, &mut HashMap::new()),
            4
        )
    }

    #[test]
    fn given_ex_small() {
        assert_eq!(
            get_combinations_count(
                &[0, 1, 4, 5, 6, 7, 10, 11, 12, 15, 16, 19, 22],
                0,
                &mut HashMap::new()
            ),
            8
        )
    }

    #[test]
    fn given_ex_large() {
        assert_eq!(
            get_combinations_count(
                &[
                    0, 1, 2, 3, 4, 7, 8, 9, 10, 11, 14, 17, 18, 19, 20, 23, 24, 25, 28, 31, 32, 33,
                    34, 35, 38, 39, 42, 45, 46, 47, 48, 49, 52
                ],
                0,
                &mut HashMap::new()
            ),
            19208
        )
    }
}
