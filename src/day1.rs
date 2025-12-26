use regex::Regex;
use std::collections::HashMap;

fn as_num(n: &str, num_map: &HashMap<&str,u32>) -> u32 {
    // println!("n = {n}\n");
    let n_dig = n.chars().next().unwrap().to_digit(10);
    if n_dig.is_some() {
        return n_dig.unwrap();
    }
    return *num_map.get(n).expect("Key not found");
}

pub fn day1(contents: &String) {
    let nums_str = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut num_dict: HashMap<&str, u32> = HashMap::new();
    let mut nums = Vec::new();
    for (j, num_str) in nums_str.iter().enumerate() {
        num_dict.insert(*num_str, (j+1).try_into().unwrap());
        nums.push(*num_str);
    }
    nums.push(r"[1-9]");
    let mut sum = 0;
    for line in contents.lines() {
        if line.len() <= 0 {
            break;
        }
        let nums: Vec<u32> = line.chars().filter_map(|a| a.to_digit(10)).collect();
        sum += nums[0]*10 + nums[nums.len() - 1];
    }
    println!("Part 1: Sum is {sum}");
    sum = 0;
    let matchers = nums.iter().map(|x| Regex::new(x).unwrap()).collect::<Vec<_>>();
    for line in contents.lines() {
        if line.len() <= 0 {
            break;
        }
        let mut min_idx: i32 = -1;
        let mut min_val = 0;
        let mut max_idx = 0;
        let mut max_val = 0;
        for matcher in matchers.clone() {
            // let num_iter = matcher.find_iter(line).map(|n| as_num(n.as_str(), &num_dict));
            for my_match in matcher.find_iter(line) {
                let start_idx: i32 = my_match.start().try_into().unwrap();
                if min_idx < 0 || start_idx <= min_idx {
                    min_idx = start_idx;
                    min_val = as_num(my_match.as_str(), &num_dict);
                }
                if start_idx >= max_idx {
                    max_idx = start_idx;
                    max_val = as_num(my_match.as_str(), &num_dict);
                }
            }
        }
        sum += min_val*10 + max_val;
    }
    println!("Part 2: Sum is {sum}");
}