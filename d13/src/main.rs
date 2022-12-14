use std::cmp::Ordering;
use std::collections::VecDeque;

fn get_input() -> String {
    let args: Vec<String> = std::env::args().collect();
    let path = args.get(1).expect("No file given!");
    String::from(std::fs::read_to_string(path).expect("Could not read file!"))
}

#[derive(Debug, Clone)]
enum Signal {
    VALUE(u32),
    COLLECTION(Vec<Signal>),
}

impl Signal {
    fn cmp(&self, other: &Signal) -> Ordering {
        match (self, other) {
            (Signal::VALUE(x), Signal::VALUE(y)) => x.cmp(&y),
            (Signal::COLLECTION(_), Signal::VALUE(y)) => {
                let other_wrapped = Signal::COLLECTION(vec![Signal::VALUE(*y)]);
                self.cmp(&other_wrapped)
            }
            (Signal::VALUE(_), Signal::COLLECTION(_)) => other.cmp(&self).reverse(),
            (Signal::COLLECTION(myvec), Signal::COLLECTION(othervec)) => {
                let myvec_len = myvec.len();
                let othervec_len = othervec.len();
                let mut res: Option<Ordering> = None;
                for idx in 0..std::cmp::min(myvec_len, othervec_len) {
                    let curr_res = myvec[idx].cmp(&othervec[idx]);
                    if curr_res == Ordering::Equal {
                        continue;
                    }
                    res = Some(curr_res);
                    break;
                }
                if res == None {
                    res = Some(myvec_len.cmp(&othervec_len));
                }

                res.unwrap()
            }
        }
    }
}

fn parse_signal(char_iter: &mut VecDeque<char>) -> Signal {
    let first = char_iter.pop_front().unwrap();
    if first != '[' {
        panic!("incorrect start to signal");
    }
    let mut signal_vec: Vec<Signal> = vec![];
    loop {
        let val = char_iter.pop_front().unwrap();
        if val == ']' {
            break;
        } else if val == '[' {
            char_iter.push_front(val);
            signal_vec.push(parse_signal(char_iter));
        } else if val == ',' {
            continue;
        } else {
            let mut digit = String::new();
            digit.push(val);
            loop {
                let next = char_iter.pop_front().unwrap();
                match next {
                    '0'..='9' => {
                        digit.push(next);
                    }
                    _ => {
                        char_iter.push_front(next);
                        break;
                    }
                }
            }
            signal_vec.push(Signal::VALUE(digit.parse::<u32>().unwrap()));
        }
    }

    Signal::COLLECTION(signal_vec)
}

fn parse_line(line: &str) -> Signal {
    let mut char_deque = line.chars().collect::<VecDeque<char>>();
    parse_signal(&mut char_deque)
}

fn parse_input(input: &str) -> Vec<[Signal; 2]> {
    let mut res: Vec<[Signal; 2]> = vec![];
    let mut iter = input.lines();
    loop {
        if let Some(first) = iter.next() {
            let second = iter.next().unwrap();
            iter.next();
            res.push([parse_line(first), parse_line(second)]);
        } else {
            break;
        }
    }

    res
}

fn main() {
    let input = get_input();
    let signal_pairs = parse_input(&input);
    let mut sum = 0;
    for (i, pair) in signal_pairs.iter().enumerate() {
        let idx = i + 1;
        if pair[0].cmp(&pair[1]) != Ordering::Greater {
            sum += idx;
        }
    }
    dbg!(sum);

    let divider1 = parse_line("[[2]]");
    let divider2 = parse_line("[[6]]");
    let mut all_packets: Vec<Signal> = vec![];

    for [p1, p2] in signal_pairs.iter() {
        all_packets.push(p1.clone());
        all_packets.push(p2.clone());
    }
    let mut m1 = 1;
    let mut m2 = 2;
    for pack in all_packets.iter() {
        if divider1.cmp(pack) == Ordering::Greater {
            m1 += 1;
        }
        if divider2.cmp(pack) == Ordering::Greater {
            m2 += 1;
        }
    }
    dbg!(m1 * m2);
}
