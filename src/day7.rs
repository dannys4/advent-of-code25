use std::collections::HashMap;
use std::hash::Hash;
use std::mem::swap;

use ndarray::prelude::*;

fn process_contents(contents: &String) -> (usize, Array2<bool>) {
    let mut lines = contents.lines();
    let first_line = lines.next().unwrap();
    let start_pos = first_line.find('S').unwrap();
    let num_lines = lines.clone().count();
    let flat_iter = lines
        .map(|line| line.chars().map(|x| x == '^'))
        .flatten()
        .collect();
    let arr_shape = (num_lines, first_line.len());
    let arr: Array2<bool> = Array2::from_shape_vec(arr_shape, flat_iter).unwrap();
    return (start_pos, arr);
}

fn part1(start_pos: usize, arr: &Array2<bool>) -> u64 {
    let row_len = arr.shape()[1];
    let mut curr_beam_row: Array1<bool> = Array1::default((row_len,));
    let mut next_beam_row: Array1<bool> = Array1::default((row_len,));
    for idx in 0..row_len {
        curr_beam_row[idx] = idx == start_pos;
    }
    let mut ret = 0;
    for row in arr.rows() {
        let overlap = &row & &curr_beam_row;
        ret += overlap.iter().fold(0u64, |acc, x| acc + (*x as u64));
        for idx in 0..row_len {
            next_beam_row[idx] = curr_beam_row[idx];
            if idx > 0 {
                next_beam_row[idx] |= curr_beam_row[idx - 1] && row[idx - 1];
            }
            if idx < row_len - 1 {
                next_beam_row[idx] |= curr_beam_row[idx + 1] && row[idx + 1];
            }
            if row[idx] {
                next_beam_row[idx] = false;
            }
        }
        swap(&mut next_beam_row, &mut curr_beam_row);
    }
    return ret;
}

fn part2_helper(
    curr_pos: usize,
    arr: &Array2<bool>,
    row_idx: usize,
    mut memos: &mut HashMap<(usize, usize), u64>,
) -> u64 {
    if row_idx >= arr.shape()[0] {
        return 1;
    }
    let key = (curr_pos, row_idx);
    if memos.contains_key(&key) {
        return *memos.get(&key).unwrap();
    }
    if arr[[row_idx, curr_pos]] {
        let mut ret = 0;
        if curr_pos > 0 {
            ret += part2_helper(curr_pos - 1, arr, row_idx + 1, memos);
        }
        if curr_pos < arr.shape()[1] - 1 {
            ret += part2_helper(curr_pos + 1, arr, row_idx + 1, memos);
        }
        memos.insert(key, ret);
        return ret;
    }
    let ret = part2_helper(curr_pos, arr, row_idx + 1, memos);
    memos.insert(key, ret);
    return ret;
}

fn part2(start_pos: usize, arr: &Array2<bool>) -> u64 {
    let mut memos: HashMap<(usize, usize), u64> = HashMap::new();
    return part2_helper(start_pos, arr, 0, &mut memos);
}

pub fn day7(contents: &String) {
    let (start_pos, arr) = process_contents(contents);
    let p1 = part1(start_pos, &arr);
    println!("Part 1: {p1}");
    let p2 = part2(start_pos, &arr);
    println!("Part 2: {p2}");
}
