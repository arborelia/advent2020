use eyre::Result;
use nom::{branch::alt, bytes::complete::tag}; // WHY IS IT CALLED THIS

use nom::character::complete::digit1;
use nom::combinator::{map_res, recognize};
use nom::sequence::tuple;
use nom::IResult;
use std::collections::HashSet;

mod helpers;
use helpers::get_lines;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Opcode {
    Nop,
    Acc,
    Jmp,
}

#[derive(Debug, Copy, Clone)]
struct Instruction {
    op: Opcode,
    val: i64,
}

fn recognize_i64(input: &str) -> IResult<&str, (&str, &str)> {
    tuple((alt((tag("+"), tag("-"))), digit1))(input)
}

/// Parse a decimal integer with no sign, and return it as an i64.
fn parse_i64(input: &str) -> IResult<&str, i64> {
    map_res(recognize(recognize_i64), str::parse)(input)
}

fn parse_acc(input: &str) -> IResult<&str, Opcode> {
    let (input, _) = tag("acc")(input)?;
    Ok((input, Opcode::Acc))
}

fn parse_jmp(input: &str) -> IResult<&str, Opcode> {
    let (input, _) = tag("jmp")(input)?;
    Ok((input, Opcode::Jmp))
}

fn parse_nop(input: &str) -> IResult<&str, Opcode> {
    let (input, _) = tag("nop")(input)?;
    Ok((input, Opcode::Nop))
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, opcode) = alt((parse_acc, parse_jmp, parse_nop))(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, num) = parse_i64(input)?;
    let inst = Instruction {
        op: opcode,
        val: num,
    };
    Ok((input, inst))
}

fn parse_instruction_complete(input: &str) -> Instruction {
    let (input, inst) = parse_instruction(input).unwrap();
    assert!(input == "");
    inst
}

/// Returns a boolean for whether the code terminates, and the value in
/// the accumulator when it terminates or loops.
fn run_until_loop(code: &Vec<Instruction>) -> (bool, i64) {
    let mut visited: HashSet<i64> = HashSet::new();
    let mut acc: i64 = 0;
    let mut pointer: i64 = 0;

    loop {
        // println!(
        //     "pointer = {}, acc = {}, instruction: {:?}",
        //     pointer, acc, &code[pointer as usize]
        // );
        if visited.contains(&pointer) {
            return (false, acc);
        }
        visited.insert(pointer);
        let instruction: &Instruction = &code[pointer as usize];
        match instruction.op {
            Opcode::Nop => {
                pointer += 1;
            }
            Opcode::Acc => {
                acc += instruction.val;
                pointer += 1;
            }
            Opcode::Jmp => {
                pointer += instruction.val;
            }
        }
        if pointer >= code.len() as i64 {
            return (true, acc);
        }
    }
}

fn find_corrupt_instruction(code: &Vec<Instruction>) -> i64 {
    for line_num in 0..code.len() {
        let mut modified_code = code.clone();
        let inst: Instruction = code[line_num];
        if inst.op != Opcode::Acc {
            if inst.op == Opcode::Nop {
                modified_code[line_num] = Instruction {
                    op: Opcode::Jmp,
                    val: inst.val,
                };
            } else {
                // op == Opcode::Jmp
                modified_code[line_num] = Instruction {
                    op: Opcode::Nop,
                    val: inst.val,
                };
            };
            let (terminates, acc) = run_until_loop(&modified_code);
            if terminates {
                return acc;
            }
        }
    }
    panic!("found no fix that makes the code terminate");
}

fn main() -> Result<()> {
    let instructions: Vec<Instruction> = get_lines("input.txt")
        .iter()
        .cloned()
        .map(|line| parse_instruction_complete(&line))
        .collect();

    let (terminates, acc) = run_until_loop(&instructions);
    assert!(!terminates);
    println!("Value before loop is {}", acc);

    let pointer = find_corrupt_instruction(&instructions);
    println!("corrupt instruction is {}", pointer);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let code = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

        let instructions: Vec<Instruction> = code
            .lines()
            .map(|line| parse_instruction_complete(&line))
            .collect();

        let (terminates, acc) = run_until_loop(&instructions);
        assert!(!terminates);
        assert_eq!(acc, 5);
    }
}
