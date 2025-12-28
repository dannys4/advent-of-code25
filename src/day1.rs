fn dir_sign(dir: char) -> i32 {
    let ldir = dir.to_ascii_uppercase();
    match ldir {
        'L' => return -1,
        'R' => return 1,
        _ => return 0,
    }
}

fn divmod(i: i32, d: i32) -> (i32, i32) {
    return (i / d, i % d);
}

fn get_number(line: &str) -> i32 {
    let sign = dir_sign(line.chars().next().unwrap());
    let num_str = &line[1..];
    let num = num_str.parse::<i32>().unwrap();
    return sign * num;
}

fn part1(histories: &Vec<i32>) -> i32 {
    let mut tracker = 50;
    let mut zeros = 0;
    for x in histories {
        tracker = (tracker + x) % 100;
        if tracker == 0 {
            zeros += 1;
        }
    }
    return zeros;
}

fn part2_iter(start: i32, inc: i32) -> (i32, i32) {
    // Returns (next, number zeros)
    let next_full = start + inc;
    let (next, num_zeros);
    let (q, r) = divmod(next_full, 100);
    if next_full < 0 {
        next = (100 + r) % 100;
    } else {
        next = r;
    }
    if next_full == 0 {
        num_zeros = (inc != 0) as i32;
    } else if next_full < 0 {
        num_zeros = q.abs() + ((start != 0) as i32);
    } else if next_full >= 100 {
        num_zeros = q;
    } else {
        num_zeros = 0;
    }
    return (next, num_zeros);
}

fn part2(histories: &Vec<i32>) -> i32 {
    let mut tracker = 50;
    let mut zeros = 0;
    for (_j, x) in histories.iter().enumerate() {
        let (t_j, z_j) = part2_iter(tracker, *x);
        tracker = t_j;
        zeros += z_j;
        // println!("{j}\t{tracker}\t{zeros}\t\t{x}");
    }
    return zeros;
}

pub fn day1(contents: &String) {
    let histories: Vec<i32> = contents.lines().map(get_number).collect();
    let p1 = part1(&histories);
    println!("Part 1: {p1}");
    let p2 = part2(&histories);
    println!("Part 2: {p2}");
}
