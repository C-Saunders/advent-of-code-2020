use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

enum Action {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}

impl Action {
    fn from_str(value: &str) -> Action {
        match value {
            "N" => Action::North,
            "S" => Action::South,
            "E" => Action::East,
            "W" => Action::West,
            "L" => Action::Left,
            "R" => Action::Right,
            "F" => Action::Forward,
            _ => panic!("Invalid action"),
        }
    }
}

enum CardinalDirection {
    East,
    North,
    West,
    South,
}

impl CardinalDirection {
    fn from_degrees(degrees: &isize) -> CardinalDirection {
        let mut normalized_degrees = degrees % 360;
        if normalized_degrees < 0 {
            normalized_degrees += 360;
        }
        match normalized_degrees {
            0 => CardinalDirection::East,
            90 => CardinalDirection::North,
            180 => CardinalDirection::West,
            270 => CardinalDirection::South,
            _ => panic!("Invalid degrees for cardinal direction {}.", { degrees }),
        }
    }
}

#[derive(Debug)]
pub struct Position {
    north_south: isize,
    east_west: isize,
}

pub struct Ship {
    position: Position,
    facing_deg: isize,
}

pub struct Instruction {
    action: Action,
    amount: isize,
}

impl Instruction {
    fn from_str(value: &str) -> Instruction {
        lazy_static! {
            static ref SPLIT_EXPR: Regex =
                Regex::new(r"^(?P<action>[NSEWLRF])(?P<amount>\d+$)").unwrap();
        }
        let parts = SPLIT_EXPR.captures(value).unwrap();
        Instruction {
            action: Action::from_str(&parts["action"]),
            amount: parts["amount"].parse::<isize>().unwrap(),
        }
    }
}

#[aoc_generator(day12)]
pub fn get_values(input: &str) -> Vec<Instruction> {
    input.lines().map(|v| Instruction::from_str(v)).collect()
}

#[aoc(day12, part1)]
pub fn part1(inputs: &[Instruction]) -> isize {
    let mut ship = Ship {
        facing_deg: 0,
        position: Position {
            north_south: 0,
            east_west: 0,
        },
    };

    for instruction in inputs {
        let Instruction { action, amount } = instruction;
        match action {
            Action::North => ship.position.north_south += amount,
            Action::South => ship.position.north_south -= amount,
            Action::East => ship.position.east_west += amount,
            Action::West => ship.position.east_west -= amount,
            Action::Left => ship.facing_deg += amount,
            Action::Right => ship.facing_deg -= amount,
            Action::Forward => match CardinalDirection::from_degrees(&ship.facing_deg) {
                CardinalDirection::North => ship.position.north_south += amount,
                CardinalDirection::South => ship.position.north_south -= amount,
                CardinalDirection::East => ship.position.east_west += amount,
                CardinalDirection::West => ship.position.east_west -= amount,
            },
        }
    }

    ship.position.north_south.abs() + ship.position.east_west.abs()
}

#[aoc(day12, part2)]
pub fn part2(inputs: &[Instruction]) -> isize {
    let mut ship = Position {
        north_south: 0,
        east_west: 0,
    };
    let mut waypoint = Position {
        north_south: 1,
        east_west: 10,
    };

    for instruction in inputs {
        let Instruction { action, amount } = instruction;
        match action {
            Action::North => waypoint.north_south += amount,
            Action::South => waypoint.north_south -= amount,
            Action::East => waypoint.east_west += amount,
            Action::West => waypoint.east_west -= amount,
            Action::Left => {
                let old_east_west = waypoint.east_west as f32;
                let (mut sin, mut cos) = (*amount as f32).to_radians().sin_cos();
                sin = sin.round();
                cos = cos.round();

                waypoint.east_west =
                    (old_east_west * cos - waypoint.north_south as f32 * sin) as isize;
                waypoint.north_south =
                    (old_east_west * sin + waypoint.north_south as f32 * cos) as isize;
            }
            Action::Right => {
                let old_east_west = waypoint.east_west as f32;
                let (mut sin, mut cos) = ((-1 * *amount) as f32).to_radians().sin_cos();
                sin = sin.round();
                cos = cos.round();

                waypoint.east_west =
                    (old_east_west * cos - waypoint.north_south as f32 * sin) as isize;
                waypoint.north_south =
                    (old_east_west * sin + waypoint.north_south as f32 * cos) as isize;
            }
            Action::Forward => {
                ship.north_south += amount * waypoint.north_south;
                ship.east_west += amount * waypoint.east_west;
            }
        }
    }

    ship.north_south.abs() + ship.east_west.abs()
}
