#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// #[derive(Debug)]
// struct State {
//     head: (i32, i32),
//     tail: (i32, i32),
// }

fn get_input() -> String {
    let args: Vec<String> = std::env::args().collect();
    let path = &args.get(1).expect("No file given");
    String::from(std::fs::read_to_string(path).expect("Could not read file!"))
}

fn get_moves(input: &str) -> Vec<(Direction, u32)> {
    let mut res: Vec<(Direction, u32)> = vec![];
    for line in input.lines() {
        if line.len() > 0 {
            let (dir, count) = line.split_once(" ").unwrap();
            let dir = match dir {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => panic!("unknown direction"),
            };
            let count = count.parse::<u32>().unwrap();
            res.push((dir, count));
        }
    }
    res
}

fn perform_move(state: &mut Vec<(i32, i32)>, direction: &Direction) {
    match direction {
        Direction::Up => state[0].1 += 1,
        Direction::Down => state[0].1 -= 1,
        Direction::Left => state[0].0 -= 1,
        Direction::Right => state[0].0 += 1,
    }

    for i in 0..(state.len() - 1) {
        let head = &state[i].clone();
        let tail = &mut state[i + 1];
        let mut diff = (tail.0 - head.0, tail.1 - head.1);
        if diff.0.abs() < 2 && diff.1.abs() < 2 {
            continue;
        }
        diff.0 = std::cmp::max(-1, std::cmp::min(diff.0, 1));
        diff.1 = std::cmp::max(-1, std::cmp::min(diff.1, 1));
        tail.0 -= diff.0;
        tail.1 -= diff.1;
    }
}

fn count_tail_states(state: &mut Vec<(i32, i32)>, moves: &Vec<(Direction, u32)>) -> usize {
    let mut tail_states = std::collections::HashSet::<(i32, i32)>::new();
    for move_instruction in moves {
        let (direction, count) = move_instruction;
        for _ in 0..*count {
            perform_move(state, &direction);
            tail_states.insert(state.last().unwrap().clone());
        }
    }
    // dbg!(&state);
    tail_states.len()
}

fn main() {
    let input = get_input();

    let moves = get_moves(&input);
    let mut state_p1 = vec![(0, 0), (0, 0)];
    dbg!(count_tail_states(&mut state_p1, &moves));
    let mut state_p2 = vec![(0, 0); 10];
    dbg!(count_tail_states(&mut state_p2, &moves));
}
