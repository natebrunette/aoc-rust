use crate::shared;
use itertools::Itertools;

pub fn part1(input: Vec<String>) -> usize {
    shared::group_on_empty(&input)
        .iter()
        .map(|group| shared::vec_to_int(group))
        .map(|vec| vec.iter().sum())
        .max()
        .unwrap()
}

pub fn part2(input: Vec<String>) -> usize {
    shared::group_on_empty(&input)
        .iter()
        .map(|group| shared::vec_to_int(group))
        .map(|vec| vec.iter().sum::<usize>())
        .sorted()
        .rev()
        .take(3)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::parse_input;

    #[test]
    fn part1_sample_test() {
        let input = parse_input("aoc2022/res/day01_sample.txt");
        assert_eq!(part1(input), 24000);
    }

    #[test]
    fn part1_test() {
        let input = parse_input("aoc2022/res/day01.txt");
        assert_eq!(part1(input), 74394);
    }

    #[test]
    fn part2_sample_test() {
        let input = parse_input("aoc2022/res/day01_sample.txt");
        assert_eq!(part2(input), 45000);
    }

    #[test]
    fn part2_test() {
        let input = parse_input("aoc2022/res/day01.txt");
        assert_eq!(part2(input), 212836);
    }
}
