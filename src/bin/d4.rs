use std::{collections::HashSet, error::Error, fs::File, io::Read};

fn get_winning_given(line: &str) -> (HashSet<u32>, Vec<u32>) {
    let interesting = line.split_once(':').unwrap().1.trim();
    let (winning, given) = interesting.split_once(" | ").unwrap();
    let winning = winning
        .split_ascii_whitespace()
        .map(str::parse)
        .collect::<Result<_, _>>()
        .unwrap();
    let given = given
        .split_ascii_whitespace()
        .map(str::parse)
        .collect::<Result<_, _>>()
        .unwrap();
    (winning, given)
}

fn count_winning(line: &str) -> usize {
    let (winning, given) = get_winning_given(line);
    given.iter().filter(|n| winning.contains(n)).count()
}

fn parse_line_1(line: &str) -> u32 {
    (1 << count_winning(line)) >> 1
}

fn parse_2(content: &str) -> u32 {
    let mut total_count = 0;
    let mut counts = vec![1; content.lines().count()];
    for (k, line) in content.lines().enumerate() {
        let card_count = counts[k];
        total_count += card_count;
        for v in counts.iter_mut().skip(k + 1).take(count_winning(line)) {
            *v += card_count;
        }
    }
    total_count
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut content = String::new();
    File::open("in/4r.txt")?.read_to_string(&mut content)?;
    println!(
        "Total (1): {}",
        content.lines().map(parse_line_1).sum::<u32>()
    );
    println!("Total (2): {}", parse_2(&content));
    Ok(())
}
