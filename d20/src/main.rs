#![feature(linked_list_cursors)]
#![feature(linked_list_remove)]

use std::collections::LinkedList;

fn get_input() -> String {
    let args: Vec<String> = std::env::args().collect();
    let path = args.get(1).expect("No file given!");
    String::from(std::fs::read_to_string(path).expect("Could not read file!"))
}

fn parse_input(input: &str) -> Vec<i64> {
    let mut res = Vec::<i64>::new();
    for line in input.lines() {
        if line.len() > 0 {
            res.push(line.parse::<i64>().unwrap());
        }
    }
    res
}

fn move_value(index: usize, move_value: i64, list: &mut LinkedList<usize>) {
    let mut cursor = list.cursor_front_mut();
    loop {
        match cursor.current() {
            Some(val) => {
                if *val == index {
                    break;
                }
                cursor.move_next();
            }
            None => {
                panic!("end of list");
            }
        }
    }
    cursor.remove_current();
    if cursor.current() == None {
        cursor.move_next();
    }
    if move_value < 0 {
        for _ in 0..-move_value {
            cursor.move_prev();
            if cursor.current() == None {
                cursor.move_prev();
            }
        }
    } else {

        for _ in 0..move_value {
            cursor.move_next();
            if cursor.current() == None {
                cursor.move_next();
            }
        }
    }
    cursor.insert_before(index);
}

fn run_puzzle(input_vec: &Vec<i64>, num_runs: usize) -> i64 {
    let list_len = input_vec.len();
    let mod_list: Vec<i64> = input_vec
        .iter()
        .map(|x| x % ((list_len as i64) - 1))
        .collect();
    let mut changing_list: LinkedList<usize> = (0..list_len).collect();

    for _ in 0..num_runs {
        for (index, move_val) in mod_list.iter().enumerate() {
            move_value(index, move_val.clone(), &mut changing_list);
        }
    }

    let zero_index = input_vec.iter().position(|&x| x == 0).unwrap();
    let mut cursor = changing_list.cursor_front();
    loop {
        if cursor.current() == Some(&zero_index) {
            break;
        }
        cursor.move_next();
    }
    let mut score = 0;

    for _ in 0..3 {
        for _ in 0..1000 {
            cursor.move_next();
            if cursor.current() == None {
                cursor.move_next();
            }
        }
        let curr_idx = cursor.current().unwrap();
        let curr_val = input_vec[*curr_idx];
        // dbg!(curr_val * 811589153);
        score += curr_val;
    }
    return score;
}

fn main() {
    let input = get_input();
    let initial_list = parse_input(&input);
    dbg!(run_puzzle(&initial_list, 1));

    let decryption_key: i64 = 811589153;
    let new_initial_list: Vec<i64> = initial_list.iter().map(|x| *x * decryption_key).collect();
    dbg!(run_puzzle(&new_initial_list, 10));
}
