fn get_input() -> String {
    let args: Vec<String> = std::env::args().collect();
    let path = &args.get(1).expect("No file given");
    String::from(std::fs::read_to_string(path).expect("Could not read file!"))
}

fn pop_from_stack(stack: &mut Vec<usize>, sizes: &mut Vec<usize>) {
    let last_size = stack.pop().unwrap();
    sizes.push(last_size);
    if stack.len() > 0 {
        let stack_idx = stack.len() - 1;
        stack[stack_idx] += last_size;
    }
}

fn main() {
    let input = get_input();
    let mut lines = input.lines();
    let mut dir_stack: Vec<usize> = vec![];
    let mut dir_sizes: Vec<usize> = vec![];
    loop {
        match lines.next() {
            Some(curr_line) => {
                if curr_line.starts_with("$ ls") {
                    continue;
                } else if curr_line.starts_with("dir") {
                    continue;
                } else if curr_line.starts_with("$ cd ..") {
                    pop_from_stack(&mut dir_stack, &mut dir_sizes);
                } else if curr_line.starts_with("$ cd") {
                    dir_stack.push(0);
                } else if curr_line.len() > 0 {
                    let (size, _) = curr_line.split_once(" ").unwrap();
                    let size = size.trim().parse::<usize>().unwrap();
                    let idx = dir_stack.len() - 1;
                    dir_stack[idx] += size;
                }
            }
            None => {
                break;
            }
        }
    }
    while dir_stack.len() > 0 {
        pop_from_stack(&mut dir_stack, &mut dir_sizes);
    }

    let mut sum = 0;
    for &entry in dir_sizes.iter() {
        if entry <= 100000 {
            sum += entry;
        }
    }
    dbg!(sum);

    let total_used = dir_sizes.last().unwrap();
    let needed = total_used - (70000000 - 30000000);

    let mut smallest_sufficient = total_used.clone();
    for &entry in dir_sizes.iter() {
        if entry >= needed && smallest_sufficient > entry {
            smallest_sufficient = entry;
        }
    }
    dbg!(smallest_sufficient);

    dbg!(total_used);
    dbg!(needed);
}
