use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

#[derive(Clone, Copy)]
enum OpCode {
    Nop,
    Acc,
    Jmp,
}

impl OpCode {
    fn from_str(value: &str) -> OpCode {
        match value {
            "nop" => OpCode::Nop,
            "acc" => OpCode::Acc,
            "jmp" => OpCode::Jmp,
            _ => panic!("Invalid opcode"),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Instruction {
    op_code: OpCode,
    number: i32,
}

enum ProgramOutput {
    Error(i32),
    Ok(i32),
}

#[aoc_generator(day8)]
pub fn get_values(input: &str) -> HashMap<usize, Instruction> {
    lazy_static! {
        static ref PARSE_EXPR: Regex =
            Regex::new(r"^(?P<op_code>nop|acc|jmp) (?P<number>[-+]\d+)$").unwrap();
    }

    input
        .lines()
        .enumerate()
        .fold(HashMap::new(), |mut acc, (idx, l)| {
            let captures = PARSE_EXPR.captures(l).unwrap();
            acc.insert(
                idx + 1,
                Instruction {
                    op_code: OpCode::from_str(&captures["op_code"]),
                    number: captures["number"].parse::<i32>().unwrap(),
                },
            );
            acc
        })
}

fn evaluate_program(program: &HashMap<usize, Instruction>) -> ProgramOutput {
    let mut current_line_number = 1;
    let mut accumulator = 0;
    let mut executed_lines: HashSet<usize> = HashSet::new();
    loop {
        if current_line_number > program.len() {
            return ProgramOutput::Ok(accumulator);
        }
        if executed_lines.get(&current_line_number).is_some() {
            return ProgramOutput::Error(accumulator);
        }
        executed_lines.insert(current_line_number);

        let instruction = program.get(&current_line_number).unwrap();
        match instruction.op_code {
            OpCode::Acc => {
                accumulator += instruction.number;
                current_line_number += 1;
            }
            OpCode::Jmp => current_line_number += instruction.number as usize,
            OpCode::Nop => {
                current_line_number += 1;
            }
        }
    }
}

#[aoc(day8, part1)]
pub fn part1(program: &HashMap<usize, Instruction>) -> i32 {
    match evaluate_program(&program) {
        ProgramOutput::Error(v) => v,
        ProgramOutput::Ok(_) => panic!("shouldn't have gotten here"),
    }
}

#[aoc(day8, part2)]
pub fn part2(program_original: &HashMap<usize, Instruction>) -> i32 {
    let mut line_number_to_change = 1;

    loop {
        let mut program = program_original.clone();
        let current_line = program.entry(line_number_to_change);
        current_line.and_modify(|mut instruction| {
            match instruction.op_code {
                OpCode::Jmp => instruction.op_code = OpCode::Nop,
                OpCode::Nop => instruction.op_code = OpCode::Jmp,
                OpCode::Acc => {}
            };
        });

        match evaluate_program(&program) {
            ProgramOutput::Ok(v) => return v,
            ProgramOutput::Error(_) => {}
        }

        line_number_to_change += 1;
    }
}
