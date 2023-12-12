use std::{
    error::Error,
    fs::File,
    io::Read,
    ops::AddAssign,
    sync::{Arc, Mutex},
};

use itertools::Itertools;
use lazy_static::lazy_static;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

lazy_static! {
    static ref COUNTER: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));
}

fn can_split_off_group_up_to_excl_i(group: &[char], len: usize, cutoff: usize) -> bool {
    group.get(cutoff).unwrap_or(&'.') != &'#'
        && cutoff
            .checked_sub(len + 1)
            .and_then(|idx| group.get(idx))
            .unwrap_or(&'.')
            != &'#'
}

fn count_exact_groupings(group: &[char], subgroup_lengths: &[usize]) -> u64 {
    if subgroup_lengths.is_empty() {
        if group.iter().all(|c| c == &'?') {
            return 1;
        } else {
            return 0;
        }
    }
    if subgroup_lengths.len() + subgroup_lengths.iter().sum::<usize>() > group.len() + 1 {
        return 0;
    }

    (subgroup_lengths[0]..=group.len())
        .take_while(|i| {
            i.checked_sub(subgroup_lengths[0] + 1)
                .and_then(|idx| group.get(idx))
                != Some(&'#')
        })
        .filter(|&i| can_split_off_group_up_to_excl_i(group, subgroup_lengths[0], i))
        .map(|i| {
            let new_i = if i < group.len() { i + 1 } else { i };
            count_exact_groupings(&group[new_i..], &subgroup_lengths[1..])
        })
        .sum::<u64>()
}

struct GroupingIter<'a> {
    group: &'a [char],
    subgroup_lengths: &'a [usize],
    first: bool,
    n_subgroups: usize,
    min_len: usize,
}

impl<'a> GroupingIter<'a> {
    fn new(group: &'a [char], subgroup_lengths: &'a [usize]) -> Self {
        GroupingIter {
            group,
            subgroup_lengths,
            first: true,
            n_subgroups: 1,
            min_len: subgroup_lengths[0],
        }
    }
}

impl<'a> Iterator for GroupingIter<'a> {
    type Item = (usize, u64);

    fn next(&mut self) -> Option<Self::Item> {
        if self.first {
            self.first = false;
            return Some((0, count_exact_groupings(self.group, &[])));
        }
        (self.min_len <= self.group.len() && self.n_subgroups <= self.subgroup_lengths.len()).then(
            || {
                let v = (
                    self.n_subgroups,
                    count_exact_groupings(self.group, &self.subgroup_lengths[..self.n_subgroups]),
                );
                if self.n_subgroups < self.subgroup_lengths.len() {
                    self.min_len += self.subgroup_lengths[self.n_subgroups] + 1;
                }
                self.n_subgroups += 1;
                v
            },
        )
    }
}

fn count_options(sym_groups: &[Vec<char>], ns: &[usize]) -> u64 {
    if ns.is_empty() {
        if sym_groups.iter().all(|g| g.iter().all(|c| c == &'?')) {
            return 1;
        } else {
            return 0;
        }
    }
    if sym_groups.is_empty() {
        return 0;
    }

    GroupingIter::new(&sym_groups[0], ns)
        .filter(|(_, opts)| *opts != 0)
        .map(|(n_ns, opts)| opts * count_options(&sym_groups[1..], &ns[n_ns..]))
        .sum()
}

fn n_options_1(line: &str) -> u64 {
    println!("{line}");
    let (syms, ns) = line.split_once(' ').unwrap();
    let sym_groups = syms
        .split('.')
        .filter(|&gr| !gr.is_empty())
        .map(|gr| gr.chars().collect_vec())
        .collect_vec();
    let ns: Vec<usize> = ns
        .split(',')
        .map(str::parse)
        .collect::<Result<_, _>>()
        .unwrap();
    let c = count_options(&sym_groups, &ns);
    println!("=> {c}\n");
    c
}

fn n_options_2(line: &&str) -> u64 {
    let mut c = COUNTER.lock().unwrap();
    println!("[{:03}] {line} ", *c);
    c.add_assign(1);
    drop(c);

    let (syms, ns) = line.split_once(' ').unwrap();

    let syms = format!("{syms}?{syms}?{syms}?{syms}?{syms}");
    let sym_groups = syms
        .split('.')
        .filter(|&gr| !gr.is_empty())
        .map(|gr| gr.chars().collect_vec())
        .collect_vec();

    let ns = format!("{ns},{ns},{ns},{ns},{ns}");
    let ns: Vec<usize> = ns
        .split(',')
        .map(str::parse)
        .map(Result::unwrap)
        .collect_vec();
    let c = count_options(&sym_groups, &ns);
    println!("{line} => {c}");
    c
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut file = String::new();
    File::open("in/12t2.txt")?.read_to_string(&mut file)?;
    let c1 = file.lines().map(n_options_1).sum::<u64>();
    println!("Total (1): {c1}");

    let lines = file.lines().collect_vec();
    let c2 = lines.par_iter().map(n_options_2).sum::<u64>();
    println!("Total (2): {c2}");

    Ok(())
}
