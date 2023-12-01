// https://adventofcode.com/2023/day/1

pub fn part1(input: Vec<String>) -> i32 {
    input.into_iter().map(parse_line).sum()
}

pub fn part2(input: Vec<String>) -> i32 {
    input
        .into_iter()
        .map(|line| {
            line.replace("one", "o1ne")
                .replace("two", "t2wo")
                .replace("three", "t3hree")
                .replace("four", "f4our")
                .replace("five", "f5ive")
                .replace("six", "s6ix")
                .replace("seven", "s7even")
                .replace("eight", "e8ight")
                .replace("nine", "n9ine")
        })
        .map(parse_line)
        .sum()
}

fn parse_line(line: String) -> i32 {
    let chars = line
        .chars()
        .filter(|c| c.to_digit(10).is_some())
        .collect::<Vec<char>>();

    let mut string = "".to_string();
    string.push(*chars.first().unwrap());
    string.push(*chars.last().unwrap());

    string.parse::<i32>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::parse_input;

    #[test]
    fn part1_sample_test() {
        let input = parse_input("aoc2023/res/day01_sample.txt");
        assert_eq!(part1(input), 142);
    }

    #[test]
    fn part1_test() {
        let input = parse_input("aoc2023/res/day01.txt");
        assert_eq!(part1(input), 54390);
    }

    #[test]
    fn part2_sample_test() {
        let input = parse_input("aoc2023/res/day01_sample2.txt");
        assert_eq!(part2(input), 281);
    }

    #[test]
    fn part2_test() {
        let input = parse_input("aoc2023/res/day01.txt");
        assert_eq!(part2(input), 54277);
    }
}
