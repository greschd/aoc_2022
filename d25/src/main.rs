fn get_input() -> String {
    let args: Vec<String> = std::env::args().collect();
    let path = args.get(1).expect("No file given!");
    String::from(std::fs::read_to_string(path).expect("Could not read file!"))
}

fn snafu_to_int(snafu: &str) -> i64 {
    let mut res = 0;
    let mut multiplier = 1;
    for char in snafu.chars().rev() {
        match char {
            '0' => {}
            '1' => {
                res += multiplier;
            }
            '2' => {
                res += 2 * multiplier;
            }
            '-' => {
                res -= multiplier;
            }
            '=' => {
                res -= 2 * multiplier;
            }
            _ => {
                panic!("Invalid SNAFU literal!")
            }
        }
        multiplier *= 5;
    }
    res
}

fn representable_val(width: usize) -> i64 {
    let mut multiplier = 1;
    let mut res = 0;
    for _ in 0..width {
        res += 2 * multiplier;
        multiplier *= 5;
    }
    res
}

fn int_to_snafu(int: &i64) -> String {
    let mut remainder = *int;

    let mut width = 0;
    while representable_val(width) < remainder {
        width += 1;
    }
    dbg!(width);

    let mut res_str = String::new();
    for next_width in (0..width).rev() {
        let val_to_check: i64;
        if remainder > 0 {
            val_to_check = remainder + representable_val(next_width)
        } else {
            val_to_check = remainder - representable_val(next_width)
        }
        let multiplier = num::checked_pow(5, next_width).unwrap();
        let curr_val = val_to_check / multiplier;
        match curr_val {
            0 => {
                res_str.push('0');
            }
            1 => {
                res_str.push('1');
            }
            2 => {
                res_str.push('2');
            }
            -1 => {
                res_str.push('-');
            }
            -2 => {
                res_str.push('=');
            }
            _ => {
                panic!("Logic error!");
            }
        }
        remainder -= curr_val * multiplier;
    }
    if remainder != 0 {
        panic!("Non-zero remainder!");
    }
    res_str
}

fn main() {
    let input = get_input();
    let mut res = 0;
    for line in input.lines() {
        if line.len() > 0 {
            res += snafu_to_int(line)
        }
    }
    println!("{}", int_to_snafu(&res));
}
