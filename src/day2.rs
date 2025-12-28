use pcre2::bytes::Regex;

fn iter_id(id: String, re: &Regex) -> i64 {
    let is_match = re.is_match(id.as_bytes()).unwrap();
    if is_match {
        return id.parse::<i64>().unwrap();
    } else {
        return 0;
    }
}

fn process_range(range: &str, re: &Regex) -> i64 {
    let mut split_str = range.split("-");
    let (l, r) = (
        split_str.next().unwrap().parse::<i64>().unwrap(),
        split_str.next().unwrap().parse::<i64>().unwrap(),
    );
    let mut ret = 0;
    for i in l..(r + 1) {
        ret += iter_id(i.to_string(), re);
    }
    return ret;
}

fn part1(ranges: &Vec<&str>) -> i64 {
    let mut ret = 0;
    let re = Regex::new(r"^(?P<dd>[1-9]\d*)\g{-1}$").unwrap();
    for range in ranges {
        ret += process_range(range, &re);
    }
    return ret;
}

fn part2(ranges: &Vec<&str>) -> i64 {
    let mut ret = 0;
    let re = Regex::new(r"^(?P<dd>[1-9]\d*)\g{-1}+$").unwrap();
    for range in ranges {
        ret += process_range(range, &re);
    }
    return ret;
}

pub fn day2(contents: &String) {
    let ranges: Vec<&str> = contents.split(",").map(|x| x.trim()).collect();
    let p1 = part1(&ranges);
    println!("Part 1: {p1}");
    let p2 = part2(&ranges);
    println!("Part 2: {p2}");
}
