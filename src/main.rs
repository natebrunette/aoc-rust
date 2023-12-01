use anyhow::{Context, Result};
use chrono::{Datelike, Local};
use clap::Parser;
use reqwest::header::{HeaderMap, HeaderValue, COOKIE};
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Challenge day
    #[arg(short, long)]
    day: Option<i32>,

    /// Challenge year
    #[arg(short, long)]
    year: Option<i32>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    create_challenge_files(args.year, args.day).await?;

    Ok(())
}

async fn create_challenge_files(year: Option<i32>, day: Option<i32>) -> Result<()> {
    let year = year.unwrap_or_else(|| Local::now().year()).to_string();
    let day = day.unwrap_or_else(|| Local::now().day() as i32);
    let day_str = format!("{:02}", day);

    // Directory paths
    let year_dir = format!("aoc{}", year);
    let dir_path = format!("src/{}", year_dir);
    let res_path = format!("{}/res", dir_path);
    let new_year = !Path::new(&dir_path).exists();

    // if we're starting a new year, update the lib file
    if new_year {
        let lib_file = "src/lib.rs";
        let lib_contents = fs::read_to_string(lib_file)?;
        let new_contents = format!("{}\n{}", format!("pub mod {};", year_dir), lib_contents);
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(lib_file)?;

        file.write_all(new_contents.as_bytes())?;
    }

    // Create directories if they don't exist
    fs::create_dir_all(&dir_path)?;
    fs::create_dir_all(&res_path)?;

    // File paths
    let day_file_path = format!("{}/day{}.rs", dir_path, day_str);
    let mod_file_path = format!("{}/mod.rs", dir_path);
    let sample_file_path = format!("{}/day{}_sample.txt", res_path, day_str);
    let input_file_path = format!("{}/day{}.txt", res_path, day_str);

    // Create or truncate files
    let mut day_file = File::create(&day_file_path)?;
    let mut input_file = File::create(&input_file_path)?;
    let _ = File::create(&sample_file_path)?;

    // Write to input file
    writeln!(input_file, "{}", fetch_challenge_input(&year, day).await?)?;

    // Write to day.rs file
    writeln!(day_file, "{}", day_template(&year, &day_str))?;

    // Check if mod.rs exists, create it if it doesn't
    if !Path::new(&mod_file_path).exists() {
        File::create(&mod_file_path)?;
    }

    // Append to mod.rs
    let mut mod_file = fs::OpenOptions::new().append(true).open(&mod_file_path)?;
    writeln!(mod_file, "pub mod day{};", day_str)?;

    Ok(())
}

async fn fetch_challenge_input(year: &str, day: i32) -> Result<String> {
    let cookie = read_cookie()?;
    fetch_input_data(str::trim(&cookie), year, day).await
}

fn read_cookie() -> Result<String> {
    let home_dir = dirs::home_dir().context("Failed to determine home directory")?;
    let cookie_file_path = home_dir.join(".aoc_cookie");

    fs::read_to_string(cookie_file_path).context("Failed to read from ~/.aoc_cookie")
}

async fn fetch_input_data(cookie: &str, year: &str, day: i32) -> Result<String> {
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    let mut request_headers = HeaderMap::new();
    request_headers.insert(COOKIE, HeaderValue::from_str(cookie)?);
    let client = reqwest::Client::builder()
        .default_headers(request_headers)
        .build()?;
    let res = client.get(url).send().await?;

    res.text()
        .await
        .context("Could not complete request to fetch input data")
}

fn day_template(year: &str, day: &str) -> String {
    format!(
        r#"// {link}

pub fn part1(input: Vec<String>) -> i32 {{
    0
}}

pub fn part2(input: Vec<String>) -> i32 {{
    0
}}

#[cfg(test)]
mod tests {{
    use super::*;
    use crate::tests::parse_input;

    #[test]
    fn part1_sample_test() {{
        let input = parse_input("aoc{year}/res/day{day}_sample.txt");
        assert_eq!(part1(input), 0);
    }}

    #[test]
    fn part1_test() {{
        let input = parse_input("aoc{year}/res/day{day}.txt");
        assert_eq!(part1(input), 0);
    }}

    #[test]
    fn part2_sample_test() {{
        let input = parse_input("aoc{year}/res/day{day}_sample.txt");
        assert_eq!(part2(input), 0);
    }}

    #[test]
    fn part2_test() {{
        let input = parse_input("aoc{year}/res/day{day}.txt");
        assert_eq!(part2(input), 0);
    }}
}}
"#,
        link = format!(
            "https://adventofcode.com/{}/day/{}",
            year,
            day.parse::<i32>().unwrap().to_string()
        ),
        year = year,
        day = day
    )
}
