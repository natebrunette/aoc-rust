use itertools::Itertools;
use std::collections::HashSet;

fn part1(input: Vec<String>) -> usize {
    input
        .iter()
        .map(|line| line.split_at(line.len() / 2))
        .map(|(left, right)| {
            (
                HashSet::<char>::from_iter(left.chars()),
                HashSet::<char>::from_iter(right.chars()),
            )
        })
        .map(|(left, right)| {
            left.intersection(&right)
                .cloned()
                .collect::<HashSet<char>>()
        })
        .map(|set| set.iter().fold(0, |acc, c| acc + to_int(c)))
        .sum()
}

fn part2(input: Vec<String>) -> usize {
    input
        .iter()
        .map(|line| HashSet::<char>::from_iter(line.chars()))
        .chunks(3)
        .into_iter()
        .map(|chunk| {
            chunk.fold(HashSet::<char>::new(), |acc, set| {
                if acc.is_empty() {
                    return set;
                }

                acc.intersection(&set).cloned().collect()
            })
        })
        .map(|set| set.iter().fold(0, |acc, c| acc + to_int(c)))
        .sum()
}

fn to_int(c: &char) -> usize {
    let int = c.clone() as usize;
    return if int >= 97 { int - 96 } else { int - 38 };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lib;

    #[test]
    fn part1_sample_test() {
        let input = lib::parse_input("aoc2022/res/day03_sample.txt");
        assert_eq!(part1(input), 157);
    }

    #[test]
    fn part1_test() {
        let input = lib::parse_input("aoc2022/res/day03.txt");
        assert_eq!(part1(input), 7428);
    }

    #[test]
    fn part2_sample_test() {
        let input = lib::parse_input("aoc2022/res/day03_sample.txt");
        assert_eq!(part2(input), 70);
    }

    #[test]
    fn part2_test() {
        let input = lib::parse_input("aoc2022/res/day03.txt");
        assert_eq!(part2(input), 2650);
    }
}
