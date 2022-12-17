use std::collections::HashSet;

fn get_input() -> String {
    let args: Vec<String> = std::env::args().collect();
    let path = args.get(1).expect("No file given!");
    String::from(std::fs::read_to_string(path).expect("Could not read file!"))
}

fn parse_coord(input: &str) -> i32 {
    let (_, tail) = input.split_once("=").unwrap();
    tail.parse::<i32>().unwrap()
}

fn parse_pos(input: &str) -> [i32; 2] {
    let (xpart, ypart) = input.split_once(", ").unwrap();
    [parse_coord(xpart), parse_coord(ypart)]
}

fn get_sensor_beacon_pairs(input: &str) -> Vec<[[i32; 2]; 2]> {
    let mut res: Vec<[[i32; 2]; 2]> = vec![];
    for line in input.lines() {
        let (_, tail) = line.split_once("Sensor at ").unwrap();
        let (sensor_part, beacon_part) = tail.split_once(": closest beacon is at ").unwrap();
        let sensor_pos = parse_pos(sensor_part);
        let beacon_pos = parse_pos(beacon_part);
        res.push([sensor_pos, beacon_pos]);
    }
    res
}

fn manhattan_distance(a: [i32; 2], b: [i32; 2]) -> i32 {
    (a[0] - b[0]).abs() + (a[1] - b[1]).abs()
}

fn main() {
    let input = get_input();
    let sensor_beacon_pairs = get_sensor_beacon_pairs(&input);
    // let target_line = 10;
    // for target_line in 0..4000000 {}
    let target_line = 2000000;
    let mut covered_pairs: Vec<[i32; 2]> = vec![];

    for &[sensor, beacon] in &sensor_beacon_pairs {
        let dist = manhattan_distance(sensor, beacon);
        let remainder_dist = dist - (target_line - sensor[1]).abs();
        if remainder_dist < 0 {
            continue;
        }
        covered_pairs.push([sensor[0] - remainder_dist, sensor[0] + remainder_dist]);
    }
    let mut beacons = HashSet::<[i32; 2]>::new();
    for [_, beacon] in sensor_beacon_pairs {
        beacons.insert(beacon);
    }
    let mut count = 0;
    for beacon in beacons {
        if beacon[1] == target_line {
            count -= 1;
        }
    }
    dbg!(count);
    covered_pairs.sort();
    let mut next_start = covered_pairs[0][0];
    for &[start, end] in &covered_pairs {
        let start_clamped = std::cmp::max(next_start, start);
        if start_clamped > end {
            continue;
        }
        next_start = end + 1;
        let added = (end - start_clamped) + 1;
        count += added;
    }
    dbg!(count);
}
