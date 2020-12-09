use eyre::Result;
use lazy_static::lazy_static;
use maplit::hashset;

use nom::{branch::alt, bytes::complete::tag}; // WHY IS IT CALLED THIS

use nom::character::complete::{alpha1, digit1};
use nom::combinator::{map_res, opt};
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
    desc: String,
    contained: Vec<MultiBag>,
}

#[derive(Debug, PartialEq, Clone)]
struct MultiBag {
    desc: String,
    num: u64,
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

fn parse_multibag(input: &str) -> IResult<&str, MultiBag> {
    let (input, num) = parse_u64(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, bagname) = parse_bag(input)?;
    Ok((
        input,
        MultiBag {
            desc: bagname,
            num: num,
        },
    ))
}

fn parse_baglist(input: &str) -> IResult<&str, Vec<MultiBag>> {
    separated_list1(tag(", "), parse_multibag)(input)
}

fn parse_empty_baglist(input: &str) -> IResult<&str, Vec<MultiBag>> {
    let (input, _) = tag("no other bags")(input)?;
    Ok((input, Vec::new()))
}

fn parse_containment(input: &str) -> IResult<&str, AllowsContainment> {
    let (input, bag1) = parse_bag(input)?;
    let (input, _) = tag(" contain ")(input)?;
    let (input, containable_bags) = alt((parse_baglist, parse_empty_baglist))(input)?;
    let containment_rule = AllowsContainment {
        desc: bag1,
        contained: containable_bags,
    };
    let (input, _) = tag(".")(input)?;
    Ok((input, containment_rule))
}

fn parse_containment_complete(input: &str) -> AllowsContainment {
    let (_input, containment_rule) = parse_containment(input).unwrap();
    containment_rule
}

fn num_contained(bagname: &str, rules: &Vec<AllowsContainment>) -> u64 {
    for rule in rules {
        if rule.desc == bagname {
            let mut num: u64 = 0;
            for contained in &rule.contained[..] {
                num += (1 + num_contained(&contained.desc, rules)) * contained.num;
            }
            return num;
        }
    }
    panic!("No rules for what a {} bag can contain", bagname);
}

fn num_containers(bagname: &str, rules: &Vec<AllowsContainment>) -> usize {
    let mut bags: HashSet<String> = HashSet::new();

    bags.insert(bagname.to_owned());
    let mut num_choices: usize = bags.len();
    loop {
        for rule in rules {
            for contained in &rule.contained {
                if bags.contains(&contained.desc) {
                    if !bags.contains(&rule.desc) {
                        bags.insert(rule.desc.to_owned());
                    }
                }
            }
        }
        if bags.len() == num_choices {
            return num_choices - 1;
        }
        num_choices = bags.len();
    }
}

fn main() -> Result<()> {
    let rules: Vec<AllowsContainment> = get_lines("input.txt")
        .iter()
        .cloned()
        .map(|line| parse_containment_complete(&line))
        .collect();

    let n_containers = num_containers("shiny gold", &rules);
    let n_contained = num_contained("shiny gold", &rules);

    println!(
        "{} different bags can contain a shiny gold bag",
        n_containers
    );
    println!("a shiny gold bag contains {} bags", n_contained);
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
