use std::{env, fs};

fn get_input() -> String {
    let args: Vec<String> = env::args().collect();
    let path = &args.get(1).expect("No file given");
    String::from(
        fs::read_to_string(path)
            .expect("Could not read file!")
            .trim()
            .clone(),
    )
}

fn parse_pair(x: &str) -> (i32, i32) {
    let (l, r) = x.split_once("-").unwrap();
    (
        l.parse::<i32>().expect("nan"),
        r.parse::<i32>().expect("nan"),
    )
}

fn contains_fully(outer_range: (i32, i32), inner_range: (i32, i32)) -> bool {
    (outer_range.0 <= inner_range.0) && (outer_range.1 >= inner_range.1)
}

fn full_overlap(r1: (i32, i32), r2: (i32, i32)) -> bool {
    contains_fully(r1, r2) || contains_fully(r2, r1)
}

fn partial_overlap(r1: (i32, i32), r2: (i32, i32)) -> bool {
    (r1.0 <= r2.1 && r1.1 >= r2.0) || (r2.0 <= r1.1 && r2.1 >= r1.0)
}

fn main() {
    let input = get_input();
    let mut sum_p1 = 0;
    let mut sum_p2 = 0;
    for line in input.split("\n") {
        let (left, right) = line.split_once(",").unwrap();
        let left = parse_pair(left);
        let right = parse_pair(right);
        if full_overlap(left, right) {
            sum_p1 += 1;
        }
        if partial_overlap(left, right) {
            sum_p2 += 1;
        }
    }
    dbg!(sum_p1);
    dbg!(sum_p2);
}
