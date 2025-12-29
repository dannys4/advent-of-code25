use ndarray::prelude::*;
use regex::Regex;
use std::{cmp::max, default, iter::zip};

fn process_contents(contents: &String) -> (Array2<u64>, Vec<char>) {
    let sp: Vec<&str> = contents.split("\n").collect();
    let ops: Vec<char> = sp[sp.len() - 1]
        .split_whitespace()
        .map(|x| if x == "*" { '*' } else { '+' })
        .collect();
    let flat_arr = sp[..(sp.len() - 1)]
        .into_iter()
        .map(|x| x.split_whitespace().map(|y| y.parse::<u64>().unwrap()))
        .flatten()
        .collect();
    let arr: Array2<u64> = Array2::from_shape_vec((sp.len() - 1, ops.len()), flat_arr).unwrap();
    return (arr, ops);
}

fn count_digits(n: usize) -> usize {
    return (n.checked_ilog10().unwrap_or(0) as usize) + 1;
}

fn process_contents_p2(contents: &String) -> (Array2<&str>, Vec<char>) {
    let lines: Vec<&str> = contents.split("\n").collect();
    let num_lines = lines.len();
    let line_len = lines[0].split_whitespace().count();
    let ops: Vec<char> = lines[num_lines - 1]
        .split_whitespace()
        .map(|x| {
            if x == "*" {
                '*'
            } else if x == "+" {
                '+'
            } else {
                panic!("Found operation {x}")
            }
        })
        .collect();
    let num_digits: Vec<usize> = lines
        .clone()
        .into_iter()
        .map(|line| line.split_whitespace().map(|n| n.len()))
        .flatten()
        .collect();
    let num_arr: Array2<usize> = Array2::from_shape_vec((num_lines, line_len), num_digits).unwrap();
    let max_digs: Vec<usize> = num_arr
        .columns()
        .into_iter()
        .map(|x| *x.into_iter().max().unwrap())
        .collect();
    let mut full_arr: Array2<&str> = Array2::default((num_lines - 1, line_len));
    for (line_idx, line) in lines.iter().enumerate() {
        if line_idx == lines.len() - 1 {
            break;
        }
        let mut start = 0;
        for num_idx in 0..line_len {
            let end = start + max_digs[num_idx];
            full_arr[[line_idx, num_idx]] = &line[start..end];
            start = end + 1;
        }
    }
    return (full_arr, ops);
}

fn part1(arr: &Array2<u64>, ops: &Vec<char>) -> u64 {
    let mut ret = 0;
    for (op, col) in zip(ops, arr.columns()) {
        if *op == '*' {
            ret += col.product();
        } else if *op == '+' {
            ret += col.sum();
        } else {
            panic!("Unexpected op {op}")
        }
    }
    return ret;
}

fn p2_iter(col: &ArrayView1<&str>, op: char) -> u64 {
    let num_count = col[0].len();
    let col_chars: Vec<Vec<char>> = col
        .to_vec()
        .into_iter()
        .map(|x| x.chars().collect())
        .collect();
    let mut ret;
    if op == '*' {
        ret = 1;
    } else if op == '+' {
        ret = 0;
    } else {
        panic!("Unexpected op {op}");
    }
    for num_idx in 0..num_count {
        let num = col_chars
            .clone()
            .into_iter()
            .map(|x| x[num_idx])
            .collect::<String>()
            .trim()
            .parse::<u64>()
            .unwrap();
        if op == '*' {
            ret *= num;
        } else {
            ret += num;
        }
    }
    return ret;
}

fn part2(contents: &String) -> u64 {
    let (p2_arr, ops) = process_contents_p2(contents);
    let mut ret = 0;
    for (col, op) in zip(p2_arr.columns(), ops) {
        ret += p2_iter(&col, op);
    }
    return ret;
}

pub fn day6(contents: &String) {
    let (arr, ops) = process_contents(contents);
    let p1 = part1(&arr, &ops);
    println!("Part 1: {p1}");
    let p2 = part2(contents);
    println!("Part 2: {p2}");
}
