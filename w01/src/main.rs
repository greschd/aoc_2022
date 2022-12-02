use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let file_content = std::fs::read_to_string(path).expect("Could not read file!");

    let lines: Vec<&str> = file_content.split('\n').collect();

    let mut snacks: Vec<Vec<u32>> = vec![vec![]];

    for &line in lines.iter() {
        match line {
            "" => {
                snacks.push(vec![]);
            }
            _ => {
                let idx = snacks.len() - 1;
                snacks[idx].push(line.parse::<u32>().expect("Could not parse"));
            }
        }
    }
    let mut sums: Vec<u32> = vec![];
    for elf_snacks in snacks.iter() {
        let mut sum = 0;
        for item_snack in elf_snacks.iter() {
            sum += item_snack;
        }
        sums.push(sum);
    }
    let max_val = sums.iter().fold(0, |a, b| a.max(*b));
    sums.sort_by(|a, b| b.cmp(a));
    println!("{}", max_val);
    let sums_part = sums[0..=2].iter().fold(0, |a, b| a + b);
    println!("{}", sums_part);
}
