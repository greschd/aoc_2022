use std::collections::HashSet;

fn get_input() -> String {
    let args: Vec<String> = std::env::args().collect();
    let path = args.get(1).expect("No file given!");
    String::from(std::fs::read_to_string(path).expect("Could not read file!"))
}

#[derive(Debug, Clone)]
struct Blizzard {
    position: [i32; 2],
    direction: [i32; 2],
}

fn parse_input(input: &str) -> (Vec<Vec<bool>>, Vec<Blizzard>) {
    let mut field: Vec<Vec<bool>> = vec![];
    let mut blizzards: Vec<Blizzard> = vec![];

    for (i, line) in input.lines().enumerate() {
        if line.len() == 0 {
            continue;
        }
        let mut field_row: Vec<bool> = vec![];
        for (j, char) in line.chars().enumerate() {
            if matches!(char, '#') {
                field_row.push(true);
            } else {
                field_row.push(false);
                let position = [i as i32, j as i32];
                match char {
                    '>' => {
                        blizzards.push(Blizzard {
                            position,
                            direction: [0, 1],
                        });
                    }
                    '<' => {
                        blizzards.push(Blizzard {
                            position,
                            direction: [0, -1],
                        });
                    }
                    '^' => {
                        blizzards.push(Blizzard {
                            position,
                            direction: [-1, 0],
                        });
                    }
                    'v' => {
                        blizzards.push(Blizzard {
                            position,
                            direction: [1, 0],
                        });
                    }
                    '.' => {}
                    _ => {
                        panic!("Invalid char!");
                    }
                }
            }
        }
        field.push(field_row);
    }

    (field, blizzards)
}

fn get_open_positions(field: &Vec<Vec<bool>>, blizzards: &Vec<Blizzard>) -> Vec<Vec<Vec<bool>>> {
    let mut res_fields: Vec<Vec<Vec<bool>>> = vec![];
    let size_x = field.len() - 2;
    let size_y = field[0].len() - 2;
    let mod_x = field.len() as i32;
    let mod_y = field[0].len() as i32;
    let num_steps = num::integer::lcm(size_x, size_y);
    let mut blizzards = blizzards.clone();
    for _ in 0..num_steps {
        let blizzards_prev = blizzards.clone();
        let mut curr_field = field.clone();
        for blz in &blizzards_prev {
            curr_field[blz.position[0] as usize][blz.position[1] as usize] = true;
        }
        res_fields.push(curr_field);

        // compute new blizzard positions
        blizzards.clear();
        for blz in blizzards_prev {
            let mut pos = blz.position;
            pos[0] += blz.direction[0];
            pos[1] += blz.direction[1];
            if field[pos[0] as usize][pos[1] as usize] {
                pos[0] += mod_x + (2 * blz.direction[0]);
                pos[1] += mod_y + (2 * blz.direction[1]);
                pos[0] %= mod_x;
                pos[1] %= mod_y;
            }
            blizzards.push(Blizzard {
                position: pos,
                direction: blz.direction,
            });
        }
    }
    res_fields
}

fn get_start_pos(field: &Vec<Vec<bool>>) -> [i32; 2] {
    for (i, val) in field[0].iter().enumerate() {
        if !val {
            return [0, i as i32];
        }
    }
    panic!("Starting position not found!");
}

fn get_target_pos(field: &Vec<Vec<bool>>) -> [i32; 2] {
    let x_idx = field.len() - 1;
    for (i, val) in field[x_idx].iter().enumerate() {
        if !val {
            return [x_idx as i32, i as i32];
        }
    }
    panic!("Target position not found!");
}

fn get_num_steps(
    start_pos: &[i32; 2],
    target_pos: &[i32; 2],
    open_positions_by_step: &Vec<Vec<Vec<bool>>>,
    initial_step_idx: usize,
) -> usize {
    let blizzard_repeat_steps = open_positions_by_step.len();
    let mut possible_locations = HashSet::<[i32; 2]>::new();
    possible_locations.insert(start_pos.clone());
    let mut step_idx = initial_step_idx;
    loop {
        step_idx += 1;
        let possible_locations_last_step = possible_locations.clone();
        possible_locations.clear();
        let occupied_fields = &open_positions_by_step[step_idx % blizzard_repeat_steps];
        for pos in possible_locations_last_step {
            for candidate in [
                [pos[0], pos[1]],
                [pos[0] + 1, pos[1]],
                [pos[0] - 1, pos[1]],
                [pos[0], pos[1] + 1],
                [pos[0], pos[1] - 1],
            ] {
                // we can only walk off the field in x-direction
                if candidate[0] < 0 || candidate[0] >= occupied_fields.len() as i32 {
                    continue;
                }
                if !occupied_fields[candidate[0] as usize][candidate[1] as usize] {
                    if candidate == *target_pos {
                        return step_idx;
                    }
                    possible_locations.insert(candidate);
                }
            }
        }
    }
}

fn main() {
    let input = get_input();
    let (field, blizzards_initial) = parse_input(&input);
    let open_positions_by_step = get_open_positions(&field, &blizzards_initial);

    let starting_pos = get_start_pos(&field);
    let target_pos = get_target_pos(&field);
    let res_p1 = get_num_steps(&starting_pos, &target_pos, &open_positions_by_step, 0);
    let back_to_start = get_num_steps(&target_pos, &starting_pos, &open_positions_by_step, res_p1);
    let res_p2 = get_num_steps(&starting_pos, &target_pos, &open_positions_by_step, back_to_start);

    println!("P1: {}", res_p1);
    println!("p2: {}", res_p2);
}
