use super::{time_run, Answer, TX};
use crate::BoxedAsync;
use crate::{util::*, ItemTX};
use color_eyre::Result;

async fn run(mut tx: ItemTX) -> Result<()> {
    let parta = time_run(|| parta(&mut tx));
    let partb = time_run(|| partb(&mut tx));
    tx.done(Answer { parta, partb })?;
    Ok(())
}

pub fn parta(_tx: &mut ItemTX) -> String {
    let mut next_file_id = 0;
    let compressed = input();
    let mut disk = vec![-1; 100000];
    let mut size: usize = 0;
    for (i, s) in compressed.iter().enumerate() {
        let new_size = size + (*s as usize);
        if i % 2 == 0 {
            disk[size..new_size].fill(next_file_id);
            next_file_id += 1;
        }
        size = new_size;
    }

    let final_size = size;

    // 2ptrs baby
    let (mut i, mut j) = (0, final_size - 1);
    while i != j {
        if disk[i] != -1 {
            i += 1;
            continue;
        }
        if disk[j] == -1 {
            j -= 1;
            continue;
        }
        disk.swap(i, j);
        i += 1;
        j -= 1;
    }
    //tx.update(vec![format!("{:?}", &disk[0..final_size])])
    //     .unwrap();

    format!(
        "{:?}",
        disk.iter()
            .filter(|&&x| x >= 0)
            .enumerate()
            .map(|(i, &x)| i * (x as usize))
            .sum::<usize>()
    )
}

pub fn partb(tx: &mut ItemTX) -> String {
    let mut next_file_id = 0;
    let compressed = input();
    let mut disk = Vec::<(i32, usize)>::new();
    for (i, s) in compressed.iter().enumerate() {
        if i % 2 == 0 {
            disk.push((next_file_id, *s));
            next_file_id += 1;
        } else {
            disk.push((-1, *s));
        }
    }
    let (mut i, mut j) = (0, disk.len() - 1);
    while i != disk.len() {
        // println!("{:?}:{:?} {:?}:{:?}", i, j, disk[i], disk[j]);
        match (disk[i], disk[j]) {
            (_, _) if i >= j => {
                i += 1;
                j = disk.len() - 1;
            }
            (_, (-1, _)) => j -= 1,
            ((id, _), _) if id != -1 => i += 1,
            ((-1, empty_size), (_, size)) if empty_size < size => j -= 1,
            ((-1, empty_size), (id, size)) if empty_size == size => {
                println!("{id} -> {i}");
                disk[i] = (id, size);
                disk[j] = (-1, size);
                i += 1;
                j = disk.len() - 1;
            }
            ((-1, empty_size), (id, size)) if empty_size > size => {
                println!(
                    "{id} -> {i} (remander: {empty_size} - {size} = {})",
                    empty_size - size
                );
                disk[j] = (-1, size);
                disk.insert(i, (id, size));
                disk[i + 1] = (-1, empty_size - size);
                i += 1;
                j = disk.len() - 1;
            }
            (a, b) => panic!("i: {:?}, j: {:?}", a, b),
        }
    }

    tx.update(vec![disk
        .iter()
        .map(|(id, size)| {
            if *id == -1 {
                "., ".repeat(*size)
            } else {
                format!("{}, ", id).repeat(*size)
            }
        })
        .collect::<Vec<String>>()
        .join("")])
        .unwrap();

    format!(
        "{:?}",
        disk.iter()
            .enumerate()
            .filter(|(i, &(id, size))| id >= 0)
            .map(|(i, &(id, size))| if id > 0 { i * (id as usize) * size } else { 0 })
            .sum::<usize>()
    )
}

fn input() -> Vec<usize> {
    parse_lines_iter("day09_ex")
        .next()
        .unwrap()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect()
}

pub fn wrapped_run(tx: ItemTX) -> BoxedAsync {
    Box::pin(run(tx))
}

#[cfg(test)]
mod tests {
    use tokio::sync::mpsc::unbounded_channel;

    use crate::Ev;

    use super::*;

    #[test]
    fn it_works() {
        let (tx, _rx) = unbounded_channel::<Ev>();
        let mut itx = (0, tx);
        partb(&mut itx);
    }
}
