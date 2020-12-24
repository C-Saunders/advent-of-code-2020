use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day13)]
pub fn get_values(input: &str) -> (u32, String) {
    let mut lines = input.lines();
    let earliest_departure = lines.next().unwrap().parse::<u32>().unwrap();

    (earliest_departure, lines.next().unwrap().to_string())
}

#[aoc(day13, part1)]
pub fn part1((earliest_departure, bus_lines_raw): &(u32, String)) -> u32 {
    let bus_lines = bus_lines_raw
        .split(',')
        .filter_map(|v| v.parse::<u32>().ok())
        .collect::<Vec<u32>>();

    let result = bus_lines
        .iter()
        .map(|bus_time| {
            let closest_departure_time =
                ((*earliest_departure as f32 / *bus_time as f32).floor() as u32 + 1) * bus_time;
            let wait_time = closest_departure_time - earliest_departure;

            (bus_time, wait_time)
        })
        .min_by_key(|v| v.1)
        .unwrap();

    result.0 * result.1
}

#[derive(Clone, Copy, Debug)]
pub struct BusSchedule {
    offset: usize,
    time: u64,
    matched: bool,
}

#[aoc(day13, part2)]
pub fn part2((_, bus_lines_raw): &(u32, String)) -> u64 {
    let schedules = bus_lines_raw
        .split(',')
        .enumerate()
        .filter_map(|(index, bus_time_or_x)| {
            if bus_time_or_x == "x" {
                None
            } else {
                Some(BusSchedule {
                    offset: index,
                    time: bus_time_or_x.parse::<u64>().unwrap(),
                    matched: false,
                })
            }
        })
        .collect::<Vec<BusSchedule>>();

    find_solution(&schedules)
}

fn find_solution(schedules_input: &[BusSchedule]) -> u64 {
    let mut schedules = schedules_input.to_owned().clone();
    schedules[0].matched = true;
    let mut step_size = schedules[0].time;
    let mut current_time = step_size as u64;

    loop {
        for mut schedule in schedules.iter_mut() {
            if !schedule.matched
                && (current_time + schedule.offset as u64) % schedule.time as u64 == 0
            {
                schedule.matched = true;
                step_size = step_size * schedule.time;
            }
        }

        if schedules.iter().all(|s| s.matched) {
            return current_time;
        }

        current_time += step_size;
    }
}

#[cfg(test)]
mod part2_tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            find_solution(&[
                BusSchedule {
                    offset: 0,
                    time: 17,
                    matched: false,
                },
                BusSchedule {
                    offset: 2,
                    time: 13,
                    matched: false,
                },
                BusSchedule {
                    offset: 3,
                    time: 19,
                    matched: false,
                }
            ]),
            3417
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            find_solution(&[
                BusSchedule {
                    offset: 0,
                    time: 67,
                    matched: false,
                },
                BusSchedule {
                    offset: 1,
                    time: 7,
                    matched: false,
                },
                BusSchedule {
                    offset: 2,
                    time: 59,
                    matched: false,
                },
                BusSchedule {
                    offset: 3,
                    time: 61,
                    matched: false,
                }
            ]),
            754018
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            find_solution(&[
                BusSchedule {
                    offset: 0,
                    time: 67,
                    matched: false,
                },
                BusSchedule {
                    offset: 2,
                    time: 7,
                    matched: false,
                },
                BusSchedule {
                    offset: 3,
                    time: 59,
                    matched: false,
                },
                BusSchedule {
                    offset: 4,
                    time: 61,
                    matched: false,
                }
            ]),
            779210
        );
    }

    #[test]
    fn example4() {
        assert_eq!(
            find_solution(&[
                BusSchedule {
                    offset: 0,
                    time: 67,
                    matched: false,
                },
                BusSchedule {
                    offset: 1,
                    time: 7,
                    matched: false,
                },
                BusSchedule {
                    offset: 3,
                    time: 59,
                    matched: false,
                },
                BusSchedule {
                    offset: 4,
                    time: 61,
                    matched: false,
                }
            ]),
            1261476
        );
    }

    #[test]
    fn example5() {
        assert_eq!(
            find_solution(&[
                BusSchedule {
                    offset: 0,
                    time: 1789,
                    matched: false,
                },
                BusSchedule {
                    offset: 1,
                    time: 37,
                    matched: false,
                },
                BusSchedule {
                    offset: 2,
                    time: 47,
                    matched: false,
                },
                BusSchedule {
                    offset: 3,
                    time: 1889,
                    matched: false,
                }
            ]),
            1202161486
        );
    }
}
