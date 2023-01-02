use std::collections::HashMap;

fn get_input() -> String {
    let args: Vec<String> = std::env::args().collect();
    let path = args.get(1).expect("No file given!");
    String::from(std::fs::read_to_string(path).expect("Could not read file!"))
}

#[derive(Debug, PartialEq, Clone)]
enum Tile {
    Void,
    Open,
    Wall,
}

fn transposed<T: num::Integer + Copy + std::ops::AddAssign, const N: usize, const M: usize>(
    a: &[[T; N]; M],
) -> [[T; M]; N] {
    let mut res: [[T; M]; N] = [[T::zero(); M]; N];
    for i in 0..M {
        for j in 0..N {
            res[j][i] = a[i][j];
        }
    }
    res
}

fn gemm<
    T: num::Integer + Copy + std::ops::AddAssign,
    const N: usize,
    const M: usize,
    const L: usize,
>(
    a: &[[T; N]; M],
    b: &[[T; L]; N],
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
    a: &[[T; M]; N],
    b: &[T; M],
) -> [T; N] {
    let mut res: [T; N] = [T::zero(); N];
    for i in 0..N {
        for j in 0..M {
            res[i] += a[i][j] * b[j];
        }
    }
    res
}

type TurnMatrix = [[i32; 2]; 2];

const CLOCKWISE: TurnMatrix = [[0, 1], [-1, 0]];
const COUNTERCLOCKWISE: TurnMatrix = [[0, -1], [1, 0]];

#[derive(Debug)]
enum Move {
    Advance(i32),
    Turn(TurnMatrix),
}

fn parse_input(input: &str) -> (Vec<Vec<Tile>>, Vec<Move>) {
    let (field, moves) = input.split_once("\n\n").unwrap();
    let width = field.lines().fold(0, |x, y| std::cmp::max(x, y.len()));

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
                res_moves.push(Move::Turn(CLOCKWISE));
            }
            'L' => {
                if move_size_str.len() > 0 {
                    res_moves.push(Move::Advance(move_size_str.parse::<i32>().unwrap()));
                }
                move_size_str.clear();
                res_moves.push(Move::Turn(COUNTERCLOCKWISE));
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

type Direction = [i32; 2];
const LEFT: Direction = [-1, 0];
const RIGHT: Direction = [1, 0];
const UP: Direction = [0, 1];
const DOWN: Direction = [0, -1];

fn dir_to_index_offset(direction: &[i32; 2]) -> [i32; 2] {
    [-direction[1], direction[0]]
}

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
    if let Move::Turn(turn_matrix) = move_to_execute {
        let new_direction = gemv(turn_matrix, &initial_location.direction);
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
    let index_offset = dir_to_index_offset(direction);
    let mut new_pos = [position[0] + ysize, position[1] + xsize];
    new_pos[0] = (new_pos[0] as i32 + index_offset[0]) as usize;
    new_pos[1] = (new_pos[1] as i32 + index_offset[1]) as usize;
    new_pos = [new_pos[0] % ysize, new_pos[1] % xsize];
    new_pos
}

fn make_move_p2(
    field: &Vec<Vec<Tile>>,
    initial_location: &Location,
    move_to_execute: &Move,
    fold_info: &FoldInfo,
) -> Location {
    if let Move::Turn(turn_matrix) = move_to_execute {
        let new_direction = gemv(turn_matrix, &initial_location.direction);
        return Location {
            position: initial_location.position.clone(),
            direction: new_direction,
        };
    }
    if let Move::Advance(num_steps) = move_to_execute {
        let mut location = initial_location.clone();
        for _ in 0..*num_steps {
            let next_location = get_advanced_location_p2(&location, fold_info);
            let next_pos = next_location.position;
            let next_tile = field[next_pos[0]][next_pos[1]].clone();
            match next_tile {
                Tile::Open => {
                    location = next_location;
                }
                Tile::Wall => {
                    break;
                }
                Tile::Void => {
                    panic!("Logic error!");
                }
            }
        }
        return location;
    }
    panic!("Logic error!");
}

fn get_advanced_location_p2(location: &Location, fold_info: &FoldInfo) -> Location {
    let current_tile_idx = get_tile_index(&location.position, fold_info.side_length);
    {
        let idx_offset = dir_to_index_offset(&location.direction);
        let advanced_pos_flat = [
            location.position[0] as i32 + idx_offset[0],
            location.position[1] as i32 + idx_offset[1],
        ];
        if advanced_pos_flat[0] >= 0 && advanced_pos_flat[1] >= 0 {
            let advanced_pos_flat = [advanced_pos_flat[0] as usize, advanced_pos_flat[1] as usize];
            if current_tile_idx == get_tile_index(&advanced_pos_flat, fold_info.side_length)
            {
                return Location {
                    position: advanced_pos_flat,
                    direction: location.direction.clone(),
                };
            }
        }
    }

    let current_tile = fold_info.map_tiles[current_tile_idx[0]][current_tile_idx[1]].as_ref().unwrap();

    let new_normal = gemv(&transposed(&[current_tile.map_east, current_tile.map_north]), &location.direction);
    let new_tile_idx = fold_info.norm_to_index.get(&new_normal).unwrap();
    let new_tile = fold_info.map_tiles[new_tile_idx[0]][new_tile_idx[1]].as_ref().unwrap();

    let pos_relative = [location.position[0] % fold_info.side_length, location.position[1] % fold_info.side_length];
    let pos_relative = [pos_relative[0] as f64, pos_relative[1] as f64];

    // convert to 3D coordinate
    let xcoord: f64 = (pos_relative[1] - ((fold_info.side_length - 1) as f64 / 2.)) + (location.direction[0] as f64 / 2.);
    let ycoord: f64 = (((fold_info.side_length - 1) as f64 / 2.) - pos_relative[0]) + (location.direction[1] as f64 / 2.);
    let zcoord: f64 = fold_info.side_length as f64 / 2.;
    let mut pos_3d: [f64;3] = [0.; 3];
    for i in 0..3 {
        pos_3d[i] += xcoord * current_tile.map_east[i] as f64;
        pos_3d[i] += ycoord * current_tile.map_north[i] as f64;
        pos_3d[i] += zcoord * current_tile.normal[i] as f64;
    }
    // and back to 2D coordinate
    let mut coord_new: [f64; 2] = [0.; 2];
    for i in 0..3 {
        if new_tile.map_east[i] != 0 {
            coord_new[0] = pos_3d[i] / new_tile.map_east[i] as f64;
        }
        if new_tile.map_north[i] != 0 {
            coord_new[1] = pos_3d[i] / new_tile.map_north[i] as f64;
        }
    }
    // coordinate to index
    let mut idx_new: [f64; 2] = [-coord_new[1], coord_new[0]];
    idx_new[0] += (fold_info.side_length - 1) as f64 / 2.;
    idx_new[1] += (fold_info.side_length - 1) as f64 / 2.;
    let mut idx_new = [idx_new[0] as usize, idx_new[1] as usize];
    idx_new[0] += fold_info.side_length * new_tile_idx[0];
    idx_new[1] += fold_info.side_length * new_tile_idx[1];

    let new_direction_3d = [-current_tile.normal[0], -current_tile.normal[1], -current_tile.normal[2]];
    for direction in [UP, DOWN, RIGHT, LEFT] {
        if gemv(&transposed(&[new_tile.map_east, new_tile.map_north]), &direction) == new_direction_3d {
            let new_loc = Location{
                position: idx_new,
                direction: direction
            };
            return new_loc;
        }
    }
    panic!("NIMPL");

}

fn get_tile_index(position: &[usize; 2], side_length: usize) -> [usize; 2] {
    [position[0] / side_length, position[1] / side_length]
}

fn get_starting_pos(field: &Vec<Vec<Tile>>) -> [usize; 2] {
    for (i, tile) in field[0].iter().enumerate() {
        if matches!(*tile, Tile::Open) {
            return [0, i];
        }
    }
    panic!("No valid starting pos found!");
}

fn p1(field: &Vec<Vec<Tile>>, moves: &Vec<Move>) {
    let starting_pos = get_starting_pos(&field);
    let mut location = Location {
        position: starting_pos,
        direction: RIGHT,
    };
    for move_to_execute in moves {
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
        _ => {
            panic!("Invalid direction!");
        }
    };
    let res = (1000 * row) + (4 * col) + dir_val;
    println!("Res: {}", res);
}

#[derive(Debug, Clone)]
struct MapTile {
    map_east: [i32; 3],
    map_north: [i32; 3],
    normal: [i32; 3],
}

fn rotate_along(map_tile: &MapTile, direction: &[i32; 2]) -> MapTile {
    let direction_3d = gemv(
        &transposed(&[map_tile.map_east, map_tile.map_north]),
        direction,
    );
    let other_dir_3d = gemv(
        &transposed(&[map_tile.map_east, map_tile.map_north]),
        &gemv(&COUNTERCLOCKWISE, direction),
    );

    let mut new_x = [
        -map_tile.normal[0],
        -map_tile.normal[1],
        -map_tile.normal[2],
    ];
    let mut new_y = other_dir_3d;
    let new_normal = direction_3d;
    // rotate until it matches RIGHT
    let mut dir_clone = direction.clone();
    loop {
        if dir_clone == RIGHT {
            break;
        }
        dir_clone = gemv(&COUNTERCLOCKWISE, &dir_clone);
        [new_x, new_y] = transposed(&gemm(&transposed(&[new_x, new_y]), &COUNTERCLOCKWISE));
    }
    MapTile {
        map_east: new_x,
        map_north: new_y,
        normal: new_normal,
    }
}

struct FoldInfo {
    side_length: usize,
    map_tiles: Vec<Vec<Option<MapTile>>>,
    norm_to_index: HashMap<[i32; 3], [usize; 2]>,
}

fn get_fold_info(field: &Vec<Vec<Tile>>) -> FoldInfo {
    let mut tile_count = 0;
    for row in field {
        for tile in row {
            if *tile != Tile::Void {
                tile_count += 1;
            }
        }
    }
    let side_area = tile_count / 6;
    let side_length = (side_area as f64 + 0.5).sqrt() as usize;
    let mut occupied_tiles: Vec<Vec<bool>> = vec![];
    for row in field.iter().step_by(side_length) {
        let mut row_vec: Vec<bool> = vec![];
        for tile in row.iter().step_by(side_length) {
            if matches!(*tile, Tile::Void) {
                row_vec.push(false);
            } else {
                row_vec.push(true);
            }
        }
        occupied_tiles.push(row_vec);
    }
    let tiles_width = occupied_tiles[0].len();
    let tiles_height = occupied_tiles.len();
    let mut map_tiles: Vec<Vec<Option<MapTile>>> = vec![];
    for _ in 0..tiles_height {
        let mut maptile_row: Vec<Option<MapTile>> = vec![];
        for _ in 0..tiles_width {
            maptile_row.push(None);
        }
        map_tiles.push(maptile_row);
    }
    let mut newly_visited: Vec<[usize; 2]> = vec![];
    for (i, occ) in occupied_tiles[0].iter().enumerate() {
        if *occ {
            map_tiles[0][i] = Some(MapTile {
                map_east: [1, 0, 0],
                map_north: [0, 1, 0],
                normal: [0, 0, 1],
            });
            newly_visited.push([0, i]);
            break;
        }
    }

    loop {
        if newly_visited.len() == 0 {
            break;
        }
        let to_check = newly_visited.clone();
        newly_visited.clear();
        for index in to_check {
            for direction in [RIGHT, LEFT, UP, DOWN] {
                let index_offset = dir_to_index_offset(&direction);
                let new_index = [
                    index[0] as i32 + index_offset[0],
                    index[1] as i32 + index_offset[1],
                ];
                if new_index[0] < 0 || new_index[0] >= tiles_height as i32 {
                    continue;
                }
                if new_index[1] < 0 || new_index[1] >= tiles_width as i32 {
                    continue;
                }
                let [i0, i1] = [new_index[0] as usize, new_index[1] as usize];
                if occupied_tiles[i0][i1] && map_tiles[i0][i1].is_none() {
                    newly_visited.push([i0, i1]);
                    map_tiles[i0][i1] = Some(rotate_along(
                        map_tiles[index[0]][index[1]].as_ref().unwrap(),
                        &direction,
                    ));
                }
            }
        }
    }

    let mut norm_to_index = HashMap::<[i32; 3], [usize;2]>::new();
    for (i, row) in map_tiles.iter().enumerate() {
        for (j, map_tile) in row.iter().enumerate() {
            if let Some(value) = map_tile.clone() {
                norm_to_index.insert(value.normal, [i,j]);
            }
        }
    }

    FoldInfo {
        side_length,
        map_tiles,
        norm_to_index,
    }
}

fn p2(field: &Vec<Vec<Tile>>, moves: &Vec<Move>) {
    let fold_info = get_fold_info(field);

    let starting_pos = get_starting_pos(&field);
    let mut location = Location {
        position: starting_pos,
        direction: RIGHT,
    };
    for move_to_execute in moves.iter() {
        location = make_move_p2(&field, &location, &move_to_execute, &fold_info);
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
        _ => {
            panic!("Invalid direction!");
        }
    };
    let res = (1000 * row) + (4 * col) + dir_val;
    println!("Res: {}", res);
}

fn main() {
    let input = get_input();
    let (field, moves) = parse_input(&input);
    p1(&field, &moves);
    p2(&field, &moves);
}
