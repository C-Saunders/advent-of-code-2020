use std::cmp;

use aoc_runner_derive::{aoc, aoc_generator};

enum SearchStep {
    Lower,
    Upper,
}

impl SearchStep {
    fn from_char(input: &char) -> Self {
        match input {
            'F' | 'L' => SearchStep::Lower,
            'B' | 'R' => SearchStep::Upper,
            _ => panic!("Bad input char"),
        }
    }
}

pub struct Ticket {
    row_search_steps: Vec<SearchStep>,
    col_search_steps: Vec<SearchStep>,
}

impl Ticket {
    fn from_string(input: &str) -> Self {
        Ticket {
            row_search_steps: input[..7]
                .chars()
                .map(|l| SearchStep::from_char(&l))
                .collect(),
            col_search_steps: input[7..]
                .chars()
                .map(|l| SearchStep::from_char(&l))
                .collect(),
        }
    }
}

#[aoc_generator(day5)]
pub fn get_values(input: &str) -> Vec<Ticket> {
    input.lines().map(|l| Ticket::from_string(l)).collect()
}

#[aoc(day5, part1)]
pub fn part1(tickets: &[Ticket]) -> u32 {
    let mut highest_seen = 0;
    for ticket in tickets {
        let row = binary_search(127, &ticket.row_search_steps);
        let col = binary_search(7, &ticket.col_search_steps);
        let seat_id = row * 8 + col;

        highest_seen = cmp::max(highest_seen, seat_id);
    }

    highest_seen
}

#[aoc(day5, part2)]
pub fn part2(tickets: &[Ticket]) -> u32 {
    let mut seat_ids = vec![];
    for ticket in tickets {
        let row = binary_search(127, &ticket.row_search_steps);
        let col = binary_search(7, &ticket.col_search_steps);
        seat_ids.push(row * 8 + col);
    }

    seat_ids.sort();

    let mut iter = seat_ids.iter().peekable();
    loop {
        let curr = iter.next();
        let next = iter.peek();

        if next.is_none() {
            panic!("Failed to find!")
        }

        let curr_value = *curr.unwrap();
        let next_value = **next.unwrap();

        if next_value == curr_value + 2 {
            return curr_value + 1;
        }
    }
}

fn binary_search(upper_bound: u32, steps: &[SearchStep]) -> u32 {
    let mut current_lower_bound = 0;
    let mut current_upper_bound = upper_bound;
    for step in steps {
        let current_range_size = current_upper_bound - current_lower_bound + 1;
        if current_range_size % 2 != 0 {
            panic!("Range size must be even!");
        }
        let step_size = current_range_size / 2;

        match step {
            SearchStep::Lower => {
                current_upper_bound -= step_size;
            }
            SearchStep::Upper => {
                current_lower_bound += step_size;
            }
        }
    }

    if current_upper_bound != current_lower_bound {
        panic!("Finished steps but didn't finish search!");
    }

    current_lower_bound
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binary_search_small() {
        assert_eq!(
            binary_search(
                7,
                &[SearchStep::Upper, SearchStep::Lower, SearchStep::Upper]
            ),
            5
        );
    }

    #[test]
    fn binary_search_large() {
        assert_eq!(
            binary_search(
                127,
                &[
                    SearchStep::Lower,
                    SearchStep::Upper,
                    SearchStep::Lower,
                    SearchStep::Upper,
                    SearchStep::Upper,
                    SearchStep::Lower,
                    SearchStep::Lower
                ]
            ),
            44
        );
    }
}
