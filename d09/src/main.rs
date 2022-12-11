#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct State {
    head: (i32, i32),
    tail: (i32, i32),
}

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

fn perform_move(state: &mut State, direction: &Direction) {
    match direction {
        Direction::Up => state.head.1 += 1,
        Direction::Down => state.head.1 -= 1,
        Direction::Left => state.head.0 -= 1,
        Direction::Right => state.head.0 += 1,
    }
    if state.tail.0 < (state.head.0 - 1) {
        state.tail.1 = state.head.1;
        state.tail.0 = state.head.0 - 1;
    } else if state.tail.0 > (state.head.0 + 1) {
        state.tail.1 = state.head.1;
        state.tail.0 = state.head.0 + 1;
    } else if state.tail.1 < (state.head.1 - 1) {
        state.tail.0 = state.head.0;
        state.tail.1 = state.head.1 - 1;
    } else if state.tail.1 > (state.head.1 + 1) {
        state.tail.0 = state.head.0;
        state.tail.1 = state.head.1 + 1;
    }
}

fn main() {
    let input = get_input();

    let moves = get_moves(&input);
    let mut state = State {
        head: (0, 0),
        tail: (0, 0),
    };
    let mut tail_states = std::collections::HashSet::<(i32, i32)>::new();
    for move_instruction in moves {
        let (direction, count) = move_instruction;
        for _ in 0..count {
            perform_move(&mut state, &direction);
            // dbg!(&state);
            tail_states.insert(state.tail.clone());
        }
    }
    // dbg!(tail_states);
    dbg!(tail_states.len());
}
