use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
pub fn get_values(input: &str) -> Vec<String> {
    input.lines().map(|l| l.to_string()).collect()
}

#[aoc(day3, part1)]
pub fn part1(inputs: &[String]) -> usize {
    calculate_tree_count(&inputs, 1, 3)
}

#[aoc(day3, part2)]
pub fn part2(inputs: &[String]) -> usize {
    let down_one_right_one = calculate_tree_count(&inputs, 1, 1);
    let down_one_right_three = calculate_tree_count(&inputs, 1, 3);
    let down_one_right_five = calculate_tree_count(&inputs, 1, 5);
    let down_one_right_seven = calculate_tree_count(&inputs, 1, 7);
    let down_two_right_one = calculate_tree_count(&inputs, 2, 1);

    down_one_right_one
        * down_one_right_three
        * down_one_right_five
        * down_one_right_seven
        * down_two_right_one
}

fn calculate_tree_count(rows: &[String], row_increment: usize, col_increment: usize) -> usize {
    let mut tree_count = 0;
    let mut x_pos = 0;
    for row_num in (0..rows.len()).step_by(row_increment) {
        let as_chars: Vec<char> = rows[row_num].chars().collect();
        let current_char = as_chars[x_pos % rows[row_num].len()];
        if current_char == '#' {
            tree_count += 1;
        }
        x_pos += col_increment;
    }

    tree_count
}
