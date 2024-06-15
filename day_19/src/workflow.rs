use std::{collections::HashMap, str::FromStr};

use nom::{
    bytes::complete::{take_until, take_while},
    character::complete::{char, digit1, one_of},
    combinator::map_res,
    error::Error,
    multi::separated_list0,
    sequence::delimited,
    Finish, IResult,
};

#[derive(Debug)]
pub enum Condition {
    Less(char, u32),
    Greater(char, u32),
}

#[derive(Debug)]
pub enum Next {
    Check(Condition, String),
    Else(String),
}

#[derive(Debug)]
pub struct Workflow(String, Vec<Next>);

#[derive(Debug)]
pub struct Workflows(pub HashMap<String, Vec<Next>>);

fn parse_condition(input: &str) -> IResult<&str, Condition> {
    let (input, variable) = one_of("xmas")(input)?;
    let (input, operator) = one_of("<>")(input)?;
    let (input, value) = map_res(digit1, u32::from_str)(input)?;
    let condition = match operator {
        '>' => Condition::Greater(variable, value),
        '<' => Condition::Less(variable, value),
        _ => unreachable!(),
    };
    Ok((input, condition))
}

fn parse_next(input: &str) -> IResult<&str, Next> {
    if let Ok((remaining, condition)) = take_until::<&str, &str, Error<_>>(":")(input) {
        let (input, condition) = parse_condition(condition)?;
        assert!(input.is_empty());
        let (input, _) = char(':')(remaining)?;
        let (input, next_name) = take_while(char::is_alphabetic)(input)?;
        return Ok((input, Next::Check(condition, next_name.to_string())));
    }

    let (input, next_name) = take_while(char::is_alphabetic)(input)?;

    Ok((input, Next::Else(next_name.to_string())))
}

fn parse_workflow(input: &str) -> IResult<&str, Workflow> {
    let (input, name) = take_until("{")(input)?;
    let (input, nexts) =
        delimited(char('{'), separated_list0(char(','), parse_next), char('}'))(input)?;

    Ok((input, Workflow(name.to_string(), nexts)))
}

fn parse_workflows(input: &str) -> IResult<&str, Workflows> {
    let x = input
        .lines()
        .map(|line| parse_workflow(line).unwrap())
        .map(|(_, workflow)| (workflow.0.to_string(), workflow.1))
        .collect::<HashMap<_, _>>();
    Ok(("", Workflows(x)))
}

impl FromStr for Workflows {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match parse_workflows(input).finish() {
            Ok((_, workflows)) => Ok(workflows),
            Err(Error { input, code }) => {
                eprintln!("input: {input}, error: {:?}", code);
                Err(())
            }
        }
    }
}
