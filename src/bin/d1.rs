use std::{error::Error, fs::File, io::Read};

const NUMS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];
const NUMS_REVERSED: [&str; 9] = [
    "eno", "owt", "eerht", "ruof", "evif", "xis", "neves", "thgie", "enin",
];

fn find_digit(line: &str) -> (usize, u32) {
    line.chars()
        .enumerate()
        .find(|(_, c)| c.is_ascii_digit())
        .map(|(i, c)| (i, c as u32 - b'0' as u32))
        .unwrap_or((usize::MAX, u32::MAX))
}

fn find_digit_string(line: &str, nums: &[&str]) -> (usize, u32) {
    nums.iter()
        .enumerate()
        .filter_map(|(i, n)| line.find(n).map(|v| (v, i as u32 + 1)))
        .min()
        .unwrap_or((usize::MAX, u32::MAX))
}

fn parse_line(line: &str) -> u32 {
    let line_reversed: String = line.chars().rev().collect();
    let first_digit = find_digit(line).min(find_digit_string(line, &NUMS)).1;
    let last_digit = find_digit(&line_reversed)
        .min(find_digit_string(&line_reversed, &NUMS_REVERSED))
        .1;
    first_digit * 10 + last_digit
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut content = String::new();
    File::open("in/1r.txt")?.read_to_string(&mut content)?;
    println!("Total: {}", content.lines().map(parse_line).sum::<u32>());
    Ok(())
}
