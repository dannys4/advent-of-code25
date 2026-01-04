use std::cmp::{max, min, Ordering};

use geo::{coord, Rect};
use itertools::Itertools;
use ndarray::prelude::*;

fn process_line(line: &str) -> (u64, u64) {
    let n = |x: Option<&str>| x.unwrap().parse::<u64>().unwrap();
    let mut sp = line.split(',');
    return (n(sp.next()), n(sp.next()));
}

fn process_contents(contents: &String) -> Vec<(u64, u64)> {
    return contents.split_whitespace().map(process_line).collect();
}

fn order<T: Ord>(a: T, b: T) -> (T, T) {
    if a.cmp(&b) == Ordering::Less {
        return (a, b);
    }
    return (b, a);
}

fn l2_dist(x: (u64, u64), y: (u64, u64)) -> u64 {
    let d = (x.0.abs_diff(y.0) as u64 + 1, x.1.abs_diff(y.1) as u64 + 1);
    return d.0 * d.1;
}

fn part1(coords: &Vec<(u64, u64)>) -> u64 {
    let mut ret = 0;
    for i in 0..coords.len() {
        for j in (i + 1)..coords.len() {
            let d = l2_dist(coords[i], coords[j]);
            if d > ret {
                ret = d;
            }
        }
    }
    return ret;
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Default)]
enum Color {
    #[default]
    Black,
    Red,
    Green,
}

fn bound_box(c1: (u64, u64), c2: (u64, u64)) -> (usize, usize, usize, usize) {
    let (x_min, x_max) = order(c1.0 as usize, c2.0 as usize);
    let (y_min, y_max) = order(c1.1 as usize, c2.1 as usize);
    return (x_min, x_max + 1, y_min, y_max + 1);
}

fn create_dense_coord_array(coords: &Vec<(u64, u64)>) -> (Vec<(u64, u64)>, Array2<Color>) {
    // Since minimum coordinate is unlikely 0, we only allocate what we expect we need
    // First find bounding box for coords
    let (row_min, row_max) = coords
        .iter()
        .map(|coord| coord.0 as usize)
        .minmax()
        .into_option()
        .unwrap();
    let (col_min, col_max) = coords
        .iter()
        .map(|coord| coord.1 as usize)
        .minmax()
        .into_option()
        .unwrap();
    // Shift all the coordinates to minimize wasted space
    let shifted_coords: Vec<_> = coords
        .iter()
        .map(|c| (c.0 - row_min as u64, c.1 - col_min as u64))
        .collect();
    // Create holding array
    let arr_shape = (row_max - row_min + 1, col_max - col_min + 1);
    let mut arr: Array2<Color> = Array2::default(arr_shape);
    // For each pair of (skipped) coordinates, i.e., (*idx, idx2, *idx3)
    // Set everything as green (inclusive)
    for idx in 0..shifted_coords.len() {
        let idx3 = (idx + 2) % shifted_coords.len();
        let (c1, c2) = (shifted_coords[idx], shifted_coords[idx3]);
        let (x_min, x_max, y_min, y_max) = bound_box(c1, c2);
        let mut slice = arr.slice_mut(s![x_min..x_max, y_min..y_max]);
        slice.fill(Color::Green);
    }
    // Now set every corner as red.
    for c in &shifted_coords {
        arr[[c.0 as usize, c.1 as usize]] = Color::Red;
    }
    return (shifted_coords, arr);
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Direction {
    N,
    S,
    E,
    W,
}

fn is_corner(arr: &Array2<Color>, idx: (usize, usize)) -> Option<(Direction, Direction)> {
    let (i, j) = idx;
    let has_n = i > 0 && arr[[i - 1, j]] != Color::Black;
    let has_w = j > 0 && arr[[i, j - 1]] != Color::Black;
    let has_s = i < arr.shape()[0] - 1 && arr[[i + 1, j]] != Color::Black;
    let has_e = j < arr.shape()[1] - 1 && arr[[i, j + 1]] != Color::Black;
    if !has_n && !has_e {
        return Some((Direction::N, Direction::E));
    }
    if !has_n && !has_w {
        return Some((Direction::N, Direction::W));
    }
    if !has_s && !has_e {
        return Some((Direction::S, Direction::E));
    }
    if !has_s && !has_w {
        return Some((Direction::S, Direction::W));
    }
    return None;
}

fn debug_arr_print(arr: &Array2<Color>) {
    let arr2 = arr
        .map(|&x| match x {
            Color::Red => '#',
            Color::Green => 'X',
            _ => '.',
        })
        .map_axis(Axis(0), |x| String::from_iter(x))
        .iter()
        .join("\n");
    println!("{}", arr2);
}

fn find_closest_reds(
    arr: &Array2<Color>,
    idx: (usize, usize),
    which_corner: (Direction, Direction),
) -> (usize, usize, usize, usize) {
    let (idx_y, idx_x) = idx;
    let (d_y, d_x) = which_corner;
    let (ret_x_min, ret_x_max, ret_y_min, ret_y_max);
    // find x first
    // println!("{:?}, {:?}, {:?}", idx, arr.shape(), which_corner);
    // debug_arr_print(arr);
    if d_x == Direction::E {
        let last_red = arr
            .slice(s![idx_y, ..idx_x])
            .iter()
            .rposition(|&c| c == Color::Red);
        (ret_x_min, ret_x_max) = (last_red.unwrap() + 1, idx_x + 1);
    } else if d_x == Direction::W {
        let first_red = arr
            .slice(s![idx_y, idx_x..])
            .iter()
            .position(|&c| c == Color::Red);
        (ret_x_min, ret_x_max) = (idx_x, first_red.unwrap() + idx_x);
    } else {
        panic!("Unexpected x direction {:?}", d_x);
    }
    if d_y == Direction::N {
        let slice = arr.slice(s![idx_y.., idx_x]);
        let first_red = slice.iter().position(|&c| c == Color::Red);
        (ret_y_min, ret_y_max) = (idx_y, first_red.unwrap() + idx_y);
    } else if d_y == Direction::S {
        let last_red = arr
            .slice(s![idx_y.., idx_x])
            .iter()
            .rposition(|&c| c == Color::Red);
        (ret_y_min, ret_y_max) = (last_red.unwrap() + 1, idx_y + 1);
    } else {
        panic!("Unexpected y direction {:?}", d_y);
    }
    return (ret_y_min, ret_y_max, ret_x_min, ret_x_max);
}

fn sparsify_coord_array(arr: &Array2<Color>) -> Array2<Color> {
    let mut ret_arr = arr.clone();
    for (idx, c) in arr.indexed_iter() {
        let which_corner = is_corner(arr, idx);
        // Must be green and a corner
        if !(*c == Color::Green && which_corner.is_some()) {
            continue;
        }
        let (x_idx_1, x_idx_2, y_idx_1, y_idx_2) =
            find_closest_reds(&ret_arr, idx, which_corner.unwrap());
        ret_arr
            .slice_mut(s![x_idx_1..x_idx_2, y_idx_1..y_idx_2])
            .fill(Color::Black);
    }
    return ret_arr;
}

fn is_box_valid(sub_arr: &ArrayRef2<Color>) -> bool {
    return sub_arr
        .iter()
        .all(|&c| c == Color::Red || c == Color::Green);
}

fn find_max_valid_box(coords: &Vec<(u64, u64)>, arr: &Array2<Color>) -> u64 {
    let mut max = 0;
    for i in 0..coords.len() {
        let c_i = coords[i];
        let (row1, col1) = c_i;
        for j in (i + 1)..coords.len() {
            let c_j = coords[j];
            let (row2, col2) = c_j;
            let (r_min, r_max) = order(row1 as usize, row2 as usize);
            let (c_min, c_max) = order(col1 as usize, col2 as usize);
            let d_ij = l2_dist(c_i, c_j);
            if d_ij > max {
                let arr_slice = arr.slice(s![r_min..r_max, c_min..c_max]);
                if is_box_valid(&arr_slice) {
                    max = d_ij;
                }
            }
        }
    }
    return max;
}

fn part2_new(coords: &Vec<(u64, u64)>) -> u64 {
    let (shifted_coords, arr) = create_dense_coord_array(&coords);
    let arr = sparsify_coord_array(&arr);
    return find_max_valid_box(&shifted_coords, &arr);
}

fn is_valid_idx(coords: &Vec<(u64, u64)>, idx: (usize, usize)) -> bool {
    let (c_1, c_2) = (coords[idx.0], coords[idx.1]);
    let (x_min, y_min) = (min(c_1.0, c_2.0), min(c_1.1, c_2.1));
    let (x_max, y_max) = (max(c_1.0, c_2.0), max(c_1.1, c_2.1));
    return coords
        .iter()
        .all(|&c| c.0 < x_min && c.0 > x_max && c.1 < y_min && c.1 > y_max);
}

fn part2(coords: &Vec<(u64, u64)>) -> u64 {
    let mut ret = 0;
    let mut idx = (0, 0);
    for i in 0..coords.len() {
        let j = (i + 2) % coords.len();
        let d = l2_dist(coords[i], coords[j]);
        if d >= ret && is_valid_idx(coords, (i, j)) {
            ret = d;
            idx = (i, j);
        }
    }
    println!("{:?}, {:?}", coords[idx.0], coords[idx.1]);
    return ret;
}

use geo::Contains;
use geo::GeoNum;
use geo::{line_string, point, LineString, Polygon};

fn part2_geom(coords: &Vec<(u64, u64)>) -> u64 {
    let points: Vec<_> = coords
        .iter()
        .map(|c| {
            point!(
                x: c.0 as f64,
                y: c.1 as f64
            )
        })
        .collect();
    let linestr = LineString::from(points);
    let poly = Polygon::new(linestr.clone(), vec![]);
    let mut max = 0;
    for i in 0..coords.len() {
        let c_i = coords[i];
        for j in (i + 1)..coords.len() {
            let c_j = coords[j];
            let d_ij = l2_dist(c_i, c_j);
            if d_ij > max {
                let box_ij = Rect::new(
                    coord! {x: c_i.0 as f64, y: c_i.1 as f64},
                    coord! {x: c_j.0 as f64, y: c_j.1 as f64},
                );
                if poly.contains(&box_ij) {
                    max = d_ij;
                }
            }
        }
    }
    return max;
}

pub fn day9(contents: &String) {
    let coords = process_contents(contents);
    // let p1 = part1(&coords);
    // println!("Part 1: {p1}");
    // let p2 = part2(&coords);
    // println!("Part 2: {p2}");
    // let p2 = part2_new(&coords);
    let p2 = part2_geom(&coords);
    println!("Part 2: {p2}");
}
