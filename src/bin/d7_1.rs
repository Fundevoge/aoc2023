use std::{
    error::Error,
    fmt::Display,
    fs::File,
    io::Read,
    ops::{Deref, DerefMut},
};

use itertools::Itertools;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
enum Value {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[repr(u8)]
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum HandOutcome {
    HighValue,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl PartialOrd for HandOutcome {
    fn partial_cmp(&self, other: &Self) -> std::option::Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HandOutcome {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (unsafe { *<*const _>::from(self).cast::<u8>() })
            .cmp(&unsafe { *<*const _>::from(other).cast::<u8>() })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Hand {
    values: Values,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let so = self.best_outcome();
        let oo = other.best_outcome();
        println!(
            "Comparing {self} to {other}: {so:?} -> {oo:?} : {:?}",
            so.cmp(&oo)
        );
        so.cmp(&oo).then_with(|| self.values.cmp(&other.values))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Values(Box<[Value]>);

impl Deref for Values {
    type Target = Box<[Value]>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Values {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<&[Value]> for Values {
    fn from(value: &[Value]) -> Self {
        Values(value.into())
    }
}

impl From<Values> for Vec<Value> {
    fn from(value: Values) -> Self {
        Vec::from(value.0)
    }
}

impl From<char> for Value {
    fn from(value: char) -> Self {
        match value {
            '2' => Value::Two,
            '3' => Value::Three,
            '4' => Value::Four,
            '5' => Value::Five,
            '6' => Value::Six,
            '7' => Value::Seven,
            '8' => Value::Eight,
            '9' => Value::Nine,
            'T' => Value::Ten,
            'J' => Value::Jack,
            'Q' => Value::Queen,
            'K' => Value::King,
            'A' => Value::Ace,
            _ => panic!("Invalid Value character {value}"),
        }
    }
}

impl<T: Iterator<Item = Value>> From<T> for Hand {
    fn from(value: T) -> Self {
        Self {
            values: Values(value.collect()),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}]",
            &self
                .values
                .iter()
                .map(|c| c.to_string())
                .fold(String::new(), |acc, v| acc + ", " + &v)[2..]
        )
    }
}

impl Hand {
    fn best_outcome(&self) -> HandOutcome {
        self.values.best_outcome()
    }
}

impl Values {
    fn best_outcome(&self) -> HandOutcome {
        let values_with_counts = &self.values();
        if values_with_counts.iter().any(|(_, c)| *c == 5) {
            HandOutcome::FiveOfAKind
        } else if values_with_counts.iter().any(|(_, c)| *c == 4) {
            HandOutcome::FourOfAKind
        } else if values_with_counts.iter().any(|(_, c)| *c == 3)
            && values_with_counts.iter().any(|(_, c)| *c == 2)
        {
            HandOutcome::FullHouse
        } else if values_with_counts.iter().any(|(_, c)| *c == 3) {
            HandOutcome::ThreeOfAKind
        } else if let Some((high_pair, _)) = values_with_counts.iter().find(|(_, c)| *c == 2) {
            if values_with_counts
                .iter()
                .filter(|(v, _)| v != high_pair)
                .any(|(_, c)| *c == 2)
            {
                HandOutcome::TwoPairs
            } else {
                HandOutcome::OnePair
            }
        } else {
            HandOutcome::HighValue
        }
    }

    fn values(&self) -> Vec<(Value, usize)> {
        let mut values = vec![];
        'outer: for c in self.iter() {
            for (v, count) in &mut values {
                if v == c {
                    *count += 1;
                    continue 'outer;
                }
            }
            values.push((*c, 1));
        }
        values
    }
}

fn parse_line(line: &str) -> (Hand, u64) {
    let (hand, value) = line.split_once(' ').unwrap();
    (hand.chars().map(Value::from).into(), value.parse().unwrap())
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut file = String::new();
    File::open("in/7r.txt")?.read_to_string(&mut file)?;
    let total = file
        .lines()
        .map(parse_line)
        .sorted_by(|(h1, _), (h2, _)| h1.cmp(h2))
        .map(|s| {
            println!("Sorted: {s:?}");
            s
        })
        .zip(1..)
        .fold(0, |acc, ((_, bet), index)| acc + bet * index);

    println!("Total: {total}");
    // 241305902
    // 241344943
    // 241480245
    Ok(())
}
