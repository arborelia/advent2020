use eyre::{eyre, Result};
use lazy_static::lazy_static;
use maplit::hashset;
use nom::bytes::complete::tag; // WHY IS IT CALLED THIS
use nom::bytes::complete::take_while_m_n;
use nom::character::complete::{alphanumeric1, multispace1};
use nom::error::{ErrorKind, ParseError};
use nom::{branch::alt, multi::separated_list1};
use nom::{
    character::complete::digit1, combinator::*, sequence::preceded, sequence::terminated, IResult,
};
use std::collections::HashSet;
use std::fs;
use std::iter::FromIterator;

lazy_static! {
    static ref REQUIRED_FIELDS: HashSet<&'static str> =
        hashset! {"byr:", "iyr:", "eyr:", "hgt:", "hcl:", "ecl:", "pid:"};
}

fn is_hex_digit(c: char) -> bool {
    c.is_digit(16)
}

fn is_dec_digit(c: char) -> bool {
    c.is_digit(10)
}

/// Construct a value to go inside an Err() to indicate that you've checked a value
/// you just parsed, and it shouldn't count as a successful parse. An example here
/// is a height that is out of range.
///
/// This function was helpfully provided by @tanriol on gitter.im/Geal/nom.
/// I do not understand it in the slightest, and it ought to be a nom internal.
fn fail_verification<I, E: ParseError<I>>(input: I) -> nom::Err<E> {
    nom::Err::Error(ParseError::from_error_kind(input, ErrorKind::Verify))
}

/// Parse a decimal integer (with no sign) and return it as a u64.
/// (Another thing that ought to already exist in nom.)
fn parse_u64(input: &str) -> IResult<&str, u64> {
    map_res(digit1, str::parse)(input)
}

// These could have been written with the verify() combinator, like the year parsers below,
// but what I found was that verify's error messages were indecipherable.
//
// When I asked for help on gitter, the suggestion was that I should rewrite the functions
// as procedural code, which became possible with the help of the `fail_verification` function.
fn parse_height_in(input: &str) -> IResult<&str, u64> {
    let (input, num) = parse_u64(input)?;
    let (input, _tag) = tag("in")(input)?;
    if num < 59 || num > 76 {
        Err(fail_verification(input))
    } else {
        Ok((input, num))
    }
}

fn parse_height_cm(input: &str) -> IResult<&str, u64> {
    let (input, num) = parse_u64(input)?;
    let (input, _tag) = tag("cm")(input)?;
    if num < 150 || num > 193 {
        Err(fail_verification(input))
    } else {
        Ok((input, num))
    }
}

fn parse_hgt(input: &str) -> IResult<&str, &str> {
    terminated(
        tag("hgt:"), // this will return "hgt=" if the height validates
        alt((parse_height_in, parse_height_cm)),
    )(input)
}

fn parse_iyr(input: &str) -> IResult<&str, &str> {
    terminated(
        tag("iyr:"),
        verify(parse_u64, |&num| (num >= 2010 && num <= 2020)),
    )(input)
}

fn parse_eyr(input: &str) -> IResult<&str, &str> {
    terminated(
        tag("eyr:"),
        verify(parse_u64, |&num| (num >= 2020 && num <= 2030)),
    )(input)
}

fn parse_byr(input: &str) -> IResult<&str, &str> {
    terminated(
        tag("byr:"),
        verify(parse_u64, |&num| (num >= 1920 && num <= 2002)),
    )(input)
}

fn parse_hcl(input: &str) -> IResult<&str, &str> {
    // parse "hcl:", followed by "#" and exactly 6 hexadecimal digits.
    //
    // The `nom::bytes::take_while_m_n` combinator consumes characters (even
    // though it's called 'bytes'?) and checks each against a condition.
    terminated(
        tag("hcl:"),
        preceded(tag("#"), take_while_m_n(6, 6, is_hex_digit)),
    )(input)
}

fn parse_ecl(input: &str) -> IResult<&str, &str> {
    terminated(
        tag("ecl:"),
        alt((
            tag("amb"),
            tag("blu"),
            tag("brn"),
            tag("gry"),
            tag("grn"),
            tag("hzl"),
            tag("oth"),
        )),
    )(input)
}

fn parse_pid(input: &str) -> IResult<&str, &str> {
    // parse "pid:", followed by exactly 9 decimal digits
    terminated(tag("pid:"), take_while_m_n(9, 9, is_dec_digit))(input)
}

/// Parse any field of the passport, returning its tag (including the colon), such as
/// "pid:" if it successfully parsed a pid field.
fn parse_any_field(input: &str) -> IResult<&str, &str> {
    alt((
        parse_hgt, parse_iyr, parse_byr, parse_eyr, parse_hcl, parse_ecl, parse_pid, parse_cid,
    ))(input)
}

// the "cid" field, if present, can contain anything. In practice, it seems to always be
// alphanumeric.
fn parse_cid(input: &str) -> IResult<&str, &str> {
    terminated(tag("cid:"), alphanumeric1)(input)
}

/// Parse a single complete passport, returning the fields it contains.
pub fn parse_passport(input: &str) -> IResult<&str, HashSet<&str>> {
    let (input, field_list) = all_consuming(separated_list1(multispace1, parse_any_field))(input)?;
    Ok((input, HashSet::from_iter(field_list)))
}

/// Parse a complete file of passports separated by empty lines, returning the number of valid passports in it.
pub fn num_valid_passports(input: &str) -> u64 {
    let passports = input.split("\n\n");
    let mut num_valid: u64 = 0;
    for passport in passports {
        let parsed = parse_passport(passport);
        if let Ok((input, fields)) = parsed {
            // the all_consuming combinator should ensure that there's no input left, but let's check
            assert!(input == "");
            if fields.is_superset(&REQUIRED_FIELDS) {
                num_valid += 1;
            }
        }
    }
    num_valid
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    println!("{}\n", num_valid_passports(&input));
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() -> Result<()> {
        Ok(())
    }
}
