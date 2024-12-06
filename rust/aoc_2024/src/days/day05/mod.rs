use std::collections::{HashMap, HashSet};

use crate::util::parse_2_parts;

use super::{Answer, Day};
use itertools::Itertools;
use ratatui::prelude::*;
mod part_a;
mod part_b;

type Rulebook = HashMap<u32, HashSet<u32>>;

#[derive(Debug)]
pub struct Day5 {}

impl Day5 {
    pub fn new() -> Self {
        Self {}
    }
}

impl Day for Day5 {
    fn run(&self, _viz: Rect, _buf: &mut Buffer) -> Answer {
        Answer {
            parta: Some(part_a::run()),
            partb: Some(part_b::run()),
        }
    }
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
