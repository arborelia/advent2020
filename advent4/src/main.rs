use nom::character::is_digit;
use nom::bytes::complete::take_while_m_n;
use eyre::{Result, eyre};
use std::collections::{HashSet, HashMap};
use std::fs;
use lazy_static::lazy_static;
use nom::{IResult, character::complete::digit1, combinator::*, sequence::preceded, sequence::terminated};
use nom::error::{ErrorKind, ParseError};
use nom::branch::alt;
use nom::bytes::complete::tag; // WHY IS IT CALLED THIS
use maplit::hashset;


lazy_static! {
    static ref REQUIRED_FIELDS: HashSet<&'static str> = hashset!{"byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"};
}


// fn fail(input: &str) -> nom::Err<dyn nom::error::ParseError<&str> > {
//     nom::Err::Error(ParseError::from_error_kind(input, ErrorKind::Verify))
// }


fn parse_u64(input: &str) -> IResult<&str, u64> {
    map_res(recognize(digit1), str::parse)(input)
}


fn parse_height_in(input: &str) -> IResult<&str, u64> {
    verify(
        terminated(
            parse_u64,
            tag("in")
        ),
        |&num| (num >= 59 && num <= 74)
    )(input)
}

fn parse_height_cm(input: &str) -> IResult<&str, u64> {
    verify(
        terminated(
            parse_u64,
            tag("cm")
        ),
        |&num| (num >= 150 && num <= 193)
    )(input)
}

fn parse_height(input: &str) -> IResult<&str, &str> {
    terminated(
        tag("hgt="),  // this will return "hgt=" if the height validates
        alt((parse_height_in, parse_height_cm))
    )(input)
}

fn parse_iyr(input: &str) -> IResult<&str, &str> {
    terminated(
        tag("iyr="),
        verify(
            parse_u64,
            |&num| (num >= 2010 && num <= 2020)
        )
    )(input)
}

fn parse_eyr(input: &str) -> IResult<&str, &str> {
    terminated(
        tag("eyr="),
        verify(
            parse_u64,
            |&num| (num >= 2020 && num <= 2030)
        )
    )(input)
}


fn parse_byr(input: &str) -> IResult<&str, &str> {
    terminated(
        tag("byr="),
        verify(
            parse_u64,
            |&num| (num >= 1920 && num <= 2002)
        )
    )(input)
}

fn is_hex_digit(c: char) -> bool {
    c.is_digit(16)
}

fn is_dec_digit(c: char) -> bool {
    c.is_digit(10)
}

fn parse_hcl(input: &str) -> IResult<&str, &str> {
    terminated(
        tag("hcl="),
        preceded(
            tag("#"),
            take_while_m_n(6, 6, is_hex_digit)
        )
    )(input)
}

fn parse_ecl(input: &str) -> IResult<&str, &str> {
    terminated(
        tag("ecl="),
        alt(
            (tag("amb"), tag("blu"), tag("brn"), tag("gry"), tag("grn"), tag("hzl"), tag("oth"))
        )
    )(input)
}

fn parse_pid(input: &str) -> IResult<&str, &str> {
    terminated(
        tag("pid="),
        take_while_m_n(9, 9, is_dec_digit)
    )(input)
}

fn main() -> Result<()> {
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