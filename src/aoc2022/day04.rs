use itertools::Itertools;
use std::str::FromStr;
use std::string::ParseError;

type AssignmentRange = (usize, usize);
type AssignmentPair = (AssignmentRange, AssignmentRange);

struct Assignments {
    worker1: AssignmentRange,
    worker2: AssignmentRange,
}

impl Assignments {
    fn partial_overlap(&self) -> bool {
        self.worker1.0 <= self.worker2.1 && self.worker1.1 >= self.worker2.0
    }

    fn complete_overlap(&self) -> bool {
        (self.worker1.0 <= self.worker2.0 && self.worker1.1 >= self.worker2.1)
            || (self.worker2.0 <= self.worker1.0 && self.worker2.1 >= self.worker1.1)
    }
}

impl From<String> for Assignments {
    fn from(line: String) -> Self {
        line.split(",")
            .map(|part| {
                part.split("-")
                    .map(|num| num.parse::<usize>().unwrap())
                    .collect_tuple::<AssignmentRange>()
                    .unwrap()
            })
            .collect_tuple::<AssignmentPair>()
            .unwrap()
            .into()
    }
}

impl From<AssignmentPair> for Assignments {
    fn from(tuple: AssignmentPair) -> Self {
        Assignments {
            worker1: tuple.0,
            worker2: tuple.1,
        }
    }
}

fn part1(input: Vec<String>) -> usize {
    input
        .into_iter()
        .map_into::<Assignments>()
        .filter_map(|assignments| assignments.complete_overlap().then(|| assignments))
        .count()
}

fn part2(input: Vec<String>) -> usize {
    input
        .into_iter()
        .map_into::<Assignments>()
        .filter_map(|assignments| assignments.partial_overlap().then(|| assignments))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lib;

    #[test]
    fn part1_sample_test() {
        let input = lib::parse_input("aoc2022/res/day04_sample.txt");
        assert_eq!(part1(input), 2);
    }

    #[test]
    fn part1_test() {
        let input = lib::parse_input("aoc2022/res/day04.txt");
        assert_eq!(part1(input), 588);
    }

    #[test]
    fn part2_sample_test() {
        let input = lib::parse_input("aoc2022/res/day04_sample.txt");
        assert_eq!(part2(input), 4);
    }

    #[test]
    fn part2_test() {
        let input = lib::parse_input("aoc2022/res/day04.txt");
        assert_eq!(part2(input), 911);
    }
}
