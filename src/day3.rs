use std::cmp::max;

fn p1_process_bank(ints: &Vec<u64>) -> u64 {
    let (mut curr_idx, mut curr_max) = (0, 0);
    for (j, x) in ints[..ints.len() - 1].iter().enumerate() {
        if *x > curr_max {
            (curr_idx, curr_max) = (j, *x);
        }
    }
    let mut next_max = 0;
    for x in &ints[(curr_idx + 1)..] {
        if *x > next_max {
            next_max = *x;
        }
    }
    // println!("{curr_max}, {next_max}");
    return curr_max * 10 + next_max;
}
const LIST_LEN: usize = 100;

fn part1(banks: &Vec<Vec<u64>>) -> u64 {
    let mut ret = 0;
    for (_j, bank) in banks.iter().enumerate() {
        let b_ret = p1_process_bank(bank);
        ret += b_ret;
    }
    return ret;
}

fn p2_helper(list: &[u64; LIST_LEN], curr: u64, start_idx: usize, last_idx: usize) -> u64 {
    if start_idx >= LIST_LEN {
        return 10 * curr + list[start_idx];
    }
    let mut max_val = 0;
    for j in start_idx..last_idx {
        if list[j] > max_val {
            max_val = list[j];
        }
    }
    let mut curr_max = curr;
    for j in start_idx..last_idx {
        if list[j] == max_val {
            let curr_j = 10 * curr + list[j];
            let out_p2;
            if last_idx >= LIST_LEN {
                out_p2 = curr_j;
            } else {
                out_p2 = p2_helper(list, curr_j, j + 1, last_idx + 1);
            }
            curr_max = max(curr_max, out_p2);
        }
    }
    return curr_max;
}

fn to_array<T>(v: &Vec<T>) -> [T; LIST_LEN]
where
    T: Copy,
{
    let slice = v.as_slice();
    let array: [T; LIST_LEN] = match slice.try_into() {
        Ok(ba) => ba,
        Err(_) => panic!(
            "Expected a Vec of length {} but it was {}",
            LIST_LEN,
            v.len()
        ),
    };
    return array;
}

fn part2(banks: &Vec<Vec<u64>>) -> u64 {
    let mut ret = 0;
    const NUM_DIGITS: usize = 12;
    for (_j, bank) in banks.iter().enumerate() {
        let bank_arr = to_array(bank);
        let b_ret = p2_helper(&bank_arr, 0, 0, LIST_LEN - NUM_DIGITS + 1);
        ret += b_ret;
    }
    return ret;
}

pub fn day3(contents: &String) {
    let ints: Vec<Vec<u64>> = contents
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| {
            x.chars()
                .map(|x| x.to_digit(10).unwrap() as u64)
                .collect::<Vec<u64>>()
        })
        .collect();
    let p1 = part1(&ints);
    println!("Part 1: {p1}");
    let p2 = part2(&ints);
    println!("Part 2: {p2}");
}
