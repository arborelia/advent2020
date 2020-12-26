use nom::bytes::complete::tag;
use nom::character::complete::{anychar, digit1, space1};
use nom::combinator::map_res;
use nom::IResult;
use nom::{branch::alt, multi::separated_list1};

#[derive(Debug, Clone, PartialEq)]
struct RuleState {
    rule: ParseRule,
    tokens_matched: usize,
    start_pos: usize,
}

#[derive(Debug, Clone, PartialEq)]
struct ParseRule {
    rule_number: u64,
    content: ParseExpression,
}
#[derive(Debug, Clone, PartialEq)]
enum ParseExpression {
    Terminal(char),
    Nonterminal(Vec<u64>),
}

/// Parse a decimal integer (with no sign) and return it as an i64.
fn parse_u64(input: &str) -> IResult<&str, u64> {
    map_res(digit1, str::parse)(input)
}

fn parse_terminal_rule(input: &str) -> IResult<&str, Vec<ParseExpression>> {
    let (input, _) = nom::character::complete::char('"')(input)?;
    let (input, target) = anychar(input)?;
    let (input, _) = nom::character::complete::char('"')(input)?;
    Ok((input, vec![ParseExpression::Terminal(target)]))
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
    let (_input, rules) = parse_parse_rule(input).unwrap();
    rules
}

fn expr_size(expr: &ParseExpression) -> usize {
    match &expr {
        ParseExpression::Terminal(_) => 1,
        ParseExpression::Nonterminal(vec) => vec.len(),
    }
}

fn is_complete(state: &RuleState) -> bool {
    expr_size(&state.rule.content) == state.tokens_matched
}

fn advance(state: &RuleState) -> RuleState {
    RuleState {
        rule: state.rule.clone(),
        tokens_matched: state.tokens_matched + 1,
        start_pos: state.start_pos,
    }
}

fn instantiate(rule: &ParseRule, pos: usize) -> RuleState {
    RuleState {
        rule: rule.clone(),
        tokens_matched: 0,
        start_pos: pos,
    }
}

fn cfg_parse_string(grammar: &[ParseRule], string: &str) -> bool {
    let mut init_rule: Option<ParseRule> = None;
    for rule in grammar.iter().cloned() {
        if rule.rule_number == 0 {
            init_rule = Some(rule);
        }
    }

    let mut chart: Vec<Vec<RuleState>> = Vec::new();
    let mut incomplete_parses: Vec<Vec<RuleState>> = Vec::new();
    let n = string.len();

    // create an empty queue for each character of input, plus one after the end
    for _i in 0..=n {
        chart.push(Vec::new());
    }
    chart[0].push(instantiate(&init_rule.unwrap(), 0));
    let chars: Vec<char> = string.chars().collect();

    // Earley algorithm
    for pos in 0..=n {
        loop {
            let mut new_states: Vec<RuleState> = Vec::new();
            for state in chart[pos].clone() {
                if is_complete(&state) {
                    if pos == n && state.rule.rule_number == 0 {
                        // this is a parse of the entire string
                        return true;
                    }

                    // Completer step
                    let prev_pos = state.start_pos;
                    let other_states: Vec<RuleState> = chart[prev_pos].iter().cloned().collect();
                    for other_state in other_states {
                        if !is_complete(&other_state) {
                            let expr = other_state.rule.content.clone();
                            match expr {
                                ParseExpression::Nonterminal(tokens) => {
                                    let next_token = tokens[other_state.tokens_matched];
                                    if next_token == state.rule.rule_number {
                                        let new_state = advance(&other_state);
                                        if !chart[pos].contains(&new_state)
                                            && !new_states.contains(&new_state)
                                        {
                                            new_states.push(new_state);
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                } else {
                    let expr = state.rule.content.clone();
                    match expr {
                        ParseExpression::Nonterminal(tokens) => {
                            // Predictor step
                            let next_token = tokens[state.tokens_matched];
                            for rule in grammar.iter().cloned() {
                                if rule.rule_number == next_token {
                                    let new_state = instantiate(&rule, pos);
                                    if !chart[pos].contains(&new_state)
                                        && !new_states.contains(&new_state)
                                    {
                                        new_states.push(new_state);
                                    }
                                }
                            }
                        }
                        ParseExpression::Terminal(ch) => {
                            // Scanner step
                            if pos < n && chars[pos] == ch {
                                let new_state = advance(&state);
                                if !chart[pos + 1].contains(&new_state) {
                                    chart[pos + 1].push(new_state);
                                }
                            }
                        }
                    }
                }
            }
            if new_states.len() > 0 {
                chart[pos].extend(new_states);
            } else {
                break;
            }
        }
        incomplete_parses.push(
            chart[pos]
                .iter()
                .cloned()
                .filter(|state| is_complete(state))
                .collect(),
        );
    }
    // we never saw a parse of the whole string
    false
}

fn main() {
    let input = std::fs::read_to_string("rules2.txt").unwrap();
    let lines = input.trim().split("\n");
    let rules: Vec<ParseRule> = lines.flat_map(parse_parse_rule_complete).collect();
    println!("{:?}", rules);

    let input = std::fs::read_to_string("input.txt").unwrap();
    let lines = input.trim().split("\n");
    let mut num_parsed: u64 = 0;
    for line in lines {
        if cfg_parse_string(&rules, line) {
            println!("parsed: {}", line);
            num_parsed += 1;
        } else {
            println!("didn't parse: {}", line);
        }
    }
    println!("Lines that match: {}", num_parsed);
}

#[test]
fn test_small_grammar() {
    let grammar_def = vec![
        "0: 4 1 5",
        "1: 2 3 | 3 2",
        "2: 4 4 | 5 5",
        "3: 4 5 | 5 4",
        "4: \"a\"",
        "5: \"b\"",
    ];
    let grammar: Vec<ParseRule> = grammar_def
        .iter()
        .cloned()
        .flat_map(parse_parse_rule_complete)
        .collect();

    assert!(cfg_parse_string(&grammar, "ababbb"));
    assert!(cfg_parse_string(&grammar, "abbbab"));
    assert!(!cfg_parse_string(&grammar, "bababa"));
    assert!(!cfg_parse_string(&grammar, "aaabbb"));
    assert!(!cfg_parse_string(&grammar, "aaaabbb"));
    assert!(!cfg_parse_string(&grammar, "a"));
    assert!(!cfg_parse_string(&grammar, ""));
}
