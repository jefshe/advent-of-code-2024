use crate::util::{bigga, grid_get, parse_chars, Grid, D::*, XY};

pub fn run() -> String {
    let chars = parse_chars("day04");
    let search = bigga(&chars, 1, '.');
    let count = look_for_mas(&search);
    count.to_string()
}

fn look_for_mas(search: &Vec<Vec<char>>) -> usize {
    let mut total = 0;
    for y in 0..search.len() {
        for x in 0..search[y].len() {
            if search[y][x] == 'A' && is_x_mas(search, XY::new(x, y)) {
                total += 1;
            }
        }
    }
    total
}

const MS: &[char; 2] = &['M', 'S'];
fn is_x_mas(search: &Grid<char>, xy: XY) -> bool {
    let side_a = [
        grid_get(search, &xy, &UpLeft),
        grid_get(search, &xy, &DownRight),
    ];
    let side_b = [
        grid_get(search, &xy, &DownLeft),
        grid_get(search, &xy, &UpRight),
    ];
    side_a.iter().all(|c| MS.contains(c))
        && side_a[0] != side_a[1]
        && side_b.iter().all(|c| MS.contains(c))
        && side_b[0] != side_b[1]
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         run();
//     }
// }
