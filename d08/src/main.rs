fn get_input() -> String {
    let args: Vec<String> = std::env::args().collect();
    let path = &args.get(1).expect("No file given");
    String::from(std::fs::read_to_string(path).expect("Could not read file!"))
}

fn parse_input(input: String) -> Vec<Vec<i32>> {
    // let size = input.lines().next().unwrap().len();
    let mut res: Vec<Vec<i32>> = vec![];
    for line in input.lines() {
        if line.len() > 0 {
            let mut curr_vec: Vec<i32> = vec![];
            for character in line.chars() {
                curr_vec.push(character.to_digit(10).unwrap() as i32);
            }
            res.push(curr_vec);
        }
    }
    res
}

fn main() {
    let trees = parse_input(get_input());

    let size = trees.len();
    dbg!(trees[0].len());
    dbg!(size);

    let mut visible: Vec<Vec<bool>> = vec![vec![false; size]; size];

    // from left
    for i in 0..size {
        let mut curr_max = -1;
        for j in 0..size {
            if trees[i][j] > curr_max {
                visible[i][j] = true;
                curr_max = trees[i][j];
            }
        }
    }
    // from right
    for i in 0..size {
        let mut curr_max = -1;
        for j in (0..size).rev() {
            if trees[i][j] > curr_max {
                visible[i][j] = true;
                curr_max = trees[i][j];
            }
        }
    }
    // from top
    for j in 0..size {
        let mut curr_max = -1;
        for i in 0..size {
            if trees[i][j] > curr_max {
                visible[i][j] = true;
                curr_max = trees[i][j];
            }
        }
    }
    // from bottom
    for j in 0..size {
        let mut curr_max = -1;
        for i in (0..size).rev() {
            if trees[i][j] > curr_max {
                visible[i][j] = true;
                curr_max = trees[i][j];
            }
        }
    }
    let mut sum = 0;
    for line in &visible {
        for val in line {
            if *val {
                sum += 1;
            }
        }
    }
    dbg!(sum);

    let mut max_view_score = 0;
    for i in 0..size {
        for j in 0..size {
            let curr_size = trees[i][j];
            let mut left_score = 0;
            for k in (0..j).rev() {
                left_score += 1;
                if trees[i][k] >= curr_size {
                    break;
                }
            }
            let mut right_score = 0;
            for k in (j + 1)..size {
                right_score += 1;
                if trees[i][k] >= curr_size {
                    break;
                }
            }
            let mut top_score = 0;
            for k in (0..i).rev() {
                top_score += 1;
                if trees[k][j] >= curr_size {
                    break;
                }
            }
            let mut bottom_score = 0;
            for k in (i + 1)..size {
                bottom_score += 1;
                if trees[k][j] >= curr_size {
                    break;
                }
            }
            let curr_score = left_score * right_score * top_score * bottom_score;
            max_view_score = std::cmp::max(curr_score, max_view_score);
        }
    }
    dbg!(max_view_score);
}
