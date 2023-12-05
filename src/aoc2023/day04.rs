// https://adventofcode.com/2023/day/4

use std::collections::{HashMap, HashSet};

pub fn part1(input: Vec<String>) -> i32 {
    input
        .into_iter()
        .map(calculate_winning_numbers)
        .map(|wins| if wins == 0 { 0 } else { 1 << (wins - 1) })
        .sum::<i32>()
}

pub fn part2(input: Vec<String>) -> i32 {
    let winning_numbers = input.into_iter().map(calculate_winning_numbers);
    let mut entries_map = (0..winning_numbers.len())
        .into_iter()
        .map(|index| (index, 1))
        .collect::<HashMap<_, _>>();
    winning_numbers.enumerate().for_each(|(index, wins)| {
        let times = entries_map.get(&index).unwrap().clone();
        for i in (index + 1)..=(index + wins) {
            entries_map.entry(i).and_modify(|count| *count += times);
        }
    });

    entries_map.into_values().sum::<_>()
}

fn calculate_winning_numbers(line: String) -> usize {
    line.split(":")
        .nth(1)
        .unwrap()
        .split("|")
        .map(|part| {
            part.split(" ")
                .filter(|str| !str.is_empty())
                .map(|char| char.trim().parse::<i32>().unwrap())
                .collect::<HashSet<_>>()
        })
        .reduce(|acc, element| acc.intersection(&element).cloned().collect())
        .unwrap()
        .len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::parse_input;

    #[test]
    fn part1_sample_test() {
        let input = parse_input("aoc2023/res/day04_sample.txt");
        assert_eq!(part1(input), 13);
    }

    #[test]
    fn part1_test() {
        let input = parse_input("aoc2023/res/day04.txt");
        assert_eq!(part1(input), 24733);
    }

    #[test]
    fn part2_sample_test() {
        let input = parse_input("aoc2023/res/day04_sample.txt");
        assert_eq!(part2(input), 30);
    }

    #[test]
    fn part2_test() {
        let input = parse_input("aoc2023/res/day04.txt");
        assert_eq!(part2(input), 5422730);
    }
}
