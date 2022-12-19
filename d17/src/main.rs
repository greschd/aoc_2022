use std::collections::HashMap;
use std::collections::HashSet;

fn get_input() -> String {
    let args: Vec<String> = std::env::args().collect();
    let path = args.get(1).expect("No file given!");
    String::from(
        std::fs::read_to_string(path)
            .expect("Could not read file!")
            .trim(),
    )
}

struct BlockIter {
    curr_idx: usize,
}

impl Iterator for BlockIter {
    type Item = HashSet<[i64; 2]>;

    fn next(&mut self) -> Option<Self::Item> {
        let blocks: [HashSet<[i64; 2]>; 5] = [
            Self::Item::from([[0, 0], [1, 0], [2, 0], [3, 0]]),
            Self::Item::from([[1, 0], [0, 1], [1, 1], [2, 1], [1, 2]]),
            Self::Item::from([[0, 0], [1, 0], [2, 0], [2, 1], [2, 2]]),
            Self::Item::from([[0, 0], [0, 1], [0, 2], [0, 3]]),
            Self::Item::from([[0, 0], [0, 1], [1, 0], [1, 1]]),
        ];

        let idx = self.curr_idx;
        self.curr_idx = (self.curr_idx + 1) % 5;
        Some(blocks[idx].clone())
    }
}

enum Move {
    LEFT,
    RIGHT,
}

struct MoveIter {
    curr_idx: usize,
    chars: Vec<char>,
}

impl Iterator for MoveIter {
    type Item = Move;

    fn next(&mut self) -> Option<Self::Item> {
        let idx = self.curr_idx;
        self.curr_idx = (self.curr_idx + 1) % self.chars.len();
        match self.chars[idx] {
            '>' => Some(Move::RIGHT),
            '<' => Some(Move::LEFT),
            val => panic!("Unknown char {}", val),
        }
    }
}

fn get_height(field: &HashSet<[i64; 2]>) -> i64 {
    let mut height = 0;
    for entry in field {
        height = std::cmp::max(height, entry[1]);
    }
    height
}

fn get_lowest(field: &HashSet<[i64; 2]>) -> i64 {
    let mut height = std::i64::MAX;
    for entry in field {
        height = std::cmp::min(height, entry[1]);
    }
    height
}

fn check_offset(field: &HashSet<[i64; 2]>, block: &HashSet<[i64; 2]>, offset: &[i64; 2]) -> bool {
    if offset[1] <= 0 {
        return false;
    }
    for entry in block {
        let entry_with_offset = [offset[0] + entry[0], offset[1] + entry[1]];
        if (entry_with_offset[0] < 0) || (entry_with_offset[0] > 6) {
            return false;
        }
        if field.contains(&entry_with_offset) {
            return false;
        }
    }
    return true;
}

fn place_block(field: &mut HashSet<[i64; 2]>, block: HashSet<[i64; 2]>, move_iter: &mut MoveIter) {
    let mut offset = [2, 4 + get_height(&field)];
    loop {
        let offset_candidate = match move_iter.next().unwrap() {
            Move::LEFT => [offset[0] - 1, offset[1]],
            Move::RIGHT => [offset[0] + 1, offset[1]],
        };
        if check_offset(&field, &block, &offset_candidate) {
            offset = offset_candidate;
        }
        let offset_candidate = [offset[0], offset[1] - 1];
        if check_offset(&field, &block, &offset_candidate) {
            offset = offset_candidate;
        } else {
            for entry in block {
                field.insert([offset[0] + entry[0], offset[1] + entry[1]]);
            }
            return;
        }
    }
}

fn get_full_line(field: &HashSet<[i64; 2]>) -> Option<i64> {
    let height = get_height(&field);
    let lowest = get_lowest(&field);
    'outer: for j in (lowest..=height).rev() {
        for i in 0..7 {
            if !field.contains(&[i, j]) {
                continue 'outer;
            }
        }
        return Some(j);
    }
    None
}

fn get_height_after_block(num_blocks: usize) -> i64 {
    let input = get_input();
    let char_vec: Vec<char> = input.chars().collect();
    let block_iter = BlockIter { curr_idx: 0 };
    let mut move_iter = MoveIter {
        curr_idx: 0,
        chars: char_vec.clone(),
    };
    let mut field = HashSet::<[i64; 2]>::new();
    let mut seen_states = HashMap::<(usize, usize), (usize, i64)>::new();
    let mut target_idx: usize = num_blocks;
    let mut offset_height: i64 = 0;
    for (i, block) in block_iter.enumerate() {
        if i == target_idx {
            break;
        }
        place_block(&mut field, block, &mut move_iter);
        if (offset_height == 0) && (get_full_line(&field) == Some(get_height(&field))) {
            let curr_state = (i % 5, move_iter.curr_idx);
            if seen_states.contains_key(&curr_state) {
                let (i1, height1) = seen_states[&curr_state];
                let istep = i - i1;
                let height_step = get_height(&field) - height1;
                let remaining = target_idx - i;
                let skip_num = remaining / istep;
                let remaining_new = remaining % istep;
                offset_height = (skip_num as i64) * height_step;
                target_idx = i + remaining_new;
            } else {
                seen_states.insert(curr_state, (i, get_height(&field)));
            }
        }
    }
    get_height(&field) + offset_height
}

fn main() {
    println!("p1: {}", get_height_after_block(2022));
    println!("p2: {}", get_height_after_block(1000000000000));
}
