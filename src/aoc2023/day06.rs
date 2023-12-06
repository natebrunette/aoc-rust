// https://adventofcode.com/2023/day/6

pub fn part1(input: Vec<String>) -> u64 {
    let times = read_line(0, &input);
    let distances = read_line(1, &input);

    times
        .into_iter()
        .zip(distances)
        .map(find_winning_times)
        .reduce(|acc, num| acc * num)
        .unwrap()
}

pub fn part2(input: Vec<String>) -> u64 {
    let no_spaces: Vec<_> = input
        .clone()
        .into_iter()
        .map(|mut string| {
            string.retain(|c| !c.is_whitespace());
            string
        })
        .collect();
    let time = read_line(0, &no_spaces)[0];
    let distance = read_line(1, &no_spaces)[0];

    find_winning_times((time, distance))
}

fn read_line(line: usize, input: &Vec<String>) -> Vec<u64> {
    input
        .iter()
        .nth(line)
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|str| str.parse::<u64>().unwrap())
        .collect()
}

fn find_winning_times(record: (u64, u64)) -> u64 {
    let (time, distance) = record;
    let mut low = 1;
    let mut high = time - 1;

    while (time - low) * low <= distance {
        low += 1;
    }

    while (time - high) * high <= distance {
        high -= 1;
    }

    high - low + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::parse_input;

    #[test]
    fn part1_sample_test() {
        let input = parse_input("aoc2023/res/day06_sample.txt");
        assert_eq!(part1(input), 288);
    }

    #[test]
    fn part1_test() {
        let input = parse_input("aoc2023/res/day06.txt");
        assert_eq!(part1(input), 512295);
    }

    #[test]
    fn part2_sample_test() {
        let input = parse_input("aoc2023/res/day06_sample.txt");
        assert_eq!(part2(input), 71503);
    }

    #[test]
    fn part2_test() {
        let input = parse_input("aoc2023/res/day06.txt");
        assert_eq!(part2(input), 36530883);
    }
}
