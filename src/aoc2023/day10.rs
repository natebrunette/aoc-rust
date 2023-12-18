// https://adventofcode.com/2023/day/10

use crate::aoc2023::day10::Direction::{Down, Left, Right, Up};
use crate::aoc2023::day10::PipeType::{
    DownLeft, DownRight, Ground, Horizontal, Starting, UpLeft, UpRight, Vertical,
};
use crate::graph::Graph;
use std::collections::HashMap;

pub fn part1(input: Vec<String>) -> u64 {
    let grid = input
        .into_iter()
        .map(|line| line.chars().collect())
        .collect();
    let graph = build_graph(&grid);
    let starting_node = get_starting_node(&grid);

    let mut data = HashMap::new();
    graph.bfs(starting_node, |node, depth| {
        data.insert(node.id.clone(), depth);
    });

    data.into_values().max().unwrap()
}

pub fn part2(input: Vec<String>) -> i32 {
    0
}

#[derive(Eq, PartialEq, Copy, Clone, Hash)]
enum PipeType {
    Starting,
    Vertical,
    Horizontal,
    DownRight,
    DownLeft,
    UpRight,
    UpLeft,
    Ground,
}

impl From<char> for PipeType {
    fn from(value: char) -> Self {
        match value {
            'S' => Starting,
            '|' => Vertical,
            '-' => Horizontal,
            'L' => DownRight,
            'J' => DownLeft,
            'F' => UpRight,
            '7' => UpLeft,
            '.' => Ground,
            _ => unimplemented!("Couldn't parse pipe type"),
        }
    }
}

#[derive(PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl PipeType {
    fn directions(&self) -> Vec<Direction> {
        match self {
            Starting => vec![Up, Right, Down, Left],
            Vertical => vec![Up, Down],
            Horizontal => vec![Right, Left],
            DownRight => vec![Up, Right],
            DownLeft => vec![Up, Left],
            UpRight => vec![Down, Right],
            UpLeft => vec![Down, Left],
            Ground => vec![],
        }
    }
}

fn build_graph(grid: &Vec<Vec<char>>) -> Graph<(usize, usize), PipeType> {
    let graph = Graph::new();
    for (row, rows) in grid.into_iter().enumerate() {
        for (col, val) in rows.into_iter().enumerate() {
            let pipe_type = PipeType::from(val.clone());
            if pipe_type == Ground {
                continue;
            }

            let node = graph.add_node_by_id((row, col), pipe_type);

            let mut neighbors = vec![];
            if row > 0 && pipe_type.directions().contains(&Up) {
                neighbors.push((row - 1, col, Up));
            }

            if col > 0 && pipe_type.directions().contains(&Left) {
                neighbors.push((row, col - 1, Left));
            }

            for (neighbor_r, neighbor_c, direction) in neighbors.into_iter() {
                if let Some(neighbor) = graph.get_node((neighbor_r, neighbor_c)) {
                    if direction == Up && neighbor.data.directions().contains(&Down) {
                        graph.add_double_edge(neighbor.id, node.id);
                    }

                    if direction == Left && neighbor.data.directions().contains(&Right) {
                        graph.add_double_edge(neighbor.id, node.id);
                    }
                }
            }
        }
    }

    graph
}

fn get_starting_node(grid: &Vec<Vec<char>>) -> (usize, usize) {
    for (row, rows) in grid.iter().enumerate() {
        for (col, &c) in rows.iter().enumerate() {
            if c == 'S' {
                return (row, col);
            }
        }
    }

    (0, 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::parse_input;

    #[test]
    fn part1_sample_test() {
        let input = parse_input("aoc2023/res/day10_sample.txt");
        assert_eq!(part1(input), 8);
    }

    #[test]
    fn part1_test() {
        let input = parse_input("aoc2023/res/day10.txt");
        assert_eq!(part1(input), 6725);
    }

    #[test]
    fn part2_sample_test() {
        let input = parse_input("aoc2023/res/day10_sample.txt");
        assert_eq!(part2(input), 0);
    }

    #[test]
    fn part2_test() {
        let input = parse_input("aoc2023/res/day10.txt");
        assert_eq!(part2(input), 0);
    }
}
