use std::str::FromStr;

use nom::{
    character::complete::{anychar, char, digit1, newline},
    combinator::map_res,
    error::Error,
    multi::separated_list0,
    sequence::{delimited, separated_pair},
    Finish, IResult,
};

#[derive(Default, Debug)]
pub struct Part {
    pub x: u32,
    pub m: u32,
    pub a: u32,
    pub s: u32,
}

#[derive(Debug)]
pub struct Parts(pub Vec<Part>);

fn key_value(input: &str) -> IResult<&str, (char, u32)> {
    separated_pair(anychar, char('='), map_res(digit1, u32::from_str))(input)
}

fn parse_part(input: &str) -> IResult<&str, Part> {
    let (input, key_values) =
        delimited(char('{'), separated_list0(char(','), key_value), char('}'))(input)?;
    let mut part = Part::default();
    for (key, value) in key_values {
        match key {
            'x' => part.x = value,
            'm' => part.m = value,
            'a' => part.a = value,
            's' => part.s = value,
            _ => unreachable!(),
        }
    }

    Ok((input, part))
}

fn parse_parts(input: &str) -> IResult<&str, Parts> {
    let (input, parts) = separated_list0(newline, parse_part)(input)?;

    Ok((input, Parts(parts)))
}

impl FromStr for Parts {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match parse_parts(input).finish() {
            Ok((_, parts)) => Ok(parts),
            Err(Error { input, code }) => {
                eprintln!("input: {input}, error: {:?}", code);
                Err(())
            }
        }
    }
}
