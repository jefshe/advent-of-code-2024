use super::*;
use crate::util::parse_2_parts;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
mod part_a;
mod part_b;
use color_eyre::Result;

type Rulebook = HashMap<u32, HashSet<u32>>;

async fn run(mut tx: ItemTX) -> Result<()> {
    let ans = Answer {
        parta: time_run(part_a::run),
        partb: time_run(part_b::run),
    };
    tx.done(ans)?;
    Ok(())
}

pub fn wrapped_run(tx: ItemTX) -> BoxedAsync {
    Box::pin(run(tx))
}

fn input() -> (Rulebook, Vec<Vec<u32>>) {
    let (rules_part, seqs_part) = parse_2_parts("day05");
    let rulebook: Rulebook = rules_part
        .into_iter()
        .map(|l| {
            l.split("|")
                .map(|n| n.parse().unwrap())
                .collect_tuple::<(u32, u32)>()
        })
        .map(|t| t.expect("Could not parse tuple"))
        .map(|(before, after)| (after, before))
        .into_group_map::<u32, u32>()
        .into_iter()
        .map(|(k, v)| (k, v.into_iter().collect()))
        .collect();

    let seqs: Vec<Vec<u32>> = seqs_part
        .into_iter()
        .map(|l| l.split(",").map(|n| n.parse().unwrap()).collect())
        .collect();
    (rulebook, seqs)
}
