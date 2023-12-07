use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{multispace0, u32},
    combinator::{map, value},
    error::Error,
    multi::separated_list1,
    sequence::{delimited, pair, separated_pair},
    Finish, IResult,
};
use std::str::FromStr;

#[derive(Clone, PartialEq)]
enum Color {
    Red,
    Blue,
    Green,
}

struct Subset {
    cubes: Vec<(u32, Color)>,
}

struct Game {
    id: u32,
    subsets: Vec<Subset>,
}

fn parse_color(s: &str) -> IResult<&str, Color> {
    alt((
        value(Color::Blue, tag("blue")),
        value(Color::Green, tag("green")),
        value(Color::Red, tag("red")),
    ))(s)
}

fn parse_cube(s: &str) -> IResult<&str, (u32, Color)> {
    let (s, _) = multispace0(s)?;
    separated_pair(u32, tag(" "), parse_color)(s)
}

fn parse_subset(s: &str) -> IResult<&str, Subset> {
    let (s, cubes) = separated_list1(tag(","), parse_cube)(s)?;
    IResult::Ok((s, Subset { cubes }))
}

fn parse_game(s: &str) -> IResult<&str, Game> {
    let game_id_parser = delimited(tag("Game "), u32, tag(":"));
    let subsets_parser = separated_list1(tag(";"), parse_subset);
    let parser = pair(game_id_parser, subsets_parser);

    let mut x = map(parser, |(id, subsets)| Game { id, subsets });
    x(s)
}

impl FromStr for Game {
    type Err = nom::error::Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_game(s).finish() {
            Ok((_, game)) => Ok(game),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

impl Game {
    fn is_possible(&self, color: Color, max_number: u32) -> bool {
        self.subsets.iter().all(|subset| {
            !subset
                .cubes
                .iter()
                .filter(|(_, cube_color)| cube_color == &color)
                .any(|(cube_count, _)| cube_count > &max_number)
        })
    }

    fn min_by_color(&self, color: Color) -> u32 {
        self.subsets
            .iter()
            .map(|subset| {
                subset
                    .cubes
                    .iter()
                    .filter(|(_, cube_color)| cube_color == &color)
                    .max_by(|a, b| a.0.cmp(&b.0))
                    .map(|(cube_count, _)| *cube_count)
                    .unwrap_or(0)
            })
            .max()
            .unwrap_or(0)
    }
}

fn main() {
    let input = include_str!("../data/input.txt");
    let games = input
        .lines()
        .map(|line| line.parse::<Game>().unwrap())
        .collect::<Vec<_>>();

    let sum_of_ids = games
        .iter()
        .filter(|game| game.is_possible(Color::Blue, 14))
        .filter(|game| game.is_possible(Color::Green, 13))
        .filter(|game| game.is_possible(Color::Red, 12))
        .map(|game| game.id)
        .sum::<u32>();

    println!("The sum of the valid game IDs is {sum_of_ids}");

    let power_of_cubes = games
        .iter()
        .map(|game| {
            game.min_by_color(Color::Blue)
                * game.min_by_color(Color::Green)
                * game.min_by_color(Color::Red)
        })
        .sum::<u32>();

    println!("The sum of the power of cubes is {power_of_cubes}");
}
