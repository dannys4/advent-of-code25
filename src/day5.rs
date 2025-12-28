// use std::cmp::{max, min};

use std::cmp::max;

fn process_range(range: &str) -> (u64, u64) {
    let mut sp = range.split("-");
    let start = sp.next().unwrap().parse::<u64>().unwrap();
    let end = sp.next().unwrap().parse::<u64>().unwrap();
    return (start, end);
}

fn process_contents(contents: &String) -> (Vec<(u64, u64)>, Vec<u64>) {
    let mut sp = contents.split("\n\n");
    let range_strs = sp.next().unwrap();
    let ranges = range_strs.split("\n").map(process_range).collect();
    let ids = sp
        .next()
        .unwrap()
        .split("\n")
        .map(|x| x.parse::<u64>().unwrap())
        .collect();
    return (ranges, ids);
}

fn is_fresh(ranges: &Vec<(u64, u64)>, id: u64) -> bool {
    for range in ranges {
        if id >= range.0 && id <= range.1 {
            return true;
        }
    }
    return false;
}

fn part1(ranges: &Vec<(u64, u64)>, ids: &Vec<u64>) -> u64 {
    let mut num_fresh = 0;
    for id in ids {
        if is_fresh(ranges, *id) {
            num_fresh += 1;
        }
    }
    return num_fresh;
}

fn part2(ranges: &Vec<(u64, u64)>) -> u64 {
    let mut sorted = ranges.clone();
    sorted.sort_by(|a, b| (a.0).cmp(&b.0));
    let mut start = sorted[0].1;
    let mut ret = sorted[0].1 - sorted[0].0 + 1;
    for j in 1..sorted.len() {
        let start_j = max(start + 1, sorted[j].0);
        let end_j = max(start, sorted[j].1);
        if start_j <= end_j {
            ret += end_j - start_j + 1;
        }
        start = end_j;
    }
    return ret;
}

pub fn day5(contents: &String) {
    let (ranges, ids) = process_contents(contents);
    let p1 = part1(&ranges, &ids);
    println!("Part 1: {p1}");
    let p2 = part2(&ranges);
    println!("Part 2: {p2}");
}
