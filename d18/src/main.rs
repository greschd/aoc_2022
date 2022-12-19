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

fn p1(points_in_object: &HashSet<[i32; 3]>) {
    let mut count = 0;
    for point in points_in_object {
        for neighbor in [
            [point[0] + 1, point[1], point[2]],
            [point[0] - 1, point[1], point[2]],
            [point[0], point[1] + 1, point[2]],
            [point[0], point[1] - 1, point[2]],
            [point[0], point[1], point[2] + 1],
            [point[0], point[1], point[2] - 1],
        ] {
            if !points_in_object.contains(&neighbor) {
                count += 1;
            }
        }
    }
    dbg!(count);
}

// TODO: marching cubes (outer) for p2

fn main() {
    let input = get_input();
    let points_in_object = parse_input(&input);
    p1(&points_in_object);
}
