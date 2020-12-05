use eyre::Result;
use lazy_static::lazy_static;
use maplit::hashset;
use nom::branch::alt;
use nom::bytes::complete::tag; // WHY IS IT CALLED THIS
use nom::bytes::complete::take_while_m_n;
use nom::character::complete::{alpha1, digit1, multispace0, multispace1, none_of};
use nom::combinator::{all_consuming, map_res, recognize, verify};
use nom::error::{ErrorKind, ParseError};
use nom::multi::{many1, separated_list1};
use nom::sequence::{preceded, terminated};
use nom::IResult;
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
fn parse_valid_field(input: &str) -> IResult<&str, &str> {
    alt((
        parse_hgt, parse_iyr, parse_byr, parse_eyr, parse_hcl, parse_ecl, parse_pid, parse_cid,
    ))(input)
}

fn non_whitespace(input: &str) -> IResult<&str, &str> {
    recognize(many1(none_of(" \n\t")))(input)
}

// the "cid" field, if present, can contain anything but whitespace.
fn parse_cid(input: &str) -> IResult<&str, &str> {
    terminated(tag("cid:"), non_whitespace)(input)
}

/// Parse any field name, returning the field name and the following colon.
fn parse_field_name(input: &str) -> IResult<&str, &str> {
    recognize(terminated(alpha1, tag(":")))(input)
}

fn parse_arbitrary_field(input: &str) -> IResult<&str, &str> {
    terminated(parse_field_name, non_whitespace)(input)
}

/// Parse just the field names of a single passport, returning the fields it contains,
/// regardless of whether their values are valid.
pub fn parse_passport_fields(input: &str) -> IResult<&str, HashSet<&str>> {
    // We need to allow optional whitespace at the end,
    let (input, field_list) = all_consuming(terminated(
        separated_list1(multispace1, parse_arbitrary_field),
        multispace0,
    ))(input)?;
    Ok((input, HashSet::from_iter(field_list)))
}

/// Parse a single complete passport, returning the valid fields it contains.
pub fn parse_valid_passport(input: &str) -> IResult<&str, HashSet<&str>> {
    // We need to allow optional whitespace at the end,
    let (input, field_list) = all_consuming(terminated(
        separated_list1(multispace1, parse_valid_field),
        multispace0,
    ))(input)?;
    Ok((input, HashSet::from_iter(field_list)))
}

/// Parse a complete file of passports separated by empty lines, returning the number of passports that
/// are well-formed (contain the seven required fields, regardless of their values).
pub fn num_wellformed_passports(input: &str) -> u64 {
    let passports = input.split("\n\n");
    let mut num_well_formed: u64 = 0;
    for passport in passports {
        let parsed = parse_passport_fields(passport);
        if let Ok((input, fields)) = parsed {
            // the all_consuming combinator should ensure that there's no input left, but let's check\
            assert!(input == "");
            if fields.is_superset(&REQUIRED_FIELDS) {
                num_well_formed += 1;
            }
        }
    }
    num_well_formed
}

/// Parse a complete file of passports separated by empty lines, returning the number of valid passports in it.
pub fn num_valid_passports(input: &str) -> u64 {
    let passports = input.split("\n\n");
    let mut num_valid: u64 = 0;
    for passport in passports {
        let parsed = parse_valid_passport(passport);
        if let Ok((input, fields)) = parsed {
            // the all_consuming combinator should ensure that there's no input left, but let's check\
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
    let num_well_formed = num_wellformed_passports(&input);
    let num_valid = num_valid_passports(&input);
    println!(
        "{} well-formed passports\n{} valid passports",
        num_well_formed, num_valid
    );
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_valid_examples() -> Result<()> {
        let needed_fields: HashSet<&str> = REQUIRED_FIELDS.clone();
        let fields_plus_cid: HashSet<&str> =
            hashset! {"iyr:", "eyr:", "byr:", "hgt:", "ecl:", "hcl:", "pid:", "cid:"};

        let (_, fields) = parse_valid_passport(
            "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
        hcl:#623a2f",
        )?;
        assert!(fields == needed_fields);

        let (_, fields) = parse_valid_passport(
            "eyr:2029 ecl:blu cid:129 byr:1989
            iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm",
        )?;
        assert!(fields == fields_plus_cid);

        let (_, fields) = parse_valid_passport(
            "hcl:#888785
            hgt:164cm byr:2001 iyr:2015 cid:88
            pid:545766238 ecl:hzl
            eyr:2022",
        )?;
        assert!(fields == fields_plus_cid);

        let (_, fields) = parse_valid_passport(
            "iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719",
        )?;
        assert!(fields == needed_fields);

        Ok(())
    }

    #[test]
    fn test_invalid_examples() -> Result<()> {
        let parsed = parse_valid_passport(
            "eyr:1972 cid:100
            hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926",
        );
        assert!(parsed.is_err());

        let parsed = parse_valid_passport(
            "iyr:2019
            hcl:#602927 eyr:1967 hgt:170cm
            ecl:grn pid:012533040 byr:1946",
        );
        assert!(parsed.is_err());

        let parsed = parse_valid_passport(
            "eyr:1972 cid:100
            hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926",
        );
        assert!(parsed.is_err());

        let parsed = parse_valid_passport(
            "eyr:1972 cid:100
            hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926",
        );
        assert!(parsed.is_err());

        // Ideally I'd be able to test where the parse failed, but these Error results
        // seem impossible to take apart.
        // Leaving this code for possible help later.
        //
        // if let Err(nom::Err::Error((rest, ErrorKind::Verify))) = parsed {
        //     assert_eq!(
        //         rest,
        //         " cid:100
        //     hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926"
        //     );
        // } else {
        //     assert!(false, "this parsed incorrectly");
        // };

        Ok(())
    }
}
