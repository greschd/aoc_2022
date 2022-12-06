fn get_input() -> String {
    let args: Vec<String> = std::env::args().collect();
    let path = &args.get(1).expect("No file given");
    String::from(
        std::fs::read_to_string(path)
            .expect("Could not read file!")
            .trim(),
    )
}

fn get_marker(input: &str, marker_size: usize) -> usize {
    let mut window = std::collections::VecDeque::<char>::new();
    for (i, character) in input.chars().enumerate() {
        window.push_back(character);
        if window.len() > marker_size {
            window.pop_front();
        }
        let count_set: std::collections::HashSet<char> = window.clone().into_iter().collect();
        if count_set.len() == marker_size {
            return i + 1;
        }
    }
    return 0;
}

fn main() {
    let input = get_input();
    println!("{}", get_marker(&input, 4));
    println!("{}", get_marker(&input, 14));
}
