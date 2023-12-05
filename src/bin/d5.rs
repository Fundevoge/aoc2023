#![warn(clippy::pedantic)]
#![warn(clippy::style)]

use std::{error::Error, fs::File, io::Read, ops::Range};

use itertools::Itertools;

fn parse_converter(line: &str) -> (Range<i64>, i64) {
    let [dest_start, source_start, len] = line
        .split_ascii_whitespace()
        .map(|d| d.parse().unwrap())
        .collect_vec()[..]
    else {
        panic!("Invalid line {line}")
    };
    (source_start..source_start + len, dest_start - source_start)
}

fn convert(mut value: i64, conversion: &[(Range<i64>, i64)]) -> i64 {
    if let Some((_, delta)) = conversion.iter().find(|(range, _)| range.contains(&value)) {
        value += delta;
    }
    value
}

fn part_1(seeds: &[i64], conversions: &[Vec<(Range<i64>, i64)>]) {
    let seeds = seeds.to_owned();
    let converted = conversions.iter().fold(seeds, |mut seeds, conv| {
        // println!("Seeds: {seeds:?}");
        for s in &mut seeds {
            *s = convert(*s, conv);
        }
        seeds
    });
    println!("Seeds: {converted:?}");
    println!("Min loc: {}", converted.iter().min().unwrap());
}

fn convert_range(mut r: Range<i64>, conversion: &[(Range<i64>, i64)]) -> Vec<Range<i64>> {
    let mut sub_ranges = vec![];
    for (conv_r, conv_delta) in conversion {
        match (conv_r.contains(&r.start), conv_r.contains(&(r.end - 1))) {
            (true, true) => {
                sub_ranges.push(r.start + conv_delta..r.end + conv_delta);
                return sub_ranges;
            }
            (true, false) => {
                sub_ranges.push(r.start + conv_delta..conv_r.end + conv_delta);
                r.start = conv_r.end;
            }
            (false, true) => {
                sub_ranges.push(conv_r.start + conv_delta..r.end + conv_delta);
                r.end = conv_r.start;
            }
            (false, false) => {}
        }
    }
    sub_ranges.push(r);
    sub_ranges
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut content = String::new();
    File::open("in/5r.txt")?.read_to_string(&mut content)?;
    let mut blocks = content.split("\n\n");
    let seeds: Vec<i64> = blocks
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split_ascii_whitespace()
        .map(str::parse)
        .collect::<Result<_, _>>()
        .unwrap();

    let conversions = blocks
        .map(|b| b.lines().skip(1).map(parse_converter).collect_vec())
        .collect_vec();

    part_1(&seeds, &conversions);
    let seeds = seeds
        .iter()
        .step_by(2)
        .zip(seeds.iter().skip(1).step_by(2))
        .map(|(&base, &len)| base..base + len)
        .collect_vec();

    let converted = conversions.iter().fold(seeds, |seeds, conv| {
        println!("Seed ranges: {seeds:?}");
        seeds
            .into_iter()
            .flat_map(|r| convert_range(r, conv))
            .collect_vec()
    });
    let min = converted.into_iter().map(|r| r.start).min().unwrap();
    println!("Min: {min}");

    Ok(())
}
