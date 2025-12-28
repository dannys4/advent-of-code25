use std::cmp::{max, min};

use ndarray::prelude::*;

fn process_contents(contents: &String) -> Array2<u16> {
    let vecs: Vec<&str> = contents.split("\n").collect();
    let shape = (vecs.len(), vecs[0].len());
    let flat = vecs
        .into_iter()
        .map(|x| x.chars().map(|y| (y == '@') as u16))
        .flatten()
        .collect();
    let arr = Array2::from_shape_vec(shape, flat).unwrap();
    return arr;
}

fn find_viable(arr: &Array2<u16>) -> (u64, Array2<u16>) {
    const MAX_NEIGH: u16 = 4;
    let mut num = 0;
    let mut remaining: Array2<u16> = Array2::default(arr.raw_dim());
    for i in 0..(arr.shape()[0] as i64) {
        for j in 0..(arr.shape()[1] as i64) {
            let (iu, ju) = (i as usize, j as usize);
            if arr[[i as usize, j as usize]] == 0 {
                remaining[[iu, ju]] = 0;
                continue;
            }
            let i_min = max(i - 1, 0) as usize;
            let j_min = max(j - 1, 0) as usize;
            let i_max = min(i + 2, arr.shape()[0] as i64) as usize;
            let j_max = min(j + 2, arr.shape()[1] as i64) as usize;
            let slice = arr.slice(s![i_min..i_max, j_min..j_max]);
            let to_remove = (slice.sum() <= MAX_NEIGH) as u16;
            remaining[[iu, ju]] = 1 - to_remove;
            num += to_remove as u64;
        }
    }
    return (num, remaining);
}

fn part1(arr: &Array2<u16>) -> u64 {
    return find_viable(arr).0;
}

fn part2(arr: &Array2<u16>) -> u64 {
    let mut arr_loop = arr.clone();
    let mut total_remove = 0;
    let mut loop_num;
    loop {
        (loop_num, arr_loop) = find_viable(&arr_loop);
        if loop_num == 0 {
            break;
        }
        total_remove += loop_num;
    }
    return total_remove;
}

pub fn day4(contents: &String) {
    let arr = process_contents(contents);
    let p1 = part1(&arr);
    println!("Part 1: {p1}");
    let p2 = part2(&arr);
    println!("Part 2: {p2}")
}
