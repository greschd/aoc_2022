use std::collections::HashSet;

fn get_input() -> String {
    let args: Vec<String> = std::env::args().collect();
    let path = args.get(1).expect("No file given!");
    String::from(std::fs::read_to_string(path).expect("Could not read file!"))
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    let mut res: Vec<Vec<char>> = vec![];
    for line in input.lines() {
        if line.len() == 0 {
            break;
        }
        res.push(line.chars().collect());
    }
    res
}

fn get_candidate_points(pt: &[usize; 2], ysize: usize, xsize: usize) -> Vec<[usize; 2]> {
    let mut candidate_points: Vec<[usize; 2]> = vec![];
    if pt[0] > 0 {
        candidate_points.push([pt[0] - 1, pt[1]]);
    }
    if pt[1] > 0 {
        candidate_points.push([pt[0], pt[1] - 1]);
    }
    if pt[0] < (ysize - 1) {
        candidate_points.push([pt[0] + 1, pt[1]]);
    }
    if pt[1] < (xsize - 1) {
        candidate_points.push([pt[0], pt[1] + 1]);
    }
    candidate_points
}

fn get_shortest_path_p1(field: &Vec<Vec<char>>, start: &[usize; 2], end: &[usize; 2]) -> usize {
    let ysize = field.len();
    let xsize = field[0].len();

    let mut distances: Vec<Vec<Option<usize>>> = vec![vec![None; xsize]; ysize];

    distances[start[0]][start[1]] = Some(0);
    let mut newly_reached_points = HashSet::<[usize; 2]>::new();
    newly_reached_points.insert(*start);
    let mut curr_value = 0;
    while newly_reached_points.len() > 0 {
        curr_value += 1;
        let iter_points = newly_reached_points.clone();
        newly_reached_points.clear();
        for pt in iter_points {
            let current_height = field[pt[0]][pt[1]] as i16;
            let candidate_points = get_candidate_points(&pt, ysize, xsize);
            for new_pt in candidate_points {
                let new_height = field[new_pt[0]][new_pt[1]] as i16;
                if new_height - current_height <= 1 {
                    if new_pt == *end {
                        return curr_value;
                    }
                    let dist_at_pt = distances[new_pt[0]][new_pt[1]];
                    match dist_at_pt {
                        None => {
                            distances[new_pt[0]][new_pt[1]] = Some(curr_value);
                            newly_reached_points.insert(new_pt);
                        }
                        Some(_) => {
                            continue;
                        }
                    }
                }
            }
        }
    }
    panic!("Not found!")
}

fn get_shortest_hike_p2(field: &Vec<Vec<char>>, end: &[usize; 2]) -> usize {
    let ysize = field.len();
    let xsize = field[0].len();

    let mut distances: Vec<Vec<Option<usize>>> = vec![vec![None; xsize]; ysize];

    distances[end[0]][end[1]] = Some(0);
    let mut newly_reached_points = HashSet::<[usize; 2]>::new();
    newly_reached_points.insert(*end);
    let mut curr_value = 0;
    while newly_reached_points.len() > 0 {
        curr_value += 1;
        let iter_points = newly_reached_points.clone();
        newly_reached_points.clear();
        for pt in iter_points {
            let current_height = field[pt[0]][pt[1]] as i16;
            let candidate_points = get_candidate_points(&pt, ysize, xsize);
            for new_pt in candidate_points {
                let new_height = field[new_pt[0]][new_pt[1]] as i16;
                if current_height - new_height <= 1 {
                    if new_height == 'a' as i16 {
                        return curr_value;
                    }
                    let dist_at_pt = distances[new_pt[0]][new_pt[1]];
                    match dist_at_pt {
                        None => {
                            distances[new_pt[0]][new_pt[1]] = Some(curr_value);
                            newly_reached_points.insert(new_pt);
                        }
                        Some(_) => {
                            continue;
                        }
                    }
                }
            }
        }
    }
    panic!("Not found!");
}

fn main() {
    let input = get_input();
    let mut field = parse_input(&input);

    let mut start: Option<[usize; 2]> = None;
    let mut end: Option<[usize; 2]> = None;

    for (i, line) in field.iter().enumerate() {
        for (j, val) in line.iter().enumerate() {
            if *val == 'S' {
                start = Some([i, j]);
            } else if *val == 'E' {
                end = Some([i, j]);
            }
        }
    }

    let start = start.unwrap();
    let end = end.unwrap();
    field[start[0]][start[1]] = 'a';
    field[end[0]][end[1]] = 'z';
    dbg!(get_shortest_path_p1(&field, &start, &end));
    dbg!(get_shortest_hike_p2(&field, &end));
}
