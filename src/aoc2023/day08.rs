// https://adventofcode.com/2023/day/8

use crate::aoc2023::day08::Instruction::{Left, Right};
use itertools::Itertools;
use num::integer::lcm;
use std::collections::{HashMap, VecDeque};

pub fn part1(input: Vec<String>) -> u64 {
    let instructions = get_instructions(&input);
    let map = build_map(&input);
    let mut key = "AAA".to_string();
    let mut count = 0;
    let mut queue = VecDeque::new();

    loop {
        queue.extend(instructions.iter().cloned());

        while let Some(instruction) = queue.pop_front() {
            key = get_next_key(&map, &key, instruction);
            count += 1;

            if key == "ZZZ" {
                return count;
            }
        }
    }
}

pub fn part2(input: Vec<String>) -> u64 {
    let instructions = get_instructions(&input);
    let map = build_map(&input);
    let keys = map
        .keys()
        .filter(|key| key.ends_with("A"))
        .map(|key| key.to_owned())
        .collect::<Vec<String>>();
    let mut queue = VecDeque::new();
    queue.extend(instructions.iter().cloned());

    let mut counts = Vec::new();
    for mut key in keys.into_iter() {
        let mut count = 0;
        queue.clear();
        queue.extend(instructions.iter().cloned());

        while let Some(instruction) = queue.pop_front() {
            key = get_next_key(&map, &key, instruction);
            count += 1;
            if key.ends_with("Z") {
                counts.push(count);
                break;
            }

            if queue.is_empty() {
                queue.extend(instructions.iter().cloned());
            }
        }
    }

    counts.into_iter().reduce(lcm).unwrap()
}

fn get_instructions(input: &Vec<String>) -> Vec<Instruction> {
    input[0].chars().map(|c| c.into()).collect()
}

fn build_map(input: &Vec<String>) -> HashMap<String, (String, String)> {
    input
        .iter()
        .skip(1)
        .map(|line| {
            let (key, tuple) = line.split(" = ").collect_tuple().unwrap();
            let (left, right) = tuple[1..tuple.len() - 1]
                .split(", ")
                .collect_tuple()
                .unwrap();

            (key.to_string(), (left.to_string(), right.to_string()))
        })
        .collect::<HashMap<String, (String, String)>>()
}

fn get_next_key(
    map: &HashMap<String, (String, String)>,
    key: &str,
    instruction: Instruction,
) -> String {
    map.get(&key as &str)
        .map(|(left, right)| match instruction {
            Left => left.to_owned(),
            Right => right.to_owned(),
        })
        .unwrap()
}

#[derive(Copy, Clone)]
enum Instruction {
    Left,
    Right,
}

impl From<char> for Instruction {
    fn from(value: char) -> Self {
        match value {
            'L' => Left,
            'R' => Right,
            _ => unimplemented!("Could not parse instruction"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::parse_input;

    #[test]
    fn part1_sample_test() {
        let input = parse_input("aoc2023/res/day08_sample.txt");
        assert_eq!(part1(input), 2);
    }

    #[test]
    fn part1_sample2_test() {
        let input = parse_input("aoc2023/res/day08_sample2.txt");
        assert_eq!(part1(input), 6);
    }

    #[test]
    fn part1_test() {
        let input = parse_input("aoc2023/res/day08.txt");
        assert_eq!(part1(input), 20569);
    }

    #[test]
    fn part2_sample_test() {
        let input = parse_input("aoc2023/res/day08_sample3.txt");
        assert_eq!(part2(input), 6);
    }

    #[test]
    fn part2_test() {
        let input = parse_input("aoc2023/res/day08.txt");
        assert_eq!(part2(input), 21366921060721);
    }
}
