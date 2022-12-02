use std::env;

fn win_score(theirs: i32, mine: i32) -> i32 {
    let diff = (3 + mine - theirs) % 3;
    match diff {
        0 => 3,
        1 => 6,
        2 => 0,
        _ => panic!("wut"),
    }
}

fn p1(lines: Vec<&str>) {
    let mut strategy: Vec<(i32, i32)> = vec![];
    for line in lines {
        if line != "" {
            let mut iter = line.splitn(2, ' ');
            let theirs = iter.next().unwrap();
            let theirs = match theirs {
                "A" => 0,
                "B" => 1,
                "C" => 2,
                _ => panic!("invalid input"),
            };
            let mine = iter.next().unwrap();
            let mine = match mine {
                "X" => 0,
                "Y" => 1,
                "Z" => 2,
                _ => panic!("invalid input"),
            };
            strategy.push((theirs, mine));
        }
    }
    let mut score = 0;
    for (theirs, mine) in strategy {
        score += win_score(theirs, mine);
        score += mine + 1;
    }
    dbg!(score);
}

fn p2(lines: Vec<&str>) {
    let mut score = 0;
    for line in lines {
        if line != "" {
            let mut iter = line.splitn(2, ' ');
            let theirs = iter.next().unwrap();
            let theirs = match theirs {
                "A" => 0,
                "B" => 1,
                "C" => 2,
                _ => panic!("invalid input"),
            };
            let mine_win_lose = iter.next().unwrap();
            let mine = match mine_win_lose {
                "X" => {
                    score += 0;
                    (theirs + 2) % 3
                }
                "Y" => {
                    score += 3;
                    theirs
                }
                "Z" => {
                    score += 6;
                    (theirs + 1) % 3
                }
                _ => panic!("invalid input"),
            };
            score += mine + 1;
        }
    }
    dbg!(score);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args.get(1).expect("No file given");
    let file_content = std::fs::read_to_string(path).expect("Could not read file!");
    let lines = file_content.split("\n").collect();
    p1(lines);
    let lines = file_content.split("\n").collect();
    p2(lines);
}
