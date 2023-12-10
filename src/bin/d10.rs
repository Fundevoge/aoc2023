use std::{collections::HashSet, error::Error, fs::File, io::Read};

use itertools::Itertools;

const NE: char = 'L';
const NW: char = 'J';
const SE: char = 'F';
const SW: char = '7';
const NS: char = '|';
const EW: char = '-';

fn points_east(c: char) -> bool {
    [NE, SE, EW, 'S'].contains(&c)
}

fn points_west(c: char) -> bool {
    [NW, SW, EW, 'S'].contains(&c)
}

fn points_south(c: char) -> bool {
    [NS, SE, SW, 'S'].contains(&c)
}

fn points_north(c: char) -> bool {
    [NE, NS, NW, 'S'].contains(&c)
}

fn adjacents(p: (usize, usize), chars: &[Vec<char>]) -> Vec<(usize, usize)> {
    let mut next_positions = vec![];
    if points_east(chars[p.1][p.0])
        && p.0 + 1 < chars[p.1].len()
        && points_west(chars[p.1][p.0 + 1])
    {
        next_positions.push((p.0 + 1, p.1));
    }
    if points_west(chars[p.1][p.0]) && p.0 > 0 && points_east(chars[p.1][p.0 - 1]) {
        next_positions.push((p.0 - 1, p.1));
    }
    if points_north(chars[p.1][p.0]) && p.1 > 0 && points_south(chars[p.1 - 1][p.0]) {
        next_positions.push((p.0, p.1 - 1));
    }
    if points_south(chars[p.1][p.0]) && p.1 + 1 < chars.len() && points_north(chars[p.1 + 1][p.0]) {
        next_positions.push((p.0, p.1 + 1));
    }

    next_positions
}

fn is_vertical(p: (usize, usize), chars: &[Vec<char>], edges: &HashSet<(usize, usize)>) -> bool {
    chars[p.1][p.0] == NS
        || (chars[p.1][p.0] == 'S'
            && p.1 > 0
            && edges.contains(&(p.0, p.1 - 1))
            && points_south(chars[p.1 - 1][p.0])
            && edges.contains(&(p.0, p.1 + 1)))
            && points_north(chars[p.1 + 1][p.0])
}

fn is_north_east(p: (usize, usize), chars: &[Vec<char>], edges: &HashSet<(usize, usize)>) -> bool {
    chars[p.1][p.0] == NE
        || (chars[p.1][p.0] == 'S'
            && p.1 > 0
            && edges.contains(&(p.0, p.1 - 1))
            && points_south(chars[p.1 - 1][p.0])
            && edges.contains(&(p.0 + 1, p.1)))
            && points_west(chars[p.1][p.0 + 1])
}

fn is_south_east(p: (usize, usize), chars: &[Vec<char>], edges: &HashSet<(usize, usize)>) -> bool {
    chars[p.1][p.0] == SE
        || (chars[p.1][p.0] == 'S'
            && edges.contains(&(p.0, p.1 + 1))
            && points_north(chars[p.1 + 1][p.0])
            && edges.contains(&(p.0 + 1, p.1)))
            && points_west(chars[p.1][p.0 + 1])
}

fn is_north_west(p: (usize, usize), chars: &[Vec<char>], edges: &HashSet<(usize, usize)>) -> bool {
    chars[p.1][p.0] == NW
        || (chars[p.1][p.0] == 'S'
            && p.1 > 0
            && edges.contains(&(p.0, p.1 - 1))
            && points_south(chars[p.1 - 1][p.0])
            && edges.contains(&(p.0 - 1, p.1)))
            && points_east(chars[p.1][p.0 - 1])
}

fn is_south_west(p: (usize, usize), chars: &[Vec<char>], edges: &HashSet<(usize, usize)>) -> bool {
    chars[p.1][p.0] == SW
        || (chars[p.1][p.0] == 'S'
            && edges.contains(&(p.0, p.1 + 1))
            && points_north(chars[p.1 + 1][p.0])
            && edges.contains(&(p.0 - 1, p.1)))
            && points_east(chars[p.1][p.0 - 1])
}

fn is_east_west(p: (usize, usize), chars: &[Vec<char>], edges: &HashSet<(usize, usize)>) -> bool {
    chars[p.1][p.0] == EW
        || (chars[p.1][p.0] == 'S'
            && edges.contains(&(p.0 + 1, p.1))
            && points_west(chars[p.1][p.0 + 1])
            && edges.contains(&(p.0 - 1, p.1)))
            && points_east(chars[p.1][p.0 - 1])
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut file = String::new();
    File::open("in/10r.txt")?.read_to_string(&mut file)?;
    let chars = file.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let start = chars
        .iter()
        .enumerate()
        .filter_map(|(row, line)| {
            line.iter()
                .enumerate()
                .find(|(_, ch)| **ch == 'S')
                .map(|i_ch| (i_ch.0, row))
        })
        .next()
        .unwrap();
    let [mut c1, mut c2] = adjacents(start, &chars)[..] else {
        panic!("First tile should have 2 adjacents")
    };
    let (mut c1_last, mut c2_last) = (start, start);

    let mut edges = HashSet::from([start, c1, c2]);
    while c1 != c2 {
        (c1, c1_last) = (
            adjacents(c1, &chars)
                .into_iter()
                .find(|c| c != &c1_last)
                .unwrap(),
            c1,
        );
        (c2, c2_last) = (
            adjacents(c2, &chars)
                .into_iter()
                .find(|c| c != &c2_last)
                .unwrap(),
            c2,
        );
        edges.insert(c1);
        edges.insert(c2);
    }
    edges.insert(c1);
    println!("Total (1): {}", edges.len() / 2);

    let inside: i32 = chars
        .iter()
        .enumerate()
        .map(|(y, row)| {
            let mut x = 0;
            let mut inside = false;
            let mut n_inside = 0;
            while x < row.len() {
                if edges.contains(&(x, y)) {
                    if is_vertical((x, y), &chars, &edges) {
                        inside = !inside;
                    } else if is_north_east((x, y), &chars, &edges) {
                        x += 1;
                        while is_east_west((x, y), &chars, &edges) {
                            x += 1;
                        }
                        if is_south_west((x, y), &chars, &edges) {
                            inside = !inside;
                        }
                    } else if is_south_east((x, y), &chars, &edges) {
                        x += 1;
                        while is_east_west((x, y), &chars, &edges) {
                            x += 1;
                        }
                        if is_north_west((x, y), &chars, &edges) {
                            inside = !inside;
                        }
                    }
                } else if inside {
                    n_inside += 1;
                }
                x += 1;
            }
            n_inside
        })
        .sum();

    println!("Total (2): {inside}");
    Ok(())
}
