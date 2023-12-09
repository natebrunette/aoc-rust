// https://adventofcode.com/2023/day/9

use itertools::Itertools;

pub fn part1(input: Vec<String>) -> i32 {
    input
        .into_iter()
        .map(string_to_int)
        .map(|nums| {
            calculate_sequences(nums)
                .into_iter()
                .map(|nums| nums[nums.len() - 1])
                .collect::<Vec<_>>()
        })
        .map(|nums| nums.iter().rfold(0, |acc, num| acc + num))
        .sum()
}

pub fn part2(input: Vec<String>) -> i32 {
    input
        .into_iter()
        .map(string_to_int)
        .map(|nums| {
            calculate_sequences(nums)
                .into_iter()
                .map(|nums| nums[0])
                .collect::<Vec<_>>()
        })
        .map(|nums| nums.iter().rfold(0, |acc, &num| num - acc))
        .sum()
}

fn string_to_int(line: String) -> Vec<i32> {
    line.split_whitespace()
        .map(|num| num.parse::<i32>().unwrap())
        .collect()
}

fn calculate_sequences(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut iterations = vec![nums.clone()];
    while !nums.iter().all(|&num| num == 0) {
        nums = nums.iter().tuple_windows().map(|(a, b)| b - a).collect();
        iterations.push(nums.clone());
    }

    iterations
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::parse_input;

    #[test]
    fn part1_sample_test() {
        let input = parse_input("aoc2023/res/day09_sample.txt");
        assert_eq!(part1(input), 114);
    }

    #[test]
    fn part1_test() {
        let input = parse_input("aoc2023/res/day09.txt");
        assert_eq!(part1(input), 1772145754);
    }

    #[test]
    fn part2_sample_test() {
        let input = parse_input("aoc2023/res/day09_sample.txt");
        assert_eq!(part2(input), 2);
    }

    #[test]
    fn part2_test() {
        let input = parse_input("aoc2023/res/day09.txt");
        assert_eq!(part2(input), 867);
    }
}
