#[derive(Debug)]
enum Instruction {
    NOOP,
    ADD(i32),
}

fn get_input() -> String {
    let args: Vec<String> = std::env::args().collect();
    let path = &args.get(1).expect("No file given");
    String::from(std::fs::read_to_string(path).expect("Could not read file!"))
}

fn get_instructions(input: &str) -> Vec<Instruction> {
    let mut res: Vec<Instruction> = vec![];
    for line in input.lines() {
        if line.starts_with("noop") {
            res.push(Instruction::NOOP)
        } else if line.starts_with("addx") {
            let (_, val) = line.split_once(" ").unwrap();
            let val = val.trim().parse::<i32>().unwrap();
            res.push(Instruction::ADD(val));
        }
    }
    res
}

fn extend_to_cycles(instructions: &Vec<Instruction>) -> Vec<Instruction> {
    let mut res: Vec<Instruction> = vec![];
    for instr in instructions {
        match instr {
            Instruction::NOOP => {
                res.push(Instruction::NOOP);
            }
            Instruction::ADD(x) => {
                res.push(Instruction::NOOP);
                res.push(Instruction::ADD(*x));
            }
        }
    }
    res
}

fn main() {
    let input = get_input();
    let instructions = get_instructions(&input);
    let extended_instructions = extend_to_cycles(&instructions);
    let mut x_register = 1;
    let mut cycle = 0;
    let mut signal_strength_sum = 0;
    // let mut should_draw: Vec<bool> = vec![];
    let mut screen = String::new();
    for instr in extended_instructions {
        cycle += 1;
        if ((cycle + 20) % 40) == 0 {
            signal_strength_sum += cycle * x_register;
        }
        let dist: i32 = (x_register + 1) - (cycle % 40);
        if dist.abs() <= 1 {
            screen.push('#');
        } else {
            screen.push('.');
        }
        if cycle % 40 == 0 {
            screen.push('\n');
        }
        // should_draw.push(dist.abs() <= 1);
        if let Instruction::ADD(val) = instr {
            x_register += val;
        }
    }
    dbg!(signal_strength_sum);

    println!("{}", screen);
}
