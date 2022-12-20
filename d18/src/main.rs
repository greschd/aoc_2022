use std::collections::HashSet;

fn get_input() -> String {
    let args: Vec<String> = std::env::args().collect();
    let path = args.get(1).expect("No file given!");
    String::from(std::fs::read_to_string(path).expect("Could not read file!"))
}

fn parse_input(input: &str) -> HashSet<[i32; 3]> {
    let mut res = HashSet::<[i32; 3]>::new();
    for line in input.lines() {
        if line.len() > 0 {
            let (first, tail) = line.split_once(",").unwrap();
            let (second, third) = tail.split_once(",").unwrap();
            let first = first.parse::<i32>().unwrap();
            let second = second.parse::<i32>().unwrap();
            let third = third.parse::<i32>().unwrap();
            res.insert([first, second, third]);
        }
    }
    res
}

fn get_neighbors(point: &[i32; 3]) -> [[i32; 3]; 6] {
    [
        [point[0] + 1, point[1], point[2]],
        [point[0] - 1, point[1], point[2]],
        [point[0], point[1] + 1, point[2]],
        [point[0], point[1] - 1, point[2]],
        [point[0], point[1], point[2] + 1],
        [point[0], point[1], point[2] - 1],
    ]
}

fn get_surface_count(points: &HashSet<[i32; 3]>) -> i32 {
    let mut count = 0;
    for pt in points {
        for neighbor in get_neighbors(&pt) {
            if !points.contains(&neighbor) {
                count += 1;
            }
        }
    }
    count
}

fn p1(points_in_object: &HashSet<[i32; 3]>) {
    dbg!(get_surface_count(&points_in_object));
}

fn get_boundaries(points_in_object: &HashSet<[i32; 3]>) -> [[i32; 2]; 3] {
    let mut max_x = 0;
    let mut max_y = 0;
    let mut max_z = 0;
    let mut min_x = std::i32::MAX;
    let mut min_y = std::i32::MAX;
    let mut min_z = std::i32::MAX;
    for pt in points_in_object {
        max_x = std::cmp::max(pt[0], max_x);
        max_y = std::cmp::max(pt[1], max_y);
        max_z = std::cmp::max(pt[2], max_z);
        min_x = std::cmp::min(pt[0], min_x);
        min_y = std::cmp::min(pt[1], min_y);
        min_z = std::cmp::min(pt[2], min_z);
    }
    [
        [min_x - 1, max_x + 1],
        [min_y - 1, max_y + 1],
        [min_z - 1, max_z + 1],
    ]
}

fn within_bounds(bounds: &[[i32; 2]; 3], point: &[i32; 3]) -> bool {
    for i in 0..3 {
        if point[i] < bounds[i][0] || point[i] > bounds[i][1] {
            return false;
        }
    }
    return true;
}

fn p2(points_in_object: &HashSet<[i32; 3]>) {
    let bounds = get_boundaries(&points_in_object);
    let mut newly_encountered = HashSet::<[i32; 3]>::new();
    newly_encountered.insert([bounds[0][0], bounds[1][0], bounds[2][0]]);
    let mut all_outer = newly_encountered.clone();
    loop {
        let to_check = newly_encountered.clone();
        if to_check.len() == 0 {
            break;
        }
        newly_encountered.clear();
        for point in to_check {
            for neighbor in get_neighbors(&point) {
                if all_outer.contains(&neighbor) {
                    continue;
                }
                if !within_bounds(&bounds, &neighbor) {
                    continue;
                }
                if points_in_object.contains(&neighbor) {
                    continue;
                }
                all_outer.insert(neighbor.clone());
                newly_encountered.insert(neighbor);
            }
        }
    }
    let count_with_outer = get_surface_count(&all_outer);
    let cube_size: Vec<i32> = bounds.iter().map(|[lo, hi]| (hi - lo) + 1).collect();
    let bounds_surface: i32 = (0..3)
        .map(|i| cube_size[i] * cube_size[(i + 1) % 3])
        .sum::<i32>()
        * 2;
    dbg!(count_with_outer - bounds_surface);
}

fn main() {
    let input = get_input();
    let points_in_object = parse_input(&input);
    p1(&points_in_object);
    p2(&points_in_object);
}
