use itertools::Itertools;
use ndarray::prelude::*;
use std::cmp::min;
use std::collections::HashSet;

fn process_line(line: &str) -> (u32, u32, u32) {
    let n = |x: Option<&str>| x.unwrap().parse::<u32>().unwrap();
    let mut sp = line.split(',');
    return (n(sp.next()), n(sp.next()), n(sp.next()));
}

fn process_contents(contents: &String) -> Vec<(u32, u32, u32)> {
    return contents.split_whitespace().map(process_line).collect();
}

fn l2_dist(x: (u32, u32, u32), y: (u32, u32, u32)) -> f64 {
    let d = (
        x.0.abs_diff(y.0) as u64,
        x.1.abs_diff(y.1) as u64,
        x.2.abs_diff(y.2) as u64,
    );
    let sq_dist = d.0 * d.0 + d.1 * d.1 + d.2 * d.2;
    return f64::sqrt(sq_dist as f64);
}

fn merge_loops(loops: &Vec<HashSet<usize>>) -> Vec<HashSet<usize>> {
    let mut unique_loops: Vec<HashSet<usize>>;
    let mut curr_loops = loops.clone();
    let mut prev_len = 0;
    loop {
        unique_loops = Vec::new();
        for s in curr_loops {
            let mut is_disjoint = true;
            for u in unique_loops.iter_mut() {
                if !s.is_disjoint(&u) {
                    u.extend(&s);
                    is_disjoint = false;
                }
            }
            if is_disjoint {
                unique_loops.push(s.clone());
            }
        }
        curr_loops = unique_loops.clone();
        if unique_loops.len() == prev_len {
            break;
        }
        prev_len = unique_loops.len();
    }
    return unique_loops;
}

fn sorted_dists(coords: &Vec<(u32, u32, u32)>) -> Vec<(usize, usize)> {
    let n = coords.len();
    let mut ret: Vec<_> = (0..n)
        .flat_map(|i| ((i + 1)..n).map(move |j| (i, j)))
        .collect();
    let cmp = |ij1: &(usize, usize), ij2: &(usize, usize)| {
        let a1 = l2_dist(coords[ij1.0], coords[ij1.1]);
        let a2 = l2_dist(coords[ij2.0], coords[ij2.1]);
        return a1.total_cmp(&a2);
    };
    ret.sort_by(cmp);
    return ret;
}

fn create_loops(queue: &[(usize, usize)]) -> Vec<HashSet<usize>> {
    let mut ret: Vec<HashSet<usize>> = Vec::default();
    for cart_idx in queue {
        let (row, col) = cart_idx;
        let mut is_used = false;
        for s in &mut ret {
            if s.contains(row) {
                s.insert(*col);
                is_used = true;
            }
            if s.contains(col) {
                s.insert(*row);
                is_used = true;
            }
        }
        if !is_used {
            let mut s = HashSet::new();
            s.insert(*row);
            s.insert(*col);
            ret.push(s);
        }
    }
    return ret;
}

fn part1_new(coords: &Vec<(u32, u32, u32)>, n_loop: usize, n_prod: usize) -> u64 {
    let dists = sorted_dists(coords);
    let n_max = min(dists.len(), n_loop);
    let loops = create_loops(&dists[..n_max]);
    let loops_merged = merge_loops(&loops);
    let mut lens: Vec<_> = loops_merged.iter().map(|s| s.len()).collect();
    lens.sort();
    lens.reverse();
    let n_ret = min(n_prod, lens.len());
    return lens[..n_ret].iter().fold(1, |acc, x| acc * (*x as u64));
}

fn part2(coords: &Vec<(u32, u32, u32)>) -> u64 {
    let mut dists = sorted_dists(coords);
    dists.reverse();
    let mut assignments: Array1<i64> = Array1::from_elem((coords.len(),), -1i64);
    let mut next_connect;
    let mut next_junction_loop = 0;
    loop {
        next_connect = dists.pop().unwrap();
        let r_1 = assignments[next_connect.0];
        let r_2 = assignments[next_connect.1];
        if r_1 < 0 && r_2 < 0 {
            assignments[next_connect.0] = next_junction_loop;
            assignments[next_connect.1] = next_junction_loop;
            next_junction_loop += 1;
        } else if r_1 < 0 {
            assignments[next_connect.0] = assignments[next_connect.1];
        } else if r_2 < 0 {
            assignments[next_connect.1] = assignments[next_connect.0];
        } else if r_1 != r_2 {
            let mod_assgn = assignments[next_connect.0];
            let to_assgn = assignments[next_connect.1];
            for idx in 0..assignments.len() {
                if assignments[idx] == mod_assgn {
                    assignments[idx] = to_assgn;
                }
            }
        }
        let (&a_min, &a_max) = assignments.iter().minmax().into_option().unwrap();
        if a_min == a_max {
            break;
        }
    }
    let (c1, c2) = (coords[next_connect.0], coords[next_connect.1]);
    return (c1.0 as u64) * (c2.0 as u64);
}

pub fn day8(contents: &String) {
    let coords = process_contents(contents);
    let p1 = part1_new(&coords, 1000, 3);
    println!("Part 1: {p1}");
    let p2 = part2(&coords);
    println!("Part 2: {p2}");
}
