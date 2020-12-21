use nom::bytes::complete::{tag, take_while_m_n};
use nom::character::complete::digit1;
use nom::combinator::map_res;
use nom::IResult;
use nom::{branch::alt, multi::separated_list1};

#[derive(Copy, Clone, PartialEq, Debug)]
struct Bitmask {
    zeros: u64, // a binary number with 0s where the zeros are, 1s otherwise
    ones: u64,  // a binary number with 1s where the ones are, 0s otherwise
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum Instruction {
    SetMask(Bitmask),     // change the bitmask
    SetValue(usize, u64), // set the value at the given location
}
use self::Instruction::*;

fn highest_addr(instructions: &[Instruction]) -> usize {
    let mut highest: usize = 0;
    for inst in instructions {
        if let SetValue(addr, _value) = inst {
            if *addr > highest {
                highest = *addr;
            }
        }
    }
    highest
}

fn run_instructions(instructions: &[Instruction]) -> Vec<u64> {
    let highest = highest_addr(instructions);
    let mut memory: Vec<u64> = vec![0; highest + 1];
    let mut current_mask = Bitmask {
        zeros: 1 << 36 - 1,
        ones: 0,
    };

    for inst in instructions {
        match inst {
            SetMask(mask) => current_mask = *mask,
            SetValue(addr, value) => memory[*addr] = apply_bitmask(current_mask, *value),
        }
    }
    memory
}

fn apply_bitmask(mask: Bitmask, value: u64) -> u64 {
    assert!(value < (1 << 36));
    (value | mask.ones) & mask.zeros
}

fn is_mask_character(c: char) -> bool {
    c == '0' || c == '1' || c == 'X'
}

fn parse_mask(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("mask = ")(input)?;
    let (input, mask_chars) = take_while_m_n(36, 36, is_mask_character)(input)?;
    let ones_chars = mask_chars.replace("X", "0");
    let zeros_chars = mask_chars.replace("X", "1");
    let mask = Bitmask {
        ones: u64::from_str_radix(&ones_chars, 2).unwrap(),
        zeros: u64::from_str_radix(&zeros_chars, 2).unwrap(),
    };
    Ok((input, SetMask(mask)))
}

/// Parse a decimal integer (with no sign) and return it as a u64.
/// (Something that ought to already exist in nom.)
fn parse_u64(input: &str) -> IResult<&str, u64> {
    map_res(digit1, str::parse)(input)
}

fn parse_set_value(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("mem[")(input)?;
    let (input, addr) = parse_u64(input)?;
    let (input, _) = tag("] = ")(input)?;
    let (input, value) = parse_u64(input)?;
    Ok((input, SetValue(addr as usize, value)))
}

fn parse_instruction_list(input: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list1(tag("\n"), alt((parse_set_value, parse_mask)))(input)
}

// Parse a listing of instructions, and panic if it doesn't work.
fn parse_instructions(input: &str) -> Vec<Instruction> {
    let (input, instructions) = parse_instruction_list(input).unwrap();
    assert_eq!(input.trim(), "");
    instructions
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let instructions = parse_instructions(&input);
    let memory = run_instructions(&instructions);
    let answer: u64 = memory.iter().sum();
    println!("Total of memory values: {}", answer);
}

#[test]
fn test_bitmask() {
    // example bitmask: XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
    let mask: Bitmask = Bitmask {
        zeros: 0b111111111111111111111111111111111101,
        ones: 0b000000000000000000000000000001000000,
    };
    assert_eq!(apply_bitmask(mask, 0b1011), 0b1001001);
    assert_eq!(apply_bitmask(mask, 101), 101);
    assert_eq!(apply_bitmask(mask, 0), 64);
}

#[test]
fn test_parse() {
    let example = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";
    let parsed = parse_instructions(example);
    let expected = vec![
        SetMask(Bitmask {
            zeros: 0b111111111111111111111111111111111101,
            ones: 0b000000000000000000000000000001000000,
        }),
        SetValue(8, 11),
        SetValue(7, 101),
        SetValue(8, 0),
    ];
    assert_eq!(parsed, expected)
}
