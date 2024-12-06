use super::{input, Rulebook};

pub fn run() -> String {
    let (rulebook, seqs) = input();
    seqs.iter()
        .filter(|s| is_valid(&rulebook, s))
        .map(|s| s[s.len() / 2])
        .sum::<u32>()
        .to_string()
}

fn is_valid(rulebook: &Rulebook, seq: &[u32]) -> bool {
    match seq {
        [a, rest @ ..] if let Some(before) = rulebook.get(a) => {
            !rest.iter().any(|n| before.contains(n)) && is_valid(rulebook, rest)
        }
        [_, rest @ ..] => is_valid(rulebook, rest),
        _ => true,
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         run();
//     }
// }
