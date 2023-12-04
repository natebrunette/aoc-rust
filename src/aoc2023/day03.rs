// https://adventofcode.com/2023/day/3

use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub fn part1(input: Vec<String>) -> i32 {
    let matrix = create_matrix(input);

    let symbol_locations = matrix
        .iter()
        .enumerate()
        .flat_map(|(row, line)| {
            line.iter().enumerate().filter_map(move |(col, &char)| {
                if char.is_ascii_digit() || char == '.' {
                    return None;
                }

                Some((row as i32, col as i32))
            })
        })
        .collect::<HashSet<(i32, i32)>>();

    // create references to be accessed from within the moved closure
    let matrix_ref = &matrix;
    let symbol_locations_ref = &symbol_locations;

    let cols = matrix[0].len();
    matrix
        .iter()
        .enumerate()
        .flat_map(|(row, line)| {
            line.iter().enumerate().filter_map(move |(col, _)| {
                if col > 0 && matrix_ref[row][col - 1].is_ascii_digit() {
                    return None;
                }

                if !matrix_ref[row][col].is_ascii_digit() {
                    return None;
                }

                let start = col;
                let mut end = col;
                let mut number_string = "".to_string();
                while end < cols && matrix_ref[row][end].is_ascii_digit() {
                    number_string.push(matrix_ref[row][end]);
                    end += 1;
                }

                end -= 1;

                let row_range = ((row as i32) - 1)..=((row as i32) + 1);
                let col_range = ((start as i32) - 1)..=((end as i32) + 1);

                let lookup = row_range
                    .cartesian_product(col_range)
                    .collect::<HashSet<(i32, i32)>>();
                if symbol_locations_ref.intersection(&lookup).count() != 0 {
                    return number_string.parse::<i32>().ok();
                }

                None
            })
        })
        .sum::<i32>()
}

pub fn part2(input: Vec<String>) -> i32 {
    let matrix = create_matrix(input);
    let potential_gear_locations = matrix
        .iter()
        .enumerate()
        .flat_map(|(row, line)| {
            line.iter().enumerate().filter_map(move |(col, &char)| {
                if char == '*' {
                    return Some((row as i32, col as i32));
                }

                None
            })
        })
        .collect::<HashSet<(i32, i32)>>();

    // create references to be accessed from within the moved closure
    let matrix_ref = &matrix;
    let potential_gear_locations_ref = &potential_gear_locations;

    let cols = matrix[0].len();
    matrix
        .iter()
        .enumerate()
        .flat_map(|(row, line)| {
            line.iter().enumerate().filter_map(move |(col, _)| {
                if col > 0 && matrix_ref[row][col - 1].is_ascii_digit() {
                    return None;
                }

                if !matrix_ref[row][col].is_ascii_digit() {
                    return None;
                }

                let start = col;
                let mut end = col;
                let mut number_string = "".to_string();
                while end < cols && matrix_ref[row][end].is_ascii_digit() {
                    number_string.push(matrix_ref[row][end]);
                    end += 1;
                }

                end -= 1;

                let row_range = ((row as i32) - 1)..=((row as i32) + 1);
                let col_range = ((start as i32) - 1)..=((end as i32) + 1);

                let lookup = row_range
                    .cartesian_product(col_range)
                    .collect::<HashSet<(i32, i32)>>();

                let collected = potential_gear_locations_ref
                    .intersection(&lookup)
                    .map(|&location| (location, number_string.parse::<i32>().unwrap()))
                    .collect::<Vec<((i32, i32), i32)>>();

                return if collected.is_empty() {
                    None
                } else {
                    Some(collected)
                };
            })
        })
        .flatten()
        .fold(
            HashMap::<(i32, i32), Vec<i32>>::new(),
            |mut acc: HashMap<(i32, i32), Vec<i32>>, elements| {
                acc.entry(elements.0).or_default().push(elements.1);
                acc
            },
        )
        .into_values()
        .filter(|list| list.len() == 2)
        .map(|list| list.into_iter().reduce(|acc, i| acc * i).unwrap())
        .sum::<i32>()
}

fn create_matrix(input: Vec<String>) -> Vec<Vec<char>> {
    input
        .into_iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::parse_input;

    #[test]
    fn part1_sample_test() {
        let input = parse_input("aoc2023/res/day03_sample.txt");
        assert_eq!(part1(input), 4361);
    }

    #[test]
    fn part1_test() {
        let input = parse_input("aoc2023/res/day03.txt");
        assert_eq!(part1(input), 539433);
    }

    #[test]
    fn part2_sample_test() {
        let input = parse_input("aoc2023/res/day03_sample.txt");
        assert_eq!(part2(input), 467835);
    }

    #[test]
    fn part2_test() {
        let input = parse_input("aoc2023/res/day03.txt");
        assert_eq!(part2(input), 75847567);
    }
}
