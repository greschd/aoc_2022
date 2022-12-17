use std::collections::HashSet;

fn get_input() -> String {
    let args: Vec<String> = std::env::args().collect();
    let path = args.get(1).expect("No file given!");
    String::from(std::fs::read_to_string(path).expect("Could not read file!"))
}

fn parse_input(input: &str) -> HashSet<[i32; 2]> {
    let mut res = HashSet::<[i32; 2]>::new();
    for line in input.lines() {
        if line.len() > 0 {
            let mut points: Vec<[i32; 2]> = vec![];
            for part in line.split(" -> ") {
                let (l, r) = part.split_once(",").unwrap();
                let l = l.parse::<i32>().unwrap();
                let r = r.parse::<i32>().unwrap();
                points.push([l, r]);
            }
            let num_pts = points.len();
            for idx in 0..num_pts - 1 {
                let [i1, j1] = points[idx];
                let [i2, j2] = points[idx + 1];
                let mut ivec = [i1, i2];
                ivec.sort();
                let mut jvec = [j1, j2];
                jvec.sort();
                for i in ivec[0]..=ivec[1] {
                    for j in jvec[0]..=jvec[1] {
                        res.insert([i, j]);
                    }
                }
            }
        }
    }
    res
}

fn place_sand(occupied: &mut HashSet<[i32; 2]>, lowest_occupied: i32) -> bool {
    let (mut pos_i, mut pos_j) = (500, 0);
    while pos_j < lowest_occupied {
        if !occupied.contains(&[pos_i, pos_j + 1]) {
            pos_j += 1;
        } else if !occupied.contains(&[pos_i - 1, pos_j + 1]) {
            pos_i -= 1;
            pos_j += 1;
        } else if !occupied.contains(&[pos_i + 1, pos_j + 1]) {
            pos_i += 1;
            pos_j += 1;
        } else {
            occupied.insert([pos_i, pos_j]);
            return true;
        }
    }
    return false;
}

fn place_sand_p2(occupied: &mut HashSet<[i32; 2]>, lowest_occupied: i32) -> bool {
    let (mut pos_i, mut pos_j) = (500, 0);
    while pos_j < lowest_occupied + 1 {
        if !occupied.contains(&[pos_i, pos_j + 1]) {
            pos_j += 1;
        } else if !occupied.contains(&[pos_i - 1, pos_j + 1]) {
            pos_i -= 1;
            pos_j += 1;
        } else if !occupied.contains(&[pos_i + 1, pos_j + 1]) {
            pos_i += 1;
            pos_j += 1;
        } else {
            occupied.insert([pos_i, pos_j]);
            return [pos_i, pos_j] != [500, 0];
        }
    }
    occupied.insert([pos_i, pos_j]);
    return true;
}

fn main() {
    let input = get_input();
    let mut occupied = parse_input(&input);
    let mut lowest_occupied = 0;
    for [_, j] in &occupied {
        lowest_occupied = std::cmp::max(lowest_occupied, *j);
    }
    let mut count = 0;
    while place_sand(&mut occupied, lowest_occupied) {
        count += 1;
    }
    dbg!(count);
    while place_sand_p2(&mut occupied, lowest_occupied) {
        count += 1;
    }
    count += 1;
    dbg!(count);
}
