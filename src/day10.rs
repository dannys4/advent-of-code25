use good_lp::{
    constraint, default_solver, variable, variables, Expression, ProblemVariables, Solution,
    SolverModel,
};
use indicatif::ProgressIterator;
use itertools::Itertools;
use ndarray::prelude::*;
use nnls;
use std::{cmp::min, collections::HashMap};

#[derive(Debug)]
struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<u16>,
}

fn make_machine(line: &str) -> Machine {
    let mut sp = line.split_whitespace();
    let light_str = sp.next().unwrap();
    let lights: Vec<_> = light_str[1..(light_str.len() - 1)]
        .chars()
        .map(|x| x == '#')
        .collect();
    let mut buttons = Vec::<Vec<usize>>::new();
    let joltage: Vec<u16>;
    loop {
        let next_button = sp.next().unwrap();
        if next_button.starts_with('{') {
            joltage = next_button[1..(next_button.len() - 1)]
                .split(',')
                .map(|x| x.parse::<u16>().unwrap())
                .collect();
            break;
        }
        buttons.push(
            next_button[1..(next_button.len() - 1)]
                .split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect(),
        )
    }
    return Machine {
        lights,
        buttons,
        joltage,
    };
}

fn part1_helper(
    light_target: &Vec<bool>,
    light_curr: &Vec<bool>,
    buttons: &[Vec<usize>],
    buttons_pressed: u16,
) -> u16 {
    if light_target == light_curr {
        return buttons_pressed;
    }
    if buttons.len() == 0 {
        return u16::MAX;
    }
    let button = &buttons[0];
    let buttons_left = &buttons[1..];
    let not_press_button = part1_helper(light_target, light_curr, buttons_left, buttons_pressed);
    let mut light_pressed = light_curr.clone();
    for &b_idx in button {
        light_pressed[b_idx] = !light_pressed[b_idx];
    }
    let press_button = part1_helper(
        light_target,
        &light_pressed,
        buttons_left,
        buttons_pressed + 1,
    );
    return min(not_press_button, press_button);
}

fn part2_helper(
    joltage_target: &Vec<u16>,
    joltage_curr: &Vec<u16>,
    buttons: &[Vec<usize>],
    memo: &mut HashMap<(Vec<u16>, usize), u16>,
) -> u16 {
    if joltage_target == joltage_curr {
        return 0;
    }
    if buttons.len() == 0 {
        return u16::MAX;
    }
    let memo_key = (joltage_curr.clone(), buttons.len());
    let memoized = memo.get(&memo_key);
    if memoized.is_some() {
        let memo_val = *memoized.unwrap();
        if memo_val == u16::MAX {
            return memo_val;
        }
        return memo_val;
    }
    let button = &buttons[0];
    let buttons_left = &buttons[1..];
    // let not_press_button =
    let mut curr_min = u16::MAX;
    let mut buttons_pressed_loop = 0;
    let mut joltage_pressed = joltage_curr.clone();
    loop {
        let press_button = part2_helper(joltage_target, &joltage_pressed, buttons_left, memo);
        if press_button != u16::MAX {
            curr_min = min(curr_min, press_button + buttons_pressed_loop);
        }
        buttons_pressed_loop += 1;

        for &b_idx in button {
            if joltage_pressed[b_idx] >= joltage_target[b_idx] {
                memo.insert(memo_key, curr_min);
                return curr_min;
            }
            joltage_pressed[b_idx] += 1;
        }
    }
}

fn part1_iter(machine: &Machine) -> u64 {
    // println!("{:?}", machine);
    let light_target = &machine.lights;
    let light_curr: Vec<bool> = (0..light_target.len()).map(|_| false).collect();
    let buttons = &machine.buttons;
    let buttons_pressed = part1_helper(light_target, &light_curr, buttons, 0);
    return buttons_pressed as u64;
}

fn part2_iter(machine: &Machine) -> u64 {
    let joltage_target = &machine.joltage;
    let joltage_curr: Vec<u16> = (0..joltage_target.len()).map(|_| 0).collect();
    let buttons = &machine.buttons;
    let mut memo = HashMap::new();
    let buttons_pressed = part2_helper(joltage_target, &joltage_curr, buttons, &mut memo);
    return buttons_pressed as u64;
}

fn part2_iter_linalg(machine: &Machine) -> u64 {
    let (m, n) = (machine.joltage.len(), machine.buttons.len());
    let mut lhs: Array2<f64> = Array2::zeros((m, n));
    for (col_idx, b) in (&machine.buttons).iter().enumerate() {
        for &b_idx in b {
            lhs[[b_idx, col_idx]] = 1.;
        }
    }
    let rhs: Array1<f64> = Array1::from_iter(machine.joltage.iter().map(|&x| x as f64));
    let (ans, err): (Array1<f64>, f64) = nnls::nnls(lhs.view(), rhs.view());
    println!("{:?}", ans);
    return 0;
}

// fn produce_constraint(target_joltage: u16, button_presses: &Vec<Variable>, buttons: &Vec<Vec<usize>>) -> Constraint

fn part2_iter_lp(machine: &Machine) -> u64 {
    let (joltage, buttons) = (&machine.joltage, &machine.buttons);
    let (m, n) = (machine.joltage.len(), buttons.len());
    // variables! {problem: x[n] (integer) >= 0;}
    let mut problem = ProblemVariables::new();
    let vars = vec![variable().min(0).integer(); n];
    let button_presses: Vec<_> = problem.add_all(vars);
    let objective: Expression = button_presses.iter().sum();
    let mut model = problem.minimise(&objective).using(default_solver);
    for joltage_idx in 0..m {
        let expr_joltage: Expression = buttons
            .iter()
            .enumerate()
            .filter_map(|(b_idx, b)| {
                if b.contains(&joltage_idx) {
                    Some(button_presses[b_idx])
                } else {
                    None
                }
            })
            .sum();
        let constr = expr_joltage.eq(joltage[joltage_idx]);
        model = model.with(constr);
        // sum_{j} button_presses[j] * button_action[i,j] = joltage[i]
    }
    let opt_obj = model.solve().unwrap().eval(&objective);
    return opt_obj as u64;
}

fn part1(machines: &Vec<Machine>) -> u64 {
    return machines.iter().map(part1_iter).sum();
}

fn part2(machines: &Vec<Machine>) -> u64 {
    return machines.iter().progress().map(part2_iter_lp).sum();
}

pub fn day10(contents: &String) {
    let machines: Vec<_> = contents.lines().map(make_machine).collect();
    let p1 = part1(&machines);
    println!("Part 1: {}", p1);
    let p2 = part2(&machines);
    println!("Part 2: {}", p2);
}
