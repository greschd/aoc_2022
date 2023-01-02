use std::collections::{HashMap, HashSet};
use std::fmt::Debug;

fn get_input() -> String {
    let args: Vec<String> = std::env::args().collect();
    let path = args.get(1).expect("No file given!");
    String::from(std::fs::read_to_string(path).expect("Could not read file!"))
}

fn parse_input(input: &str) -> HashSet<[i32; 2]> {
    let mut res = HashSet::<[i32; 2]>::new();
    for (i, line) in input.lines().enumerate() {
        for (j, char) in line.chars().enumerate() {
            if char == '#' {
                res.insert([i as i32, j as i32]);
            }
        }
    }
    res
}

fn add<T: num::Integer + Copy + std::ops::AddAssign + Debug, const N: usize>(
    a: &[T; N],
    b: &[T; N],
) -> [T; N] {
    (0..N).map(|i| a[i] + b[i]).collect::<Vec<T>>().try_into().unwrap()
}

const SOUTH: [i32;2] = [1, 0];
const EAST: [i32;2] = [0, 1];
const NORTH: [i32;2] = [-1, 0];
const WEST: [i32;2] = [0, -1];
const SOUTHEAST: [i32;2] = [1, 1];
const SOUTHWEST: [i32;2] = [1, -1];
const NORTHEAST: [i32;2] = [-1, 1];
const NORTHWEST: [i32;2] = [-1, -1];
const ALL_NEIGHBOR_POS: [[i32;2];8] = [NORTH, NORTHWEST, WEST, SOUTHWEST, SOUTH, SOUTHEAST, EAST, NORTHEAST];
const DIRECTIONS: [[[i32;2];3];4] = [[NORTH, NORTHEAST, NORTHWEST], [SOUTH, SOUTHEAST, SOUTHWEST], [WEST, SOUTHWEST, NORTHWEST], [EAST, SOUTHEAST, NORTHEAST]];

fn get_proposed_step(elf: &[i32;2], iter_idx: usize, elves: &HashSet<[i32;2]>) -> [i32;2] {
    let mut has_neighbor = false;
    for pos_offset in ALL_NEIGHBOR_POS {
        let pos = add(elf, &pos_offset);
        if elves.contains(&pos) {
            has_neighbor = true;
            break;
        }
    }
    if !has_neighbor {
        return elf.clone();
    }
    let i = iter_idx % DIRECTIONS.len();
    'dir_loop: for dir in DIRECTIONS[i..4].iter().chain(DIRECTIONS[0..i].iter()) {
        for pos_offset in dir {
            let pos = add(elf, &pos_offset);
            if elves.contains(&pos) {
                continue 'dir_loop;
            }
        }
        return add(elf, &dir[0]);
    }
    return elf.clone();
}

fn get_bounds(elves: &HashSet<[i32;2]>) ->[[i32;2];2] {
    let mut xmin = 0;
    let mut xmax = 0;
    let mut ymin = 0;
    let mut ymax = 0;
    for elf in elves {
        xmin = std::cmp::min(elf[0], xmin);
        ymin = std::cmp::min(elf[1], ymin);
        xmax = std::cmp::max(elf[0], xmax);
        ymax = std::cmp::max(elf[1], ymax);
    }
    xmax += 1;
    ymax += 1;
    [[xmin, xmax], [ymin, ymax]]
}

fn print_elves(elves: &HashSet<[i32;2]>) {
    let [[xmin, xmax], [ymin, ymax]] = get_bounds(elves);
    let mut res = String::new();
    for x in xmin..xmax {
        for y in ymin..ymax {
            if elves.contains(&[x, y]) {
                res.push('#');
            } else {
                res.push('.')
            }
        }
        res.push('\n');
    }
    println!("{}", res);
}

fn main() {
    let input = get_input();
    let elves_initial = parse_input(&input);


    let mut elves = elves_initial.clone();

    for i in 0..10000 {
        if i == 10 {
            let [[xmin, xmax], [ymin, ymax]] = get_bounds(&elves);
            let area = (xmax - xmin) * (ymax - ymin);
            println!("Res P1: {}", area - (elves.len() as i32));
        }
        let mut proposed = HashMap::<[i32;2], i32>::new();
        let elves_prev = elves.clone();
        elves.clear();
        for elf in &elves_prev {
            let prop = get_proposed_step(elf, i, &elves_prev);
            *proposed.entry(prop).or_insert(0) += 1;
        }
        let mut elves_moving = false;
        for elf in &elves_prev {
            let prop = get_proposed_step(elf, i, &elves_prev);
            if *proposed.entry(prop.clone()).or_insert(0) > 1 {
                elves.insert(elf.clone());
            } else {
                if prop != *elf {
                    elves_moving = true;
                }
                elves.insert(prop);
            }
        }
        if !elves_moving {
            println!("Res P2: {}", i + 1);
            break;
        }
        if elves.len() != elves_prev.len() {
            panic!("We lost an elf! {} {}", elves.len(), elves_prev.len());
        }
    }


}
