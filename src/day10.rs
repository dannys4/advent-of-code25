use good_lp::{default_solver, variable, Expression, ProblemVariables, Solution, SolverModel};
use indicatif::ProgressIterator;
use std::cmp::min;

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

fn part1_iter(machine: &Machine) -> u64 {
    // println!("{:?}", machine);
    let light_target = &machine.lights;
    let light_curr: Vec<bool> = (0..light_target.len()).map(|_| false).collect();
    let buttons = &machine.buttons;
    let buttons_pressed = part1_helper(light_target, &light_curr, buttons, 0);
    return buttons_pressed as u64;
}

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
