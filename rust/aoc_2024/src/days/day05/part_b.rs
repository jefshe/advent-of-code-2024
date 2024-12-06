use super::{input, Rulebook};

pub fn run() -> String {
    let (rulebook, seqs) = input();
    seqs.into_iter()
        .filter(|s| !is_valid(&rulebook, s))
        .map(|mut s| {
            let mut i = 0;
            loop {
                if i >= s.len() - 1 {
                    return s;
                }
                if let Some(before) = rulebook.get(&s[i])
                    && let Some(j) = s[i + 1..].iter().position(|el| before.contains(el))
                {
                    s.swap(i, j + i + 1);
                    i = 0;
                    continue;
                }
                i += 1;
            }
        })
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
