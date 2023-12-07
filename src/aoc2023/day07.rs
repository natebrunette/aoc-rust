// https://adventofcode.com/2023/day/7

use crate::aoc2023::day07::HandType::{
    FiveOfAKind, FourOfAKind, FullHouse, HighCard, OnePair, ThreeOfAKind, TwoPair,
};
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::str::FromStr;

pub fn part1(input: Vec<String>) -> i32 {
    input
        .into_iter()
        .map(|str| (&str).parse::<Entry>().unwrap())
        .sorted()
        .enumerate()
        .map(|(index, entry)| entry.bid * ((index as i32) + 1))
        .sum::<i32>()
}

pub fn part2(input: Vec<String>) -> i32 {
    input
        .into_iter()
        .map(|str| str.replace("J", "?"))
        .map(|str| (&str).parse::<Entry>().unwrap())
        .sorted()
        .enumerate()
        .map(|(index, entry)| entry.bid * ((index as i32) + 1))
        .sum::<i32>()
}

#[derive(Debug)]
struct Entry {
    hand: Hand,
    bid: i32,
}

impl Eq for Entry {}
impl PartialEq<Self> for Entry {
    fn eq(&self, other: &Self) -> bool {
        self.hand == other.hand && self.bid == other.bid
    }
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hand.cmp(&other.hand)
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.hand.partial_cmp(&other.hand)
    }
}

impl FromStr for Entry {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input: Vec<_> = s.split_whitespace().collect();
        let hand = Hand::new(input[0].chars().map(|c| c.into()).collect::<Vec<Card>>());
        let bid = input[1].parse::<i32>().unwrap();

        Ok(Entry { hand, bid })
    }
}

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    hand_type: HandType,
}

impl Eq for Hand {}
impl PartialEq<Self> for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(&other).unwrap()
    }
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Less => Some(Ordering::Less),
            Ordering::Greater => Some(Ordering::Greater),
            Ordering::Equal => self
                .cards
                .iter()
                .zip(other.cards.iter())
                .find_map(|(a, b)| match a.cmp(b) {
                    Ordering::Equal => None,
                    other => Some(other),
                })
                .or(Some(Ordering::Equal)),
        }
    }
}

impl Hand {
    fn new(cards: Vec<Card>) -> Self {
        let hand_type: HandType = (&cards).into();

        Hand { cards, hand_type }
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl From<&Vec<Card>> for HandType {
    fn from(cards: &Vec<Card>) -> HandType {
        let mut counts = cards
            .iter()
            .fold(HashMap::<Card, i32>::new(), |mut acc, &card| {
                *acc.entry(card).or_default() += 1;
                acc
            });
        let num_jokers = counts.remove(&Card::JOKER).unwrap_or(0);

        match counts.values().max().unwrap_or(&0) {
            5 => FiveOfAKind,
            4 => {
                if num_jokers >= 1 {
                    FiveOfAKind
                } else {
                    FourOfAKind
                }
            }
            3 => match num_jokers {
                2 => FiveOfAKind,
                1 => FourOfAKind,
                _ => {
                    if counts.values().contains(&2) {
                        FullHouse
                    } else {
                        ThreeOfAKind
                    }
                }
            },
            2 => match num_jokers {
                3 => FiveOfAKind,
                2 => FourOfAKind,
                1 => {
                    if counts.values().filter(|&&count| count == 2).count() == 2 {
                        FullHouse
                    } else {
                        ThreeOfAKind
                    }
                }
                _ => {
                    if counts.values().filter(|&&count| count == 2).count() == 2 {
                        TwoPair
                    } else {
                        OnePair
                    }
                }
            },
            1 => match num_jokers {
                4 => FiveOfAKind,
                3 => FourOfAKind,
                2 => ThreeOfAKind,
                1 => OnePair,
                _ => HighCard,
            },
            _ => match num_jokers {
                5 => FiveOfAKind,
                _ => unimplemented!("Could not determine hand type"),
            },
        }
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Hash, Debug)]
enum Card {
    JOKER,
    ONE,
    TWO,
    THREE,
    FOUR,
    FIVE,
    SIX,
    SEVEN,
    EIGHT,
    NINE,
    TEN,
    J,
    Q,
    K,
    A,
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            'A' => Card::A,
            'K' => Card::K,
            'Q' => Card::Q,
            'J' => Card::J,
            'T' => Card::TEN,
            '9' => Card::NINE,
            '8' => Card::EIGHT,
            '7' => Card::SEVEN,
            '6' => Card::SIX,
            '5' => Card::FIVE,
            '4' => Card::FOUR,
            '3' => Card::THREE,
            '2' => Card::TWO,
            '1' => Card::ONE,
            '?' => Card::JOKER,
            _ => unimplemented!("Could not parse card"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::parse_input;

    #[test]
    fn part1_sample_test() {
        let input = parse_input("aoc2023/res/day07_sample.txt");
        assert_eq!(part1(input), 6440);
    }

    #[test]
    fn part1_test() {
        let input = parse_input("aoc2023/res/day07.txt");
        assert_eq!(part1(input), 250602641);
    }

    #[test]
    fn part2_sample_test() {
        let input = parse_input("aoc2023/res/day07_sample.txt");
        assert_eq!(part2(input), 5905);
    }

    #[test]
    fn part2_test() {
        let input = parse_input("aoc2023/res/day07.txt");
        assert_eq!(part2(input), 251037509);
    }
}
