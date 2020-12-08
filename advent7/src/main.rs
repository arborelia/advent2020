use eyre::Result;
use lazy_static::lazy_static;
use maplit::hashset;

use nom::{branch::alt, bytes::complete::tag}; // WHY IS IT CALLED THIS

use nom::character::complete::{alpha1, digit1};
use nom::combinator::{map_res, opt};
use nom::error::{ErrorKind, ParseError};
use nom::multi::separated_list1;
use nom::IResult;
use std::collections::HashSet;

mod helpers;
use helpers::get_lines;

lazy_static! {
    static ref REQUIRED_FIELDS: HashSet<&'static str> =
        hashset! {"byr:", "iyr:", "eyr:", "hgt:", "hcl:", "ecl:", "pid:"};
}

#[derive(Debug, PartialEq, Clone)]
struct AllowsContainment {
    container: String,
    contained: String,
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

fn parse_bag(input: &str) -> IResult<&str, String> {
    let (input, word1) = alpha1(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, word2) = alpha1(input)?;
    let (input, _) = tag(" bag")(input)?;
    let (input, _) = opt(tag("s"))(input)?;
    let bagname = format!("{} {}", word1, word2);
    Ok((input, bagname))
}

fn parse_multibag(input: &str) -> IResult<&str, String> {
    let (input, _num) = parse_u64(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, bagname) = parse_bag(input)?;
    Ok((input, bagname))
}

fn parse_baglist(input: &str) -> IResult<&str, Vec<String>> {
    separated_list1(tag(", "), parse_multibag)(input)
}

fn parse_empty_baglist(input: &str) -> IResult<&str, Vec<String>> {
    let (input, _) = tag("no other bags")(input)?;
    Ok((input, Vec::new()))
}

fn parse_containment(input: &str) -> IResult<&str, Vec<AllowsContainment>> {
    let (input, bag1) = parse_bag(input)?;
    let (input, _) = tag(" contain ")(input)?;
    let (input, containable_bags) = alt((parse_baglist, parse_empty_baglist))(input)?;
    let containment_rules: Vec<AllowsContainment> = containable_bags
        .iter()
        .cloned()
        .map(|bag2| AllowsContainment {
            container: bag1.clone(),
            contained: bag2.clone(),
        })
        .collect();
    let (input, _) = tag(".")(input)?;
    Ok((input, containment_rules))
}

fn parse_containment_complete(input: &str) -> Vec<AllowsContainment> {
    let (input, containment_rules) = parse_containment(input).unwrap();
    containment_rules
}

fn num_containers(bagname: &str, rules: &Vec<AllowsContainment>) -> usize {
    let mut bags: HashSet<String> = HashSet::new();
    for rule in rules {
        if rule.contained == bagname {
            if !bags.contains(&rule.container) {
                bags.insert(rule.container.to_owned());
            }
        }
    }

    let mut num_choices: usize = bags.len();
    loop {
        for rule in rules {
            if bags.contains(&rule.contained) {
                if !bags.contains(&rule.container) {
                    bags.insert(rule.container.to_owned());
                }
            }
        }
        if bags.len() == num_choices {
            return num_choices;
        }
        num_choices = bags.len();
    }
}

fn main() -> Result<()> {
    let rules: Vec<AllowsContainment> = get_lines("input.txt")
        .iter()
        .cloned()
        .flat_map(|line| parse_containment_complete(&line))
        .collect();
    // let mut rules: Vec<AllowsContainment> = Vec::new();
    // for line in get_lines("input.txt") {
    //     let (_, new_rules) = parse_containment(&line)?;
    //     rules.extend(new_rules);
    // }
    let num = num_containers("shiny gold", &rules);
    println!("{}", num);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_examples() -> Result<()> {
        Ok(())
    }
}
