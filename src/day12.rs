use indicatif::ProgressIterator;
use itertools::Itertools;
use ndarray::prelude::*;
use regex::Regex;

const GIFT_SHAPE: (usize, usize) = (3, 3);

#[derive(Debug, PartialEq, Clone)]
struct Gift {
    north: Array2<bool>,
    east: Array2<bool>,
    south: Array2<bool>,
    west: Array2<bool>,
    num_occupied: u64,
}

fn count_array(input: &Array2<bool>) -> u64 {
    return input.map(|&i| i as u64).sum();
}

fn _print_bool_array(arr: ArrayView2<bool>) {
    println!(
        "{}",
        arr.map_axis(Axis(0), |a| {
            a.map(|&x| if x { '#' } else { '.' }).iter().join("")
        })
        .iter()
        .join("\n")
    );
}

fn rotate_once(gift: &Array2<bool>) -> Array2<bool> {
    let (rows, cols) = gift.dim();
    let mut arr = Array2::default((cols, rows));
    for shape_row in 0..gift.shape()[0] {
        for shape_col in 0..gift.shape()[1] {
            arr[[shape_col, rows - shape_row - 1]] = gift[[shape_row, shape_col]];
        }
    }
    return arr;
}

fn create_gift(gift_str: &str) -> Option<Gift> {
    let mut lines = gift_str.lines();
    lines.next();
    let it = lines.map(|l| l.chars().map(|c| c == '#')).flatten();
    let north = Array2::from_shape_vec(GIFT_SHAPE, it.collect()).ok()?;
    let east = rotate_once(&north);
    let south = rotate_once(&east);
    let west = rotate_once(&south);
    let num_occupied = count_array(&north);
    return Some(Gift {
        north,
        east,
        south,
        west,
        num_occupied,
    });
}

#[derive(Debug, PartialEq, Clone)]
struct Tree {
    rows: usize,
    cols: usize,
    requirements: Vec<u64>,
}

fn create_tree(tree_str: &str) -> Option<Tree> {
    let re = Regex::new(r"\d+").ok()?;
    let nums: Vec<_> = re.find_iter(tree_str).collect();
    let rows = nums[0].as_str().parse::<usize>().ok()?;
    let cols = nums[1].as_str().parse::<usize>().ok()?;
    let requirements: Vec<_> = nums[2..]
        .iter()
        .map(|m| m.as_str().parse::<u64>().unwrap())
        .collect();
    return Some(Tree {
        rows,
        cols,
        requirements,
    });
}

fn is_piece_fit(insert_area: ArrayView2<bool>, gift: ArrayView2<bool>) -> bool {
    assert!(insert_area.shape() == gift.shape());
    return !insert_area.iter().zip(gift).any(|(&a, &b)| a && b);
}

fn is_valid_num_occupied(gifts: &Vec<Gift>, tree: &Tree) -> bool {
    let tree_numel = (tree.rows * tree.cols) as u64;
    let total_num_occupied: u64 = gifts
        .iter()
        .zip(&tree.requirements)
        .map(|(g, &r)| g.num_occupied * r)
        .sum();
    return total_num_occupied <= tree_numel;
}

fn get_orientation(gift: &Gift, idx: u8) -> Option<&Array2<bool>> {
    return match idx {
        0 => Some(&gift.north),
        1 => Some(&gift.east),
        2 => Some(&gift.south),
        3 => Some(&gift.west),
        _ => None,
    };
}

fn fit_under_tree_loop(
    row_idx: usize,
    col_idx: usize,
    gifts: &Vec<Gift>,
    gift_arr: ArrayView2<bool>,
    state: ArrayView2<bool>,
    mut new_state: ArrayViewMut2<bool>,
    requirements: &mut Vec<u64>,
) -> Option<bool> {
    let row_bound = row_idx + gift_arr.shape()[0];
    let col_bound = col_idx + gift_arr.shape()[1];
    let slice_idxs = s![row_idx..row_bound, col_idx..col_bound];
    let slice: ArrayView2<bool> = state.slice(slice_idxs);
    let piece_fits = is_piece_fit(slice, gift_arr);
    // println!("Piece fits: {}", piece_fits);
    if piece_fits {
        let mut slice_mut: ArrayViewMut2<bool> = new_state.slice_mut(slice_idxs);
        for ((i, j), b) in gift_arr.indexed_iter() {
            slice_mut[[i, j]] |= b;
        }
        let state_view = new_state.view();
        let all_fit = fit_under_tree_helper(gifts, state_view, requirements)?;
        if all_fit {
            return Some(true);
        }
        let mut slice_mut: ArrayViewMut2<bool> = new_state.slice_mut(slice_idxs);
        slice_mut.assign(&slice);
    }
    return Some(false);
}

fn fit_under_tree_helper(
    gifts: &Vec<Gift>,
    state: ArrayView2<bool>,
    requirements: &mut Vec<u64>,
) -> Option<bool> {
    let first_idx = requirements.iter().find_position(|&&x| x != 0);
    if first_idx.is_none() {
        return Some(true);
    }
    let next_gift = first_idx.unwrap().0;
    requirements[next_gift] -= 1;
    let gift = &gifts[next_gift];
    // println!("\nstate and gift:"); //\n{:?}\n{:?}", state, gift.north);
    // print_bool_array(state);
    // print!("\n");
    // print_bool_array(gift.north.view());
    let mut new_state: Array2<bool> = state.to_owned();
    for orientation in 0..3 {
        let gift_arr = get_orientation(gift, orientation)?;
        let max_row = state.shape()[0] - gift_arr.shape()[0] + 1;
        let max_col = state.shape()[1] - gift_arr.shape()[1] + 1;
        // println!("{}: {:?}", orientation, (max_row, max_col));
        for row_idx in 0..max_row {
            for col_idx in 0..max_col {
                let fits = fit_under_tree_loop(
                    row_idx,
                    col_idx,
                    gifts,
                    gift_arr.view(),
                    state,
                    new_state.view_mut(),
                    requirements,
                )?;
                if fits {
                    return Some(true);
                }
            }
        }
    }
    requirements[next_gift] += 1;
    return Some(false);
}

fn fit_under_tree(tree: &Tree, gifts: &Vec<Gift>) -> Option<bool> {
    if !is_valid_num_occupied(gifts, tree) {
        // println!("Invalid gifts and tree:\n{:?}\n{:?}", gifts, tree);
        return Some(false);
    }
    let tree_size = (tree.rows, tree.cols);
    let tree_field: Array2<bool> = Array2::from_shape_fn(tree_size, |_| false);
    let mut requirements = tree.requirements.clone();
    return fit_under_tree_helper(gifts, tree_field.view(), &mut requirements);
}

fn process_contents(contents: &String) -> Option<(Vec<Gift>, Vec<Tree>)> {
    let sp: Vec<_> = contents.split("\n\n").collect();
    let gifts: Vec<_> = sp[..sp.len() - 1]
        .iter()
        .map(|&g| create_gift(g).unwrap())
        .collect();
    let trees = sp
        .last()?
        .split("\n")
        .map(|x| create_tree(x).unwrap())
        .collect();
    return Some((gifts, trees));
}

fn part1(contents: &String) -> Option<u64> {
    let (gifts, trees) = process_contents(contents)?;
    let ret = trees
        .iter()
        .progress()
        .map(|tree| fit_under_tree(tree, &gifts).unwrap() as u64)
        .sum();
    return Some(ret);
}

pub fn day12(contents: &String) {
    let p1 = part1(contents);
    println!("Part 1: {:?}", p1);
}
