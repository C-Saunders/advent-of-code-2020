use std::{cmp, collections::HashMap};

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct Space {
    pub row: isize,
    pub col: isize,
}

#[derive(Clone, Copy, PartialEq)]
enum SpaceVariant {
    Floor,
    Seat,
}

impl SpaceVariant {
    fn from_char(c: &char) -> Self {
        match c {
            '.' => SpaceVariant::Floor,
            'L' => SpaceVariant::Seat,
            _ => panic!("Unknown space char"),
        }
    }
}

#[derive(Clone)]
pub struct SpaceState {
    variant: SpaceVariant,
    occupied: bool,
}

pub struct SeatingArea {
    area: HashMap<Space, SpaceState>,
    num_rows: usize,
    num_cols: usize,
}

#[aoc_generator(day11)]
pub fn get_values(input: &str) -> SeatingArea {
    let mut area: HashMap<Space, SpaceState> = HashMap::new();

    for (row_num, line) in input.lines().enumerate() {
        for (col_num, ch) in line.chars().enumerate() {
            area.insert(
                Space {
                    row: row_num as isize,
                    col: col_num as isize,
                },
                SpaceState {
                    variant: SpaceVariant::from_char(&ch),
                    occupied: false,
                },
            );
        }
    }

    SeatingArea {
        area,
        num_rows: input.lines().collect::<Vec<&str>>().len(),
        num_cols: input
            .lines()
            .next()
            .unwrap()
            .chars()
            .collect::<Vec<char>>()
            .len(),
    }
}

#[aoc(day11, part1)]
pub fn part1(starting_setup: &SeatingArea) -> usize {
    let mut result = evaluate_round(&starting_setup, 4, get_num_adjacent_occupied);
    loop {
        result = evaluate_round(
            &SeatingArea {
                area: result.0,
                num_rows: starting_setup.num_rows,
                num_cols: starting_setup.num_cols,
            },
            4,
            get_num_adjacent_occupied,
        );
        if !result.1 {
            return result
                .0
                .values()
                .fold(0, |acc, v| acc + if v.occupied { 1 } else { 0 });
        }
    }
}

#[aoc(day11, part2)]
pub fn part2(starting_setup: &SeatingArea) -> usize {
    let mut result = evaluate_round(&starting_setup, 5, get_num_visible_occupied);
    loop {
        result = evaluate_round(
            &SeatingArea {
                area: result.0,
                num_rows: starting_setup.num_rows,
                num_cols: starting_setup.num_cols,
            },
            5,
            get_num_visible_occupied,
        );
        if !result.1 {
            return result
                .0
                .values()
                .fold(0, |acc, v| acc + if v.occupied { 1 } else { 0 });
        }
    }
}

fn evaluate_round(
    starting_setup: &SeatingArea,
    num_occupied_seats_triggers_vacating: u32,
    adjacency_evaluator: fn(&SeatingArea, isize, col: isize) -> u32,
) -> (HashMap<Space, SpaceState>, bool) {
    let SeatingArea {
        area,
        num_rows,
        num_cols,
    } = starting_setup;
    let mut updated_area = area.clone();
    let mut have_made_update = false;

    for row in 0..*num_rows as isize {
        for col in 0..*num_cols as isize {
            let current_space = area.get(&Space { row, col }).unwrap();
            if current_space.variant == SpaceVariant::Floor {
                continue;
            }

            let num_adjacent_occupied = adjacency_evaluator(&starting_setup, row, col);

            let updated_space = updated_area.entry(Space { row, col });

            if !current_space.occupied && num_adjacent_occupied == 0 {
                updated_space.and_modify(|e| {
                    e.occupied = true;
                });
                have_made_update = true;
            } else if current_space.occupied
                && num_adjacent_occupied >= num_occupied_seats_triggers_vacating
            {
                updated_space.and_modify(|e| {
                    e.occupied = false;
                });
                have_made_update = true;
            }
        }
    }

    (updated_area, have_made_update)
}

fn get_num_adjacent_occupied(seating_area: &SeatingArea, row: isize, col: isize) -> u32 {
    let SeatingArea {
        area,
        num_rows,
        num_cols,
    } = seating_area;
    let mut num_adjacent_occupied = 0;

    let row_lower = cmp::max(row - 1, 0);
    let row_upper = cmp::min(row + 1, *num_rows as isize - 1);
    let col_lower = cmp::max(col - 1, 0);
    let col_upper = cmp::min(col + 1, *num_cols as isize - 1);

    for adjacent_row in row_lower..=row_upper {
        for adjacent_col in col_lower..=col_upper {
            if adjacent_row == row && adjacent_col == col {
                continue;
            }
            let adjacent_space = area
                .get(&Space {
                    row: adjacent_row,
                    col: adjacent_col,
                })
                .unwrap();

            if adjacent_space.occupied {
                num_adjacent_occupied += 1;
            }
        }
    }

    num_adjacent_occupied
}

fn get_num_visible_occupied(seating_area: &SeatingArea, row: isize, col: isize) -> u32 {
    let SeatingArea {
        area,
        num_rows,
        num_cols,
    } = seating_area;
    let mut num_visible_occupied = 0;

    for row_increment in -1..=1 {
        for col_increment in -1..=1 {
            if row_increment == 0 && col_increment == 0 {
                continue;
            }

            let mut current_row = row;
            let mut current_col = col;

            loop {
                current_row += row_increment;
                current_col += col_increment;

                if current_row < 0
                    || current_row >= *num_rows as isize
                    || current_col < 0
                    || current_col >= *num_cols as isize
                {
                    break;
                }

                let looking_at = area
                    .get(&Space {
                        row: current_row,
                        col: current_col,
                    })
                    .unwrap();

                if looking_at.variant == SpaceVariant::Seat {
                    if looking_at.occupied {
                        num_visible_occupied += 1;
                    }
                    break;
                }
            }
        }
    }

    num_visible_occupied
}

#[cfg(test)]
mod part_1_tests {
    use super::*;

    #[test]
    fn single_seat() {
        let mut area = HashMap::new();
        area.insert(
            Space { row: 0, col: 0 },
            SpaceState {
                variant: SpaceVariant::Seat,
                occupied: false,
            },
        );

        let seating_area = SeatingArea {
            area,
            num_rows: 1,
            num_cols: 1,
        };

        assert_eq!(get_num_adjacent_occupied(&seating_area, 0, 0), 0);

        let (result, made_update) = evaluate_round(&seating_area, 4, get_num_adjacent_occupied);

        assert_eq!(made_update, true);
        assert_eq!(
            result.get(&Space { row: 0, col: 0 }).unwrap().occupied,
            true
        );

        let (result_2, made_update_2) = evaluate_round(
            &SeatingArea {
                area: result,
                num_rows: 1,
                num_cols: 1,
            },
            4,
            get_num_adjacent_occupied,
        );
        assert_eq!(made_update_2, false);
        assert_eq!(
            result_2.get(&Space { row: 0, col: 0 }).unwrap().occupied,
            true
        );
    }

    #[test]
    fn single_floor() {
        let mut area = HashMap::new();
        area.insert(
            Space { row: 0, col: 0 },
            SpaceState {
                variant: SpaceVariant::Floor,
                occupied: false,
            },
        );

        let (result, made_update) = evaluate_round(
            &SeatingArea {
                area,
                num_rows: 1,
                num_cols: 1,
            },
            4,
            get_num_adjacent_occupied,
        );

        assert_eq!(made_update, false);
        assert_eq!(
            result.get(&Space { row: 0, col: 0 }).unwrap().occupied,
            false
        );
    }

    #[test]
    fn four_seats() {
        let mut area = HashMap::new();
        for row in 0..=1 {
            for col in 0..=1 {
                area.insert(
                    Space { row, col },
                    SpaceState {
                        variant: SpaceVariant::Seat,
                        occupied: false,
                    },
                );
            }
        }
        let (result, made_update) = evaluate_round(
            &SeatingArea {
                area,
                num_rows: 2,
                num_cols: 2,
            },
            4,
            get_num_adjacent_occupied,
        );

        assert_eq!(made_update, true);
        for row in 0..=1 {
            for col in 0..1 {
                assert_eq!(result.get(&Space { row, col }).unwrap().occupied, true);

                assert_eq!(
                    get_num_adjacent_occupied(
                        &SeatingArea {
                            area: result.clone(),
                            num_rows: 2,
                            num_cols: 2,
                        },
                        row,
                        col
                    ),
                    3
                );
            }
        }
    }

    #[test]
    fn nine_seats() {
        let mut area = HashMap::new();
        for row in 0..=2 {
            for col in 0..=2 {
                area.insert(
                    Space { row, col },
                    SpaceState {
                        variant: SpaceVariant::Seat,
                        occupied: false,
                    },
                );
            }
        }
        let (result, made_update) = evaluate_round(
            &SeatingArea {
                area,
                num_rows: 3,
                num_cols: 3,
            },
            4,
            get_num_adjacent_occupied,
        );

        assert_eq!(made_update, true);
        for row in 0..=2 {
            for col in 0..2 {
                assert_eq!(result.get(&Space { row, col }).unwrap().occupied, true);
            }
        }

        // middle space
        assert_eq!(
            get_num_adjacent_occupied(
                &SeatingArea {
                    area: result.clone(),
                    num_rows: 3,
                    num_cols: 3,
                },
                1,
                1
            ),
            8
        );

        // bottom middle
        assert_eq!(
            get_num_adjacent_occupied(
                &SeatingArea {
                    area: result,
                    num_rows: 3,
                    num_cols: 3,
                },
                2,
                1
            ),
            5
        );
    }
}

#[cfg(test)]
mod part_2_tests {
    use super::*;

    #[test]
    fn single_seat() {
        let mut area = HashMap::new();
        area.insert(
            Space { row: 0, col: 0 },
            SpaceState {
                variant: SpaceVariant::Seat,
                occupied: false,
            },
        );

        let seating_area = SeatingArea {
            area,
            num_rows: 1,
            num_cols: 1,
        };

        assert_eq!(get_num_visible_occupied(&seating_area, 0, 0), 0);
    }

    #[test]
    fn larger_area() {
        let mut area = HashMap::new();
        for row in 0..=4 {
            for col in 0..=4 {
                area.insert(
                    Space { row, col },
                    SpaceState {
                        variant: if col == 0 {
                            SpaceVariant::Seat
                        } else {
                            SpaceVariant::Floor
                        },
                        occupied: false,
                    },
                );
            }
        }

        /*
        L . . . .
        L . . . .
        L . . . .
        L . # . .
        L . . . .
        */
        area.entry(Space { row: 3, col: 2 }).and_modify(|state| {
            state.variant = SpaceVariant::Seat;
            state.occupied = true;
        });

        let seating_area = SeatingArea {
            area,
            num_rows: 5,
            num_cols: 5,
        };

        assert_eq!(get_num_visible_occupied(&seating_area, 0, 0), 0, "0, 0");
        assert_eq!(get_num_visible_occupied(&seating_area, 1, 0), 1, "1, 0");
        assert_eq!(get_num_visible_occupied(&seating_area, 2, 0), 0, "2, 0");
        assert_eq!(get_num_visible_occupied(&seating_area, 3, 0), 1, "3, 0");
        assert_eq!(get_num_visible_occupied(&seating_area, 4, 0), 0, "4, 0");
    }
}
