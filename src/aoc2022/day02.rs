use crate::aoc2022::day02::Move::{Paper, Rock, Scissors};
use crate::aoc2022::day02::Outcome::{Draw, Lose, Win};
use itertools::Itertools;
use std::ops::{Add, Sub};

#[derive(Clone)]
enum Outcome {
    Lose,
    Draw,
    Win,
}

impl Outcome {
    fn value(&self) -> usize {
        (self.clone() as usize) * 3
    }
}

impl Add<Move> for Outcome {
    type Output = Move;

    fn add(self, opp_move: Move) -> Self::Output {
        match (self, opp_move) {
            (Win, Rock) => Paper,
            (Win, Paper) => Scissors,
            (Win, Scissors) => Rock,
            (Lose, Rock) => Scissors,
            (Lose, Paper) => Rock,
            (Lose, Scissors) => Paper,
            (Draw, Rock) => Rock,
            (Draw, Paper) => Paper,
            (Draw, Scissors) => Scissors,
        }
    }
}

impl From<&str> for Outcome {
    fn from(item: &str) -> Self {
        match item {
            "X" => Lose,
            "Y" => Draw,
            "Z" => Win,
            _ => panic!("Could not create outcome from string"),
        }
    }
}

#[derive(Clone)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn value(&self) -> usize {
        (self.clone() as usize) + 1
    }
}

impl Sub<Move> for Move {
    type Output = Outcome;

    fn sub(self, opp_move: Move) -> Self::Output {
        match (self, opp_move) {
            (Rock, Rock) => Draw,
            (Rock, Paper) => Lose,
            (Rock, Scissors) => Win,
            (Paper, Rock) => Win,
            (Paper, Paper) => Draw,
            (Paper, Scissors) => Lose,
            (Scissors, Rock) => Lose,
            (Scissors, Paper) => Win,
            (Scissors, Scissors) => Draw,
        }
    }
}

impl From<&str> for Move {
    fn from(item: &str) -> Self {
        match item {
            "A" | "X" => Rock,
            "B" | "Y" => Paper,
            "C" | "Z" => Scissors,
            _ => panic!("Could not create move from string"),
        }
    }
}

struct Round {
    me: Move,
    opponent: Move,
}

impl From<(Move, Move)> for Round {
    fn from(item: (Move, Move)) -> Self {
        Round {
            me: item.1,
            opponent: item.0,
        }
    }
}

impl From<(Move, Outcome)> for Round {
    fn from(item: (Move, Outcome)) -> Self {
        Round {
            me: item.1 + item.0.clone(),
            opponent: item.0,
        }
    }
}

impl Round {
    fn outcome(&self) -> Outcome {
        self.me.clone() - self.opponent.clone()
    }

    fn score(&self) -> usize {
        self.me.value() + self.outcome().value()
    }
}

fn part1(input: Vec<String>) -> usize {
    input
        .iter()
        .map(|line| parse_move_line(line))
        .map(|round| round.score())
        .sum()
}

fn part2(input: Vec<String>) -> usize {
    input
        .iter()
        .map(|line| parse_outcome_line(line))
        .map(|round| round.score())
        .sum()
}

fn parse_move_line(line: &String) -> Round {
    line.split(' ')
        .map(|str| str.into())
        .next_tuple::<(Move, Move)>()
        .unwrap()
        .into()
}

fn parse_outcome_line(line: &String) -> Round {
    line.split(' ')
        .next_tuple::<(&str, &str)>()
        .map(|(opponent, outcome)| (Move::from(opponent), Outcome::from(outcome)))
        .unwrap()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lib;

    #[test]
    fn part1_sample_test() {
        let input = lib::parse_input("aoc2022/res/day02_sample.txt");
        assert_eq!(part1(input), 15);
    }

    #[test]
    fn part1_test() {
        let input = lib::parse_input("aoc2022/res/day02.txt");
        assert_eq!(part1(input), 15632);
    }

    #[test]
    fn part2_sample_test() {
        let input = lib::parse_input("aoc2022/res/day02_sample.txt");
        assert_eq!(part2(input), 12);
    }

    #[test]
    fn part2_test() {
        let input = lib::parse_input("aoc2022/res/day02.txt");
        assert_eq!(part2(input), 14416);
    }
}
