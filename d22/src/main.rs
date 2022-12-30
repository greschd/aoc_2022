use num::Zero;

fn get_input() -> String {
    let args: Vec<String> = std::env::args().collect();
    let path = args.get(1).expect("No file given!");
    String::from(std::fs::read_to_string(path).expect("Could not read file!"))
}

#[derive(Debug)]
enum Tile {
    Void,
    Open,
    Wall,
}

fn gemm<
    T: num::Integer + Copy + std::ops::AddAssign,
    const N: usize,
    const M: usize,
    const L: usize,
>(
    a: [[T; N]; M],
    b: [[T; L]; N],
) -> [[T; L]; M] {
    let mut res: [[T; L]; M] = [[T::zero(); L]; M];
    for i in 0..M {
        for k in 0..N {
            for j in 0..L {
                res[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    res
}

fn gemv<T: num::Integer + Copy + std::ops::AddAssign, const N: usize, const M: usize>(
    a: [[T; M]; N],
    b: [T; M],
) -> [T; N] {
    let mut res: [T; N] = [T::zero(); N];
    for i in 0..N {
        for j in 0..M {
            res[i] += a[i][j] * b[j];
        }
    }
    res
}

#[derive(Debug)]
enum Move {
    Advance(i32),
    TurnClockwise,
    TurnCounterclockwise,
}



fn parse_input(input: &str) -> (Vec<Vec<Tile>>, Vec<Move>) {
    let (field, moves) = input.split_once("\n\n").unwrap();
    let width = field.lines().fold(0, |x, y| std::cmp::max(x, y.len()));
    dbg!(width);

    let mut res_field: Vec<Vec<Tile>> = vec![];
    for line in field.lines() {
        if line.len() == 0 {
            continue;
        }
        let mut curr_vec: Vec<Tile> = vec![];
        for char in line.chars() {
            match char {
                ' ' => {
                    curr_vec.push(Tile::Void);
                }
                '.' => {
                    curr_vec.push(Tile::Open);
                }
                '#' => {
                    curr_vec.push(Tile::Wall);
                }
                unknown => {
                    panic!("Invalid char: {}", unknown);
                }
            }
        }
        for _ in line.len()..width {
            curr_vec.push(Tile::Void);
        }
        if curr_vec.len() != width {
            panic!("o.O");
        }
        res_field.push(curr_vec);
    }

    let mut res_moves: Vec<Move> = vec![];

    let mut move_size_str = String::new();
    for char in moves.trim().chars() {
        match char {
            'R' => {
                if move_size_str.len() > 0 {
                    res_moves.push(Move::Advance(move_size_str.parse::<i32>().unwrap()));
                }
                move_size_str.clear();
                res_moves.push(Move::TurnClockwise);
            }
            'L' => {
                if move_size_str.len() > 0 {
                    res_moves.push(Move::Advance(move_size_str.parse::<i32>().unwrap()));
                }
                move_size_str.clear();
                res_moves.push(Move::TurnCounterclockwise);
            }
            _ => {
                move_size_str.push(char);
            }
        }
    }
    if move_size_str.len() > 0 {
        res_moves.push(Move::Advance(move_size_str.parse::<i32>().unwrap()));
    }

    (res_field, res_moves)
}

// #[derive(Debug, Clone)]
// enum Direction {
type Direction = [i32;2];
const LEFT: Direction = [-1, 0];
const RIGHT: Direction = [1, 0];
const UP:Direction = [0, 1];
const DOWN: Direction = [0, -1];


#[derive(Debug, Clone)]
struct Location {
    position: [usize; 2],
    direction: Direction,
}

fn make_move(
    field: &Vec<Vec<Tile>>,
    initial_location: &Location,
    move_to_execute: &Move,
) -> Location {
    if matches!(
        move_to_execute,
        Move::TurnClockwise | Move::TurnCounterclockwise
    ) {
        let new_direction = match move_to_execute {
            Move::TurnClockwise => match initial_location.direction {
                UP => RIGHT,
                RIGHT => DOWN,
                DOWN => LEFT,
                LEFT => UP,
                _ => {panic!("Invalid direction!");}

            },
            Move::TurnCounterclockwise => match initial_location.direction {
                UP => LEFT,
                LEFT => DOWN,
                DOWN => RIGHT,
                RIGHT => UP,
                _ => {panic!("Invalid direction!");}
            },
            _ => {
                panic!("Logic error!");
            }
        };
        return Location {
            position: initial_location.position.clone(),
            direction: new_direction,
        };
    }
    if let Move::Advance(num_steps) = move_to_execute {
        let mut position = initial_location.position.clone();
        for _ in 0..*num_steps {
            let new_pos = get_next_pos(field, position, &initial_location.direction);
            match new_pos {
                None => {
                    break;
                }
                Some(value) => {
                    position = value;
                }
            }
        }
        return Location {
            position: position,
            direction: initial_location.direction.clone(),
        };
    }
    panic!("Logic error!");
}

fn get_next_pos(
    field: &Vec<Vec<Tile>>,
    position: [usize; 2],
    direction: &Direction,
) -> Option<[usize; 2]> {
    let mut candidate_pos = position.clone();
    loop {
        candidate_pos = get_candidate_pos(field, candidate_pos, direction);
        let new_val = &field[candidate_pos[0]][candidate_pos[1]];
        match *new_val {
            Tile::Open => {
                return Some(candidate_pos);
            }
            Tile::Wall => {
                return None;
            }
            Tile::Void => {
                continue;
            }
        }
    }
}

fn get_candidate_pos(
    field: &Vec<Vec<Tile>>,
    position: [usize; 2],
    direction: &Direction,
) -> [usize; 2] {
    let ysize = field.len();
    let xsize = field[0].len();
    let mut new_pos = [position[0] + ysize, position[1] + xsize];
    new_pos[0] = (new_pos[0] as i32 - direction[1]) as usize;
    new_pos[1] = (new_pos[1] as i32 + direction[0]) as usize;
    // let mut new_pos = match direction {
    //     UP => [(position[0] + ysize) - 1, position[1]],
    //     DOWN => [position[0] + 1, position[1]],
    //     RIGHT => [position[0], position[1] + 1],
    //     LEFT => [position[0], (position[1] + xsize) - 1],
    //     _ => {panic!("Invalid direction!");}
    // };
    new_pos = [new_pos[0] % ysize, new_pos[1] % xsize];
    new_pos
}

fn get_starting_pos(field: &Vec<Vec<Tile>>) -> [usize; 2] {
    for (i, tile) in field[0].iter().enumerate() {
        if matches!(*tile, Tile::Open) {
            return [0, i];
        }
    }
    panic!("No valid starting pos found!");
}

fn main() {
    let input = get_input();
    let (field, moves) = parse_input(&input);
    let starting_pos = get_starting_pos(&field);
    dbg!(&starting_pos);
    let mut location = Location {
        position: starting_pos,
        direction: RIGHT,
    };
    println!("{}", moves.len());
    for (i, move_to_execute) in moves.iter().enumerate() {
        println!("{}", i);
        println!("{:?}", location);
        println!("{:?}", move_to_execute);
        location = make_move(&field, &location, &move_to_execute);
    }
    let Location {
        direction: final_direction,
        position: [yval, xval],
    } = location;
    let row = yval + 1;
    let col = xval + 1;
    let dir_val = match final_direction {
        RIGHT => 0,
        DOWN => 1,
        LEFT => 2,
        UP => 3,
        _ => {panic!("Invalid direction!");}
    };
    let res = (1000 * row) + (4 * col) + dir_val;
    println!("Res: {}", res);
}
