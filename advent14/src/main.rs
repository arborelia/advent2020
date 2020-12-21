use nom::bytes::complete::{tag, take_while_m_n};
use nom::character::complete::digit1;
use nom::combinator::map_res;
use nom::IResult;
use nom::{branch::alt, multi::separated_list1};
use std::collections::{HashMap, HashSet};

#[derive(Copy, Clone, PartialEq, Debug)]
struct Bitmask {
    zeros: u64, // a binary number with 0s where the zeros are, 1s otherwise
    ones: u64,  // a binary number with 1s where the ones are, 0s otherwise
}

#[derive(Clone, PartialEq, Debug)]
struct FloatingMask {
    ones: u64, // a binary number with 1s where the ones are, 0s otherwise
    zeros: u64,
    x_locations: Vec<usize>,
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum Instruction {
    SetMask(Bitmask),     // change the bitmask
    SetValue(usize, u64), // set the value at the given location
}
use self::Instruction::*;

fn apply_bitmask(mask: Bitmask, value: u64) -> u64 {
    assert!(value < (1 << 36));
    (value | mask.ones) & mask.zeros
}

fn bitmask_to_floating(mask: Bitmask) -> FloatingMask {
    let mut locations: Vec<usize> = Vec::new();
    for bit in 0usize..36 {
        let val: u64 = 1 << bit;
        if val & mask.zeros & (!(mask.ones)) > 0 {
            locations.push(bit);
        }
    }
    let mut new_zeros: u64 = 0;
    for bit in locations.clone() {
        new_zeros |= 1 << bit
    }
    FloatingMask {
        ones: mask.ones,
        zeros: !new_zeros,
        x_locations: locations,
    }
}

fn apply_floating_mask(mask: &FloatingMask, addr: u64) -> HashSet<u64> {
    let n = mask.x_locations.len();
    let mut modified_addrs: HashSet<u64> = HashSet::new();
    for step in 0..(1 << n) {
        let mut bit_pattern: u64 = 0;
        for bit in 0..n {
            // if there's a 1 in the correct place in the step number
            if step & (1 << bit) != 0 {
                // set a 1 in the corresponding place in the bit pattern
                bit_pattern |= 1 << mask.x_locations[bit];
            }
        }
        let modified_addr = ((addr | mask.ones) & mask.zeros) + bit_pattern;
        modified_addrs.insert(modified_addr);
    }
    modified_addrs
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

fn run_instructions_step1(instructions: &[Instruction]) -> HashMap<usize, u64> {
    let mut memory: HashMap<usize, u64> = HashMap::new();
    let mut current_mask = Bitmask {
        zeros: 1 << 36 - 1,
        ones: 0,
    };

    for inst in instructions {
        match inst {
            SetMask(mask) => current_mask = *mask,
            SetValue(addr, value) => {
                memory.insert(*addr, apply_bitmask(current_mask, *value));
                ()
            }
        }
    }
    memory
}

fn run_instructions_step2(instructions: &[Instruction]) -> HashMap<usize, u64> {
    let mut memory: HashMap<usize, u64> = HashMap::new();
    let mut current_mask = bitmask_to_floating(Bitmask {
        zeros: 1 << 36 - 1,
        ones: 0,
    });

    for inst in instructions {
        match inst {
            SetMask(mask) => current_mask = bitmask_to_floating(*mask),
            SetValue(addr, value) => {
                for new_addr in apply_floating_mask(&current_mask, *addr as u64) {
                    memory.insert(new_addr as usize, *value);
                }
            }
        }
    }
    memory
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let instructions = parse_instructions(&input);
    let memory = run_instructions_step1(&instructions);
    let answer: u64 = memory.values().sum();
    println!("Total of memory values in version 1: {}", answer);

    let memory = run_instructions_step2(&instructions);
    let answer: u64 = memory.values().sum();
    println!("Total of memory values in version 2: {}", answer);
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

#[test]
fn test_floating_mask() {
    let mask = Bitmask {
        zeros: 0b000000000000000000000000000000110011,
        ones: 0b000000000000000000000000000000010010,
    };
    let floating_mask = bitmask_to_floating(mask);
    println!("mask is: {:?}", floating_mask);
    let mut assigned_addresses: Vec<u64> = apply_floating_mask(&floating_mask, 42)
        .into_iter()
        .collect();
    assigned_addresses.sort();
    assert_eq!(assigned_addresses, vec![26, 27, 58, 59]);
}
