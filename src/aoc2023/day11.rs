// https://adventofcode.com/2023/day/11

use crate::matrix::Matrix;
use crate::point::Point;
use crate::range::Range;
use itertools::Itertools;
use std::collections::HashSet;

pub fn part1(input: Vec<String>) -> isize {
    calc(input, 1)
}

pub fn part2(input: Vec<String>, increase_by: isize) -> isize {
    calc(input, increase_by - 1)
}

fn calc(input: Vec<String>, increase_by: isize) -> isize {
    let expanded_matrix = build_and_expand_grid(input);
    expanded_matrix
        .matrix
        .into_iter()
        .enumerate()
        .flat_map(|(row, rows)| {
            rows.into_iter().enumerate().filter_map(move |(col, c)| {
                if c == '#' {
                    Some((row as isize, col as isize))
                } else {
                    None
                }
            })
        })
        .map(|coords| Point(coords))
        .combinations(2)
        .map(|points| {
            let rows_between = calc_between(
                get_row_range(&points[0], &points[1]),
                &expanded_matrix.empty_rows,
                increase_by,
            );
            let cols_between = calc_between(
                get_col_range(&points[0], &points[1]),
                &expanded_matrix.empty_cols,
                increase_by,
            );

            points[0].manhattan(&points[1]) + rows_between + cols_between
        })
        .sum::<_>()
}

struct ExpandedMatrix {
    matrix: Matrix<char>,
    empty_rows: HashSet<isize>,
    empty_cols: HashSet<isize>,
}

fn calc_between(range: Range<isize>, empty: &HashSet<isize>, increase_by: isize) -> isize {
    empty
        .iter()
        .map(|&index| {
            if range.0.contains(&index) {
                increase_by
            } else {
                0
            }
        })
        .sum()
}

fn get_row_range(a: &Point<isize>, b: &Point<isize>) -> Range<isize> {
    Range::from((a.0 .0, b.0 .0))
}

fn get_col_range(a: &Point<isize>, b: &Point<isize>) -> Range<isize> {
    Range::from((a.0 .1, b.0 .1))
}

fn build_and_expand_grid(input: Vec<String>) -> ExpandedMatrix {
    let grid = input
        .into_iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();
    let matrix = Matrix(grid);

    let empty_rows = get_expanded(&matrix);
    let empty_cols = get_expanded(&matrix.rotate());

    ExpandedMatrix {
        matrix,
        empty_rows,
        empty_cols,
    }
}

fn get_expanded(matrix: &Matrix<char>) -> HashSet<isize> {
    matrix
        .0
        .iter()
        .enumerate()
        .filter_map(|(row, chars)| {
            if chars.iter().all(|&c| c == '.') {
                Some(row as isize)
            } else {
                None
            }
        })
        .collect::<_>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::parse_input;

    #[test]
    fn part1_sample_test() {
        let input = parse_input("aoc2023/res/day11_sample.txt");
        assert_eq!(part1(input), 374);
    }

    #[test]
    fn part1_test() {
        let input = parse_input("aoc2023/res/day11.txt");
        assert_eq!(part1(input), 9233514);
    }

    #[test]
    fn part2_sample_1_test() {
        let input = parse_input("aoc2023/res/day11_sample.txt");
        assert_eq!(part2(input, 10), 1030);
    }

    #[test]
    fn part2_sample_2_test() {
        let input = parse_input("aoc2023/res/day11_sample.txt");
        assert_eq!(part2(input, 100), 8410);
    }

    #[test]
    fn part2_test() {
        let input = parse_input("aoc2023/res/day11.txt");
        assert_eq!(part2(input, 1000000), 363293506944);
    }
}
