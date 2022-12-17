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

fn p1(input: &str, target_line: i32) {
    let sensor_beacon_pairs = get_sensor_beacon_pairs(&input);
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

fn p2(input: &str, bounds: i32) -> [i32; 2] {
    let sensor_beacon_pairs = get_sensor_beacon_pairs(&input);
    let mut beacons = HashSet::<[i32; 2]>::new();
    for &[_, beacon] in &sensor_beacon_pairs {
        beacons.insert(beacon);
    }
    for line_idx in 0..bounds {
        let mut covered_pairs: Vec<[i32; 2]> = vec![];

        for &[sensor, beacon] in &sensor_beacon_pairs {
            let dist = manhattan_distance(sensor, beacon);
            let remainder_dist = dist - (line_idx - sensor[1]).abs();
            if remainder_dist < 0 {
                continue;
            }
            covered_pairs.push([sensor[0] - remainder_dist, sensor[0] + remainder_dist]);
        }

        covered_pairs.sort();
        let mut next_start = 0;
        for &[start, end] in &covered_pairs {
            let start_clamped = std::cmp::max(0, start);
            let end_clamped = std::cmp::min(bounds, end);
            if next_start < start_clamped {
                for j in next_start..start_clamped {
                    let candidate = [line_idx, j];
                    if !beacons.contains(&candidate) {
                        return candidate;
                    }
                }
            }
            let start_clamped = std::cmp::max(next_start, start_clamped);
            if start_clamped > end_clamped {
                continue;
            }
            next_start = end_clamped + 1;
            if next_start > bounds {
                break;
            }
        }
    }
    panic!("beacon not found");
}

fn main() {
    let input = get_input();

    // p1(&input, 10);
    // let [y, x] = p2(&input, 20);
    p1(&input, 2000000);
    let [y, x] = p2(&input, 4000000);
    dbg!(y, x);
    dbg!(4000000 * (x as i64) + (y as i64));
}
