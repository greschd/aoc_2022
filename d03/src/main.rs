use std::env;

fn value_from_char(x: &u8) -> i32 {
    let offset: i32;
    if x >= &b'a' {
        offset = (b'a' as i32) - 1;
    } else {
        offset = (b'A' as i32) - 27;
    }
    (x.clone() as i32) - offset
}

fn p01(file_content: &String) {
    let mut sum = 0;
    for line in file_content.split("\n") {
        if line != "" {
            let count = line.len();
            let half_idx = count / 2;
            let left = std::collections::HashSet::<u8>::from_iter(
                line.as_bytes()[..half_idx].into_iter().cloned(),
            );
            let right = std::collections::HashSet::<u8>::from_iter(
                line.as_bytes()[half_idx..].into_iter().cloned(),
            );
            for common in left.intersection(&right) {
                let value = value_from_char(common);
                sum += value;
            }
        }
    }
    println!("{}", sum);
}

fn p02(file_content: &String) {
    let mut sum = 0;
    let mut sequence = file_content.split("\n");
    loop {
        fn to_hashset(x: &str) -> std::collections::HashSet<u8> {
            std::collections::HashSet::<u8>::from_iter(x.as_bytes().into_iter().cloned())
        }
        let first = sequence.next().expect("Exceptional emptiness encountered");
        if first == "" {
            break;
        }
        let first = to_hashset(first);
        let second = sequence.next().expect("Exceptional emptiness encountered");
        let second = to_hashset(second);
        let third = sequence.next().expect("Exceptional emptiness encountered");
        let third = to_hashset(third);
        for common in &(&first & &second) & &third {
            sum += value_from_char(&common);
        }
    }
    println!("{}", sum);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args.get(1).expect("No file given");
    let file_content = std::fs::read_to_string(path).expect("Could not read file!");

    p01(&file_content);
    p02(&file_content);
}
