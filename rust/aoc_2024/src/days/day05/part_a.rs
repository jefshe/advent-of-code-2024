use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::util::parse_2_parts;

type Rulebook = HashMap<u32, HashSet<u32>>;

pub fn run() -> String {
    let (rules_part, seqs_part) = parse_2_parts("day05_ex");
    let rules: Vec<(u32, u32)> = rules_part
        .into_iter()
        .map(|l| {
            l.split("|")
                .map(|n| n.parse().unwrap())
                .collect_tuple::<(u32, u32)>()
        })
        .map(|t| t.expect("Could not parse tuple"))
        .collect();
    let seqs: Vec<Vec<u32>> = seqs_part
        .into_iter()
        .map(|l| l.split(",").map(|n| n.parse().unwrap()).collect())
        .collect();
    println!("{:?}", seqs);
    let rulebook: Rulebook = rules
        .into_iter()
        .map(|(before, after)| (after, before))
        .into_group_map::<u32, u32>()
        .into_iter()
        .map(|(k, v)| (k, v.into_iter().collect()))
        .collect();
    seqs.iter()
        .filter(|s| is_valid(&rulebook, s))
        .map(|s| s[s.len() / 2])
        .sum::<u32>()
        .to_string()
}

fn is_valid(rulebook: &Rulebook, seq: &[u32]) -> bool {
    let mut seen = HashSet::new();
    for s in seq {
        if let Some(before) = rulebook.get(s)
            && seen.is_disjoint(before)
        {
            println!("{:?}: {s} is disjoint", seq);
            return false;
        }
        seen.insert(*s);
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        run();
    }
}
