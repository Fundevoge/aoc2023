use std::{error::Error, fs::File, io::Read};

use itertools::Itertools;

fn taxicab_11(((x1, y1), (x2, y2)): (&(usize, usize), &(usize, usize))) -> usize {
    x1.abs_diff(*x2) + y1.abs_diff(*y2)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut file = String::new();
    File::open("in/11r.txt")?.read_to_string(&mut file)?;
    let positions = file
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .filter_map(move |(x, c)| (c == '#').then_some((x, y)))
        })
        .collect_vec();
    let empty_rows = (0..file.lines().count())
        .filter(|y| positions.iter().all(|(_, y2)| y != y2))
        .collect_vec();
    let empty_cols = (0..file.lines().next().unwrap().len())
        .filter(|x| positions.iter().all(|(x2, _)| x != x2))
        .collect_vec();

    let expanded_positions_1 = positions
        .iter()
        .map(|(x, y)| {
            (
                x + empty_cols.iter().take_while(|x2| x2 < &x).count(),
                y + empty_rows.iter().take_while(|y2| y2 < &y).count(),
            )
        })
        .collect_vec();
    let sum = expanded_positions_1
        .iter()
        .cartesian_product(expanded_positions_1.iter())
        .map(taxicab_11)
        .sum::<usize>()
        / 2;

    println!("Total (1): {sum}");

    let expanded_positions_2 = positions
        .iter()
        .map(|(x, y)| {
            (
                x + empty_cols.iter().take_while(|x2| x2 < &x).count() * 999999,
                y + empty_rows.iter().take_while(|y2| y2 < &y).count() * 999999,
            )
        })
        .collect_vec();
    let sum = expanded_positions_2
        .iter()
        .cartesian_product(expanded_positions_2.iter())
        .map(taxicab_11)
        .sum::<usize>()
        / 2;

    println!("Total (2): {sum}");

    Ok(())
}
