use std::{error::Error, fs::File, io::Read};

use bacon_sci::interp::lagrange;
use itertools::Itertools;

fn main() -> Result<(), Box<dyn Error>> {
    let mut file = String::new();
    File::open("in/9r.txt")?.read_to_string(&mut file)?;

    let (sum_1, sum_2): (i64, i64) = file
        .lines()
        .map(|l| {
            let ns = l
                .split_ascii_whitespace()
                .map(str::parse)
                .collect::<Result<Vec<f64>, _>>()
                .unwrap();
            let poly = lagrange(
                &(0..ns.len() as u32).map(f64::from).collect_vec(),
                &ns,
                1e-30,
            )
            .unwrap();
            let next = poly.evaluate(ns.len() as f64).round() as i64;
            let prev = poly.evaluate(-1.0).round() as i64;
            (prev, next)
        })
        .fold((0, 0), |(s1, s2), (p, n)| (s1 + p, s2 + n));

    println!("Total (1): {sum_1}, Total (2): {sum_2}");

    Ok(())
}
