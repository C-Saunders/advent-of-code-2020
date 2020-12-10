use std::collections::VecDeque;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day9)]
pub fn get_values(input: &str) -> Vec<usize> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[aoc(day9, part1)]
pub fn part1(inputs: &[usize]) -> usize {
    let mut window = inputs.iter().take(25).collect::<VecDeque<_>>();

    for number in inputs.iter().skip(25) {
        if !is_valid(*number, &inputs) {
            return *number;
        }
        window.pop_front();
        window.push_back(number);
    }
    panic!("Did not find an invalid number")
}

#[aoc(day9, part2)]
pub fn part2(inputs: &[usize]) -> usize {
    let target_number = 90433990;

    for start_index in 0..inputs.len() {
        let mut current_sum = 0;
        let mut end_index = start_index;

        while current_sum < target_number {
            current_sum += inputs[end_index];

            if current_sum == target_number {
                return inputs[start_index..=end_index].iter().min().unwrap()
                    + inputs[start_index..=end_index].iter().max().unwrap();
            }

            end_index += 1;
        }
    }

    panic!("Failed to find a solution")
}

fn is_valid(number: usize, values: &[usize]) -> bool {
    for (i, first) in values.iter().enumerate() {
        for second in values.iter().skip(i) {
            if number == first + second {
                return true;
            }
        }
    }

    return false;
}
