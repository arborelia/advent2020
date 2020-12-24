use std::fs::read_to_string;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, space0};
use nom::combinator::map_res;
use nom::multi::{many0, separated_list1};
use nom::IResult;

#[derive(Debug, Clone, PartialEq)]
enum Operator {
    Add,
    Multiply,
}

/// Parse a decimal integer (with no sign) and return it as an i64.
fn parse_i64(input: &str) -> IResult<&str, i64> {
    map_res(digit1, str::parse)(input)
}

fn parse_add_operator(input: &str) -> IResult<&str, Operator> {
    let (input, _) = space0(input)?;
    let (input, _) = tag("+")(input)?;
    let (input, _) = space0(input)?;
    Ok((input, Operator::Add))
}

fn parse_multiply_operator(input: &str) -> IResult<&str, Operator> {
    let (input, _) = space0(input)?;
    let (input, _) = tag("*")(input)?;
    let (input, _) = space0(input)?;
    Ok((input, Operator::Multiply))
}

fn parse_operator(input: &str) -> IResult<&str, Operator> {
    alt((parse_add_operator, parse_multiply_operator))(input)
}

fn parse_parentheses_no_prec(input: &str) -> IResult<&str, i64> {
    let (input, _) = tag("(")(input)?;
    let (input, val) = parse_operation_sequence(input)?;
    let (input, _) = tag(")")(input)?;
    Ok((input, val))
}

fn parse_parentheses_with_prec(input: &str) -> IResult<&str, i64> {
    let (input, _) = tag("(")(input)?;
    let (input, val) = parse_product(input)?;
    let (input, _) = tag(")")(input)?;
    Ok((input, val))
}

fn parse_expression_no_prec(input: &str) -> IResult<&str, i64> {
    alt((parse_parentheses_no_prec, parse_i64))(input)
}

fn parse_expression_with_prec(input: &str) -> IResult<&str, i64> {
    alt((parse_parentheses_with_prec, parse_i64))(input)
}

fn parse_operation_tail(input: &str) -> IResult<&str, (Operator, i64)> {
    let (input, op) = parse_operator(input)?;
    let (input, val) = parse_expression_no_prec(input)?;
    Ok((input, (op, val)))
}

fn parse_sum(input: &str) -> IResult<&str, i64> {
    let (input, addends) = separated_list1(parse_add_operator, parse_expression_with_prec)(input)?;
    Ok((input, addends.iter().sum()))
}

fn parse_product(input: &str) -> IResult<&str, i64> {
    let (input, multipliers) = separated_list1(parse_multiply_operator, parse_sum)(input)?;
    Ok((input, multipliers.iter().product()))
}

// We have to parse entire operation sequences as a flat list. Trying to parse them by recursively extending
// them would require left-recursion, given the structure of the problem, and left-recursion is hard to support.
fn parse_operation_sequence(input: &str) -> IResult<&str, i64> {
    let (input, val) = parse_expression_no_prec(input)?;
    let (input, more_ops) = many0(parse_operation_tail)(input)?;

    let mut val = val;
    for (op, next_val) in more_ops {
        if op == Operator::Multiply {
            val *= next_val;
        } else {
            val += next_val;
        }
    }
    Ok((input, val))
}

fn calculate_no_precedence(input: &str) -> i64 {
    let (remain, val) =
        parse_operation_sequence(&input).expect(format!("failed to parse: {}", input).as_ref());
    if remain != "" {
        panic!("Leftover input: {}", remain);
    }
    val
}

/// Calculate an expression in this different version of math, where + takes precedence
/// over *.
fn calculate_with_precedence(input: &str) -> i64 {
    let (remain, val) =
        parse_product(&input).expect(format!("failed to parse: {}", input).as_ref());
    if remain != "" {
        panic!("Leftover input: {}", remain);
    }
    val
}

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let mut overall_sum: i64 = 0;
    for line in input.split("\n") {
        if line != "" {
            overall_sum += calculate_no_precedence(&line);
        }
    }
    println!("without precedence: {}", overall_sum);

    overall_sum = 0;
    for line in input.split("\n") {
        if line != "" {
            overall_sum += calculate_with_precedence(&line);
        }
    }
    println!("with inverted precedence: {}", overall_sum);
}

#[test]
fn test_no_parens() {
    assert_eq!(calculate_no_precedence("1+2*3+4*5+6"), 71);
    assert_eq!(calculate_no_precedence("1 + 2 * 3 + 4 * 5 + 6"), 71);
    assert_eq!(calculate_with_precedence("1+2*3+4*5+6"), 231);
    assert_eq!(calculate_with_precedence("1 + 2 * 3 + 4 * 5 + 6"), 231);
}

#[test]
fn test_simplest() {
    assert_eq!(calculate_no_precedence("1"), 1);
    assert_eq!(calculate_with_precedence("1"), 1);
}

#[test]
fn test_parens() {
    assert_eq!(
        calculate_no_precedence("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
        13632
    );
    assert_eq!(
        calculate_with_precedence("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
        23340
    );
}
