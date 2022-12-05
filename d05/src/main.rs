fn get_input() -> String {
    let args: Vec<String> = std::env::args().collect();
    let path = &args.get(1).expect("No file given");
    String::from(
        std::fs::read_to_string(path)
            .expect("Could not read file!")
            .clone(),
    )
}

fn get_result(state: &Vec<Vec<char>>) -> String {
    let mut res = String::new();
    for stack in state {
        res.push(*stack.last().unwrap());
    }
    res
}

fn main() {
    let input = get_input();

    let (initial, moves) = input.split_once("\n\n").unwrap();

    let mut state01: Vec<Vec<char>> = vec![];

    let mut initial_state_iter = initial.lines().rev();
    let index_line = initial_state_iter.next().unwrap();
    let num_stacks = index_line
        .split(" ")
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    for _ in 0..num_stacks {
        state01.push(vec![]);
    }
    for line in initial_state_iter {
        for i in 0..num_stacks {
            let char_idx = 1 + (4 * i);
            match line.chars().nth(char_idx) {
                None => (),
                Some(' ') => (),
                Some(value) => state01[i].push(value),
            }
        }
    }
    let mut state02 = state01.clone();

    for line in moves.lines() {
        if line == "" {
            continue;
        }
        let (_, rest) = line.split_once("move").unwrap();
        let (move_count, rest) = rest.split_once("from").unwrap();
        let move_count = move_count.trim().parse::<usize>().unwrap();
        let (from, to) = rest.split_once("to").unwrap();
        let from = from.trim().parse::<usize>().unwrap() - 1;
        let to = to.trim().parse::<usize>().unwrap() - 1;
        for _ in 0..move_count {
            let value = state01[from].pop().unwrap();
            state01[to].push(value);
        }
        let at_idx = state02[from].len() - move_count;
        let new_part = state02[from].split_off(at_idx);
        state02[to].extend(new_part);
    }

    println!("{}", get_result(&state01));
    println!("{}", get_result(&state02));
}
