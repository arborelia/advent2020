use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, digit1, space1};
use nom::combinator::{map_res, recognize};
use nom::IResult;
use nom::{branch::alt, multi::separated_list1};

#[derive(Debug, Clone)]
struct ParseRule {
    rule_number: u64,
    content: ParseExpression,
}

#[derive(Debug, Clone)]
enum ParseExpression {
    Terminal(String),
    Nonterminal(Vec<u64>),
}

/// Parse a decimal integer (with no sign) and return it as an i64.
fn parse_u64(input: &str) -> IResult<&str, u64> {
    map_res(digit1, str::parse)(input)
}

fn parse_terminal_rule(input: &str) -> IResult<&str, Vec<ParseExpression>> {
    let (input, _) = nom::character::complete::char('"')(input)?;
    let (input, target) = alphanumeric1(input)?;
    let (input, _) = nom::character::complete::char('"')(input)?;
    Ok((input, vec![ParseExpression::Terminal(target.to_string())]))
}

fn parse_nonterminal_rule(input: &str) -> IResult<&str, Vec<ParseExpression>> {
    let (input, sequences) = separated_list1(tag(" | "), parse_sequence)(input)?;
    Ok((
        input,
        sequences
            .into_iter()
            .map(|seq| ParseExpression::Nonterminal(seq))
            .collect(),
    ))
}

fn parse_sequence(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(space1, parse_u64)(input)
}

fn parse_parse_rule(input: &str) -> IResult<&str, Vec<ParseRule>> {
    let (input, rule_number) = parse_u64(input)?;
    let (input, _) = tag(": ")(input)?;
    // update to parse_nonterminal_rule
    let (input, rule_contents) = alt((parse_terminal_rule, parse_nonterminal_rule))(input)?;
    let rules = rule_contents
        .into_iter()
        .map(|content| ParseRule {
            rule_number,
            content,
        })
        .collect();
    Ok((input, rules))
}

fn parse_parse_rule_complete(input: &str) -> Vec<ParseRule> {
    let (input, rules) = parse_parse_rule(input).unwrap();
    rules
}
fn main() {
    let input = std::fs::read_to_string("rules.txt").unwrap();
    let lines = input.trim().split("\n");
    let rules: Vec<ParseRule> = lines.flat_map(parse_parse_rule_complete).collect();
    println!("{:?}", rules);
}
