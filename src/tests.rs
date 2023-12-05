use std::env::current_dir;
use std::fs;
use std::path::PathBuf;

pub fn get_input(path: &str) -> String {
    fs::read_to_string(get_path(path)).expect("Could not read input file")
}

pub fn parse_input(path: &str) -> Vec<String> {
    get_input(path)
        .lines()
        .filter(|&line| !line.is_empty())
        .map(|str| str.to_string())
        .collect()
}

fn get_path(path: &str) -> PathBuf {
    let mut current = cwd();
    current.push(format!("src/{}", path));

    current
}

fn cwd() -> PathBuf {
    current_dir().unwrap()
}
