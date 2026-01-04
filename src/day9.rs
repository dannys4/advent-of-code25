use geo::{coord, point, Contains, LineString, Polygon, Rect};

fn process_line(line: &str) -> (u64, u64) {
    let n = |x: Option<&str>| x.unwrap().parse::<u64>().unwrap();
    let mut sp = line.split(',');
    return (n(sp.next()), n(sp.next()));
}

fn process_contents(contents: &String) -> Vec<(u64, u64)> {
    return contents.split_whitespace().map(process_line).collect();
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

fn part2_geom(coords: &Vec<(u64, u64)>) -> u64 {
    let points: Vec<_> = coords
        .iter()
        .map(|c| {
            point!(
                x: c.0 as f32,
                y: c.1 as f32
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
                    coord! {x: c_i.0 as f32, y: c_i.1 as f32},
                    coord! {x: c_j.0 as f32, y: c_j.1 as f32},
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
    let p1 = part1(&coords);
    println!("Part 1: {p1}");
    let p2 = part2_geom(&coords);
    println!("Part 2: {p2}");
}
