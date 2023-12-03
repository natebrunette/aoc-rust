// https://adventofcode.com/2023/day/2

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, space1};
use nom::combinator::{map, map_res};
use nom::multi::separated_list1;
use nom::sequence::{preceded, separated_pair};
use nom::IResult;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Display;
use std::ops::Mul;
use std::str::FromStr;

pub fn part1(input: Vec<String>) -> i32 {
    input
        .into_iter()
        .map(|line| game_parser(&line).unwrap().1)
        .filter(Game::valid)
        .map(|game| game.id)
        .sum::<i32>()
}

pub fn part2(input: Vec<String>) -> i32 {
    input
        .into_iter()
        .map(|line| game_parser(&line).unwrap().1)
        .map(|game| game.get_max_colors())
        .map(|colors| {
            colors
                .into_iter()
                .map(|revealed| revealed.value())
                .reduce(Mul::mul)
                .unwrap()
        })
        .sum::<i32>()
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum Revealed {
    RED(i32),
    GREEN(i32),
    BLUE(i32),
}

#[derive(Debug)]
struct Game {
    id: i32,
    sets: Vec<Vec<Revealed>>,
}

impl Revealed {
    fn value(&self) -> i32 {
        match self {
            Revealed::RED(val) | Revealed::GREEN(val) | Revealed::BLUE(val) => *val,
        }
    }
}

impl Display for Revealed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Revealed::RED(_) => "red",
            Revealed::GREEN(_) => "green",
            Revealed::BLUE(_) => "blue",
        };
        write!(f, "{}", str)
    }
}

impl From<(i32, &str)> for Revealed {
    fn from(value: (i32, &str)) -> Self {
        match value {
            (count, "red") => Revealed::RED(count),
            (count, "blue") => Revealed::BLUE(count),
            (count, "green") => Revealed::GREEN(count),
            _ => unimplemented!("Color not supported"),
        }
    }
}

impl Ord for Revealed {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for Revealed {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Revealed::RED(a), Revealed::RED(b)) => a.partial_cmp(b),
            (Revealed::GREEN(a), Revealed::GREEN(b)) => a.partial_cmp(b),
            (Revealed::BLUE(a), Revealed::BLUE(b)) => a.partial_cmp(b),
            _ => None,
        }
    }
}

impl Game {
    fn valid(&self) -> bool {
        self.sets.iter().all(|set| {
            set.into_iter().all(|revealed| match revealed {
                Revealed::RED(num) => *num <= 12,
                Revealed::GREEN(num) => *num <= 13,
                Revealed::BLUE(num) => *num <= 14,
            })
        })
    }

    fn get_max_colors(&self) -> Vec<Revealed> {
        let max_colors = HashMap::from([
            (Revealed::RED(1).to_string(), Revealed::RED(1)),
            (Revealed::GREEN(1).to_string(), Revealed::GREEN(1)),
            (Revealed::BLUE(1).to_string(), Revealed::BLUE(1)),
        ]);

        self.sets
            .iter()
            .fold(max_colors, |mut acc, set| {
                set.into_iter().for_each(|revealed| {
                    acc.entry(revealed.to_string())
                        .and_modify(|current_revealed| {
                            *current_revealed = current_revealed.clone().max(revealed.clone())
                        });
                });

                acc
            })
            .into_values()
            .collect()
    }
}

fn digit_parser(input: &str) -> IResult<&str, i32> {
    map_res(digit1, i32::from_str)(input)
}

fn game_id_parser(input: &str) -> IResult<&str, i32> {
    preceded(tag("Game "), digit_parser)(input)
}

fn color_parser(input: &str) -> IResult<&str, &str> {
    alt((tag("red"), tag("green"), tag("blue")))(input)
}

fn revealed_parser(input: &str) -> IResult<&str, Revealed> {
    map(
        separated_pair(digit_parser, space1, color_parser),
        Revealed::from,
    )(input)
}

fn set_parser(input: &str) -> IResult<&str, Vec<Revealed>> {
    separated_list1(tag(", "), revealed_parser)(input)
}

fn game_parser(input: &str) -> IResult<&str, Game> {
    let (input, id) = game_id_parser(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, sets) = separated_list1(tag("; "), set_parser)(input)?;

    Ok((input, Game { id, sets }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::parse_input;

    #[test]
    fn part1_sample_test() {
        let input = parse_input("aoc2023/res/day02_sample.txt");
        assert_eq!(part1(input), 8);
    }

    #[test]
    fn part1_test() {
        let input = parse_input("aoc2023/res/day02.txt");
        assert_eq!(part1(input), 1867);
    }

    #[test]
    fn part2_sample_test() {
        let input = parse_input("aoc2023/res/day02_sample.txt");
        assert_eq!(part2(input), 2286);
    }

    #[test]
    fn part2_test() {
        let input = parse_input("aoc2023/res/day02.txt");
        assert_eq!(part2(input), 84538);
    }
}
