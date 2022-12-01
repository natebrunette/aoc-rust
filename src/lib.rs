use std::env::current_dir;
use std::fs;
use std::path::PathBuf;

pub fn parse_input(path: &str) -> Vec<String> {
    fs::read_to_string(get_path(path))
        .map(|string| {
            string
                .split("\n")
                .map(|string| string.to_string())
                .collect()
        })
        .expect("Could not parse input file")
}

pub fn group_on_empty(input: &Vec<String>) -> Vec<Vec<String>> {
    input
        .split(|line| line == "")
        .map(|slice| slice.to_vec())
        .collect()
}

pub fn vec_to_int(vec: &Vec<String>) -> Vec<usize> {
    vec.iter()
        .map(|string| string.parse::<usize>().unwrap())
        .collect()
}

fn cwd() -> PathBuf {
    current_dir().unwrap()
}

fn get_path(path: &str) -> PathBuf {
    let mut current = cwd();
    current.push(format!("src/{}", path));

    current
}
