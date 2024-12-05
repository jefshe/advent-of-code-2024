use crate::util::{bigga, parse_chars, D, XY};
pub fn run() -> String {
    let chars = parse_chars("day04");
    let search = bigga(&chars, 1, '.');
    let count = look_for_xmas(&search);
    count.to_string()
}

static XMAS: &[char; 4] = &['X', 'M', 'A', 'S'];

fn look_for_xmas(search: &Vec<Vec<char>>) -> usize {
    let mut total = 0;
    for y in 0..search.len() {
        for x in 0..search[y].len() {
            if search[y][x] == 'X' {
                let xy = XY::new(x, y);
                total += xmas_cnt(&search, &xy, D::Up, 0)
                    + xmas_cnt(&search, &xy, D::Down, 0)
                    + xmas_cnt(&search, &xy, D::Left, 0)
                    + xmas_cnt(&search, &xy, D::Right, 0)
                    + xmas_cnt(&search, &xy, D::UpLeft, 0)
                    + xmas_cnt(&search, &xy, D::UpRight, 0)
                    + xmas_cnt(&search, &xy, D::DownLeft, 0)
                    + xmas_cnt(&search, &xy, D::DownRight, 0)
            }
        }
    }
    total
}

fn xmas_cnt(search: &Vec<Vec<char>>, xy: &XY, dir: D, xmas_pos: usize) -> usize {
    if xmas_pos == 4 {
        1
    } else if search[xy.y][xy.x] == XMAS[xmas_pos] {
        xmas_cnt(search, &xy.dir(&dir), dir, xmas_pos + 1)
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        run();
    }
}
