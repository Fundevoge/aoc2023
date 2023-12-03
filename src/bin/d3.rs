#![warn(clippy::pedantic)]
#![warn(clippy::style)]

use itertools::Itertools;

use std::{error::Error, fs::File, io::Read, ops::Range};

fn is_symbol(c: &char) -> bool {
    !c.is_ascii_digit() && *c != '.'
}

fn nums_of_line(line: &[char]) -> Vec<(u32, Range<usize>)> {
    let mut line_iter = line.iter().enumerate().peekable();
    let mut last_char = *line_iter.next().unwrap().1;
    let mut num = last_char.to_digit(10).unwrap_or(0);
    let mut range_start = 0;
    let mut range_end = 0;
    let mut nums = vec![];
    for (i, &ch) in line_iter {
        if let Some(d) = ch.to_digit(10) {
            range_end = i;
            if last_char.is_ascii_digit() {
                num = num * 10 + d;
            } else {
                range_start = i;
                num = d;
            }
        } else if last_char.is_ascii_digit() {
            nums.push((num, range_start..range_end + 1));
        }
        last_char = ch;
    }
    if last_char.is_ascii_digit() {
        nums.push((num, range_start..range_end + 1));
    }
    nums
}

fn check_line_for_symbol(line: &[char], r: &Range<usize>) -> bool {
    line.iter().skip(r.start).take(r.len()).any(is_symbol)
        || (r.start > 0 && is_symbol(&line[r.start - 1]))
        || (r.end < line.len() && is_symbol(&line[r.end]))
}

fn has_symbol(symbols: &[Vec<char>], k: usize, r: &Range<usize>) -> bool {
    (k < symbols.len() - 1 && check_line_for_symbol(&symbols[k + 1], r))
        || (k > 0 && check_line_for_symbol(&symbols[k - 1], r))
        || (r.start > 0 && is_symbol(&symbols[k][r.start - 1]))
        || (r.end < symbols[0].len() && is_symbol(&symbols[k][r.end]))
}

fn gears(symbols: &[Vec<(u32, Range<usize>)>], k: usize, i: usize) -> Option<u64> {
    let mut gear_list: Vec<u64> = vec![];
    if k > 0 {
        if let Some((n, _)) = symbols[k - 1].iter().find(|(_, r)| r.contains(&i)) {
            gear_list.push(u64::from(*n));
        } else {
            if let Some((n, _)) = symbols[k - 1].iter().find(|(_, r)| r.end == i) {
                gear_list.push(u64::from(*n));
            }
            if let Some((n, _)) = symbols[k - 1].iter().find(|(_, r)| r.start == i + 1) {
                gear_list.push(u64::from(*n));
            }
        }
    }
    if k < symbols.len() - 1 {
        if let Some((n, _)) = symbols[k + 1].iter().find(|(_, r)| r.contains(&i)) {
            gear_list.push(u64::from(*n));
        } else {
            if let Some((n, _)) = symbols[k + 1].iter().find(|(_, r)| r.end == i) {
                gear_list.push(u64::from(*n));
            }
            if let Some((n, _)) = symbols[k + 1].iter().find(|(_, r)| r.start == i + 1) {
                gear_list.push(u64::from(*n));
            }
        }
    }
    if let Some((n, _)) = symbols[k].iter().find(|(_, r)| r.contains(&i)) {
        gear_list.push(u64::from(*n));
    } else {
        if let Some((n, _)) = symbols[k].iter().find(|(_, r)| r.end == i) {
            gear_list.push(u64::from(*n));
        }
        if let Some((n, _)) = symbols[k].iter().find(|(_, r)| r.start == i + 1) {
            gear_list.push(u64::from(*n));
        }
    }
    if let [gear_a, gear_b] = &gear_list[..] {
        Some(gear_a * gear_b)
    } else {
        None
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut content = String::new();
    File::open("in/3r.txt")?.read_to_string(&mut content)?;
    let symbols = content
        .lines()
        .map(|l| l.chars().collect_vec())
        .collect_vec();
    let all_nums: Vec<Vec<(u32, Range<usize>)>> = symbols
        .iter()
        .map(Vec::as_slice)
        .map(nums_of_line)
        .collect_vec();

    let sum = all_nums
        .iter()
        .enumerate()
        .map(|(k, numbers)| {
            numbers
                .iter()
                .filter(|(_, r)| has_symbol(&symbols, k, r))
                .map(|(n, _)| *n)
                .sum::<u32>()
        })
        .sum::<u32>();
    println!("Total (1): {sum}");
    let sum = symbols
        .iter()
        .enumerate()
        .flat_map(|(k, line)| {
            line.iter()
                .enumerate()
                .filter_map(|(i, c)| (*c == '*').then_some((k, i)))
                .collect_vec()
        })
        .filter_map(|(k, i)| gears(&all_nums, k, i))
        .sum::<u64>();
    println!("Total (2): {sum}");
    Ok(())
}
