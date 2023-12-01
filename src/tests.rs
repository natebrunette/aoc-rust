use std::env::current_dir;
use std::fs;
use std::path::PathBuf;

pub fn parse_input(path: &str) -> Vec<String> {
    fs::read_to_string(get_path(path))
        .map(|string| {
            string
                .split("\n")
                .map(|string| string.to_string())
                .filter(|line| !line.is_empty())
                .collect()
        })
        .expect("Could not parse input file")
}

fn get_path(path: &str) -> PathBuf {
    let mut current = cwd();
    current.push(format!("src/{}", path));

    current
}

fn cwd() -> PathBuf {
    current_dir().unwrap()
}
