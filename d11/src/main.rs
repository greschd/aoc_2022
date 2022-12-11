use std::collections::VecDeque;

fn get_input() -> String {
    let args: Vec<String> = std::env::args().collect();
    let path = &args.get(1).expect("No file given");
    String::from(std::fs::read_to_string(path).expect("Could not read file!"))
}

#[derive(Debug, Clone)]
enum Operation {
    ADD(i64),
    MUL(i64),
    SQR,
}

#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<i64>,
    operation: Operation,
    test_div: i64,
    target_true: usize,
    target_false: usize,
}

fn get_items(line: &str) -> VecDeque<i64> {
    let mut items = VecDeque::<i64>::new();
    let (_, line) = line.split_once(": ").unwrap();
    for item in line.split(",") {
        let item = item.trim().parse::<i64>().unwrap();
        items.push_back(item);
    }
    items
}

fn get_operation(line: &str) -> Operation {
    let (_, op_str) = line.split_once("= old ").unwrap();
    let operation: Operation;
    if op_str.starts_with("* old") {
        operation = Operation::SQR;
    } else {
        let op_val = op_str[1..].trim().parse::<i64>().unwrap();
        if op_str.starts_with("*") {
            operation = Operation::MUL(op_val);
        } else {
            operation = Operation::ADD(op_val);
        }
    }
    operation
}

fn get_throw_target(line: &str) -> usize {
    line.split_once("throw to monkey")
        .unwrap()
        .1
        .trim()
        .parse()
        .unwrap()
}

fn get_monkeys(input: &str) -> Vec<Monkey> {
    let mut res: Vec<Monkey> = vec![];
    let mut line_iter = input.lines();
    loop {
        let line = line_iter.next();
        match line {
            Some(line) => {
                if !line.starts_with("Monkey") {
                    break;
                }
                let item_line = line_iter.next().unwrap();
                let items = get_items(&item_line);

                let op_line = line_iter.next().unwrap();
                let operation = get_operation(&op_line);

                let test_line = line_iter.next().unwrap();
                let test_div = test_line
                    .split_once("divisible by")
                    .unwrap()
                    .1
                    .trim()
                    .parse::<i64>()
                    .unwrap();

                let line_target_true = line_iter.next().unwrap();
                let target_true = get_throw_target(&line_target_true);

                let line_target_false = line_iter.next().unwrap();
                let target_false = get_throw_target(&line_target_false);

                line_iter.next();

                res.push(Monkey {
                    items: items,
                    operation: operation,
                    test_div: test_div,
                    target_true: target_true,
                    target_false: target_false,
                })
            }
            None => {
                break;
            }
        }
    }
    res
}

fn count_monkey_business(
    mut monkeys: Vec<Monkey>,
    worry_reducer: &dyn Fn(i64) -> i64,
    rounds: i32,
) -> u64 {
    let num_monkeys = monkeys.len();
    let mut counter: Vec<u64> = vec![0; num_monkeys];
    for _ in 0..rounds {
        for i in 0..num_monkeys {
            while monkeys[i].items.len() > 0 {
                counter[i] += 1;
                let mut curr_item = monkeys[i].items.pop_front().unwrap();
                match monkeys[i].operation {
                    Operation::ADD(x) => {
                        curr_item += x;
                    }
                    Operation::MUL(x) => {
                        curr_item *= x;
                    }
                    Operation::SQR => {
                        curr_item *= curr_item;
                    }
                }
                curr_item = worry_reducer(curr_item);
                let target: usize;
                if curr_item % monkeys[i].test_div == 0 {
                    target = monkeys[i].target_true;
                } else {
                    target = monkeys[i].target_false;
                }
                monkeys[target].items.push_back(curr_item);
            }
        }
    }
    counter.sort();
    counter.reverse();
    counter[0..2].iter().fold(1, |x, y| x * y)
}

fn main() {
    let input = get_input();
    let monkeys = get_monkeys(&input);
    let divide_by_three = |x| x / 3;
    dbg!(count_monkey_business(monkeys.clone(), &divide_by_three, 20));

    let mut lcm = 1;
    for m in &monkeys {
        lcm = num::integer::lcm(lcm, m.test_div);
    }
    let mod_by_lcm = |x| x % lcm;
    dbg!(count_monkey_business(monkeys.clone(), &mod_by_lcm, 10000));
}
