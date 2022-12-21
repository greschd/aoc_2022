use std::cmp::Ordering;
use std::collections::HashSet;
use std::ops::{Add, Mul, Sub};

fn get_input() -> String {
    let args: Vec<String> = std::env::args().collect();
    let path = args.get(1).expect("No file given!");
    String::from(std::fs::read_to_string(path).expect("Could not read file!"))
}

fn parse_input(input: &str) -> Vec<Blueprint> {
    let mut blueprints: Vec<Blueprint> = vec![];
    for line in input.lines() {
        if line.len() > 0 {
            let (_, tail) = line.split_once(": Each ore robot costs ").unwrap();
            let (ore_per_ore_robot, tail) =
                tail.split_once(" ore. Each clay robot costs ").unwrap();
            let ore_per_ore_robot = ore_per_ore_robot.parse::<i32>().unwrap();
            let (ore_per_clay_robot, tail) =
                tail.split_once(" ore. Each obsidian robot costs ").unwrap();
            let ore_per_clay_robot = ore_per_clay_robot.parse::<i32>().unwrap();
            let (ore_per_obsidian_robot, tail) = tail.split_once(" ore and ").unwrap();
            let ore_per_obsidian_robot = ore_per_obsidian_robot.parse::<i32>().unwrap();
            let (clay_per_obsidian_robot, tail) =
                tail.split_once(" clay. Each geode robot costs ").unwrap();
            let clay_per_obsidian_robot = clay_per_obsidian_robot.parse::<i32>().unwrap();
            let (ore_per_geode_robot, tail) = tail.split_once(" ore and ").unwrap();
            let ore_per_geode_robot = ore_per_geode_robot.parse::<i32>().unwrap();
            let (obsidian_per_geode_robot, _) = tail.split_once(" obsidian.").unwrap();
            let obsidian_per_geode_robot = obsidian_per_geode_robot.parse::<i32>().unwrap();
            blueprints.push(Blueprint {
                ore_robot_cost: Materials {
                    ore: ore_per_ore_robot,
                    clay: 0,
                    obsidian: 0,
                    geode: 0,
                },
                clay_robot_cost: Materials {
                    ore: ore_per_clay_robot,
                    clay: 0,
                    obsidian: 0,
                    geode: 0,
                },
                obsidian_robot_cost: Materials {
                    ore: ore_per_obsidian_robot,
                    clay: clay_per_obsidian_robot,
                    obsidian: 0,
                    geode: 0,
                },
                geode_robot_cost: Materials {
                    ore: ore_per_geode_robot,
                    clay: 0,
                    obsidian: obsidian_per_geode_robot,
                    geode: 0,
                },
            })
        }
    }
    blueprints
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Materials {
    ore: i32,
    clay: i32,
    obsidian: i32,
    geode: i32,
}

impl PartialOrd for Materials {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let comparisons = HashSet::<Ordering>::from([
            self.ore.cmp(&other.ore),
            self.clay.cmp(&other.clay),
            self.obsidian.cmp(&other.obsidian),
            self.geode.cmp(&other.geode),
        ]);
        if comparisons.len() == 3 {
            return None;
        }
        if comparisons.len() == 2 {
            if comparisons.contains(&Ordering::Equal) {
                if comparisons.contains(&Ordering::Less) {
                    return Some(Ordering::Less);
                }
                if comparisons.contains(&Ordering::Greater) {
                    return Some(Ordering::Greater);
                }
            }
            return None; // Case Less + Equal
        }
        comparisons.iter().next().copied()
    }
}

impl Mul<i32> for Materials {
    type Output = Self;
    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            ore: self.ore * rhs,
            clay: self.clay * rhs,
            obsidian: self.obsidian * rhs,
            geode: self.geode * rhs,
        }
    }
}

impl Sub for Materials {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            ore: self.ore - other.ore,
            clay: self.clay - other.clay,
            obsidian: self.obsidian - other.obsidian,
            geode: self.geode - other.geode,
        }
    }
}

impl Add for Materials {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            ore: self.ore + other.ore,
            clay: self.clay + other.clay,
            obsidian: self.obsidian + other.obsidian,
            geode: self.geode + other.geode,
        }
    }
}

#[derive(Debug)]
struct Blueprint {
    ore_robot_cost: Materials,
    clay_robot_cost: Materials,
    obsidian_robot_cost: Materials,
    geode_robot_cost: Materials,
}

fn count_from_cost(cost: i32, mat: i32) -> i32 {
    if cost == 0 {
        return std::i32::MAX;
    }
    if mat == 0 {
        return 0;
    }
    return mat / cost;
}

fn get_optimistic_robot_count(materials: &Materials, cost: &Materials) -> i32 {
    [
        count_from_cost(cost.ore, materials.ore),
        count_from_cost(cost.clay, materials.clay),
        count_from_cost(cost.obsidian, materials.obsidian),
    ]
    .iter()
    .fold(std::i32::MAX, |x, y| std::cmp::min(x, *y))
}

fn get_optimistic_estimate(
    blueprint: &Blueprint,
    time: i32,
    materials: &Materials,
    robots_initial: &Materials,
    robots: &Materials,
) -> i32 {
    // let mut res: Vec<i32> = vec![];

    if time == 0 {
        return materials.geode;
    }
    let materials_next_step = materials.clone() + robots.clone();
    // dbg!(&materials_next_step);
    let robots_next_step = robots_initial.clone()
        + Materials {
            ore: get_optimistic_robot_count(&materials, &blueprint.ore_robot_cost),
            clay: get_optimistic_robot_count(&materials, &blueprint.clay_robot_cost),
            obsidian: get_optimistic_robot_count(&materials, &blueprint.obsidian_robot_cost),
            geode: get_optimistic_robot_count(&materials, &blueprint.geode_robot_cost),
        };

    return get_optimistic_estimate(
        &blueprint,
        time - 1,
        &materials_next_step,
        &robots_initial,
        &robots_next_step,
    );
    // res.push(materials_next_step.geode);
    // res
}

fn time_to_buy_single_resource(mat: i32, rbt: i32, cost: i32) -> Option<i32> {
    if cost == 0 {
        return Some(0);
    } else if rbt == 0 {
        return None;
    } else if mat >= cost {
        return Some(0);
    } else {
        let to_gain = cost - mat;
        return Some((to_gain + (rbt - 1)) / rbt); // rounding-up division
    }
}

fn time_to_buy(materials: &Materials, robots: &Materials, cost: &Materials) -> Option<i32> {
    let mut time = 0;
    let times_per_resources = [
        time_to_buy_single_resource(materials.ore, robots.ore, cost.ore),
        time_to_buy_single_resource(materials.clay, robots.clay, cost.clay),
        time_to_buy_single_resource(materials.obsidian, robots.obsidian, cost.obsidian),
    ];
    for t in times_per_resources {
        match t {
            Some(val) => {
                time = std::cmp::max(val, time);
            }
            None => {
                return None;
            }
        };
    }
    return Some(time);
}

fn buy_next_robot(
    blueprint: &Blueprint,
    materials: &Materials,
    robots: &Materials,
    next_robot: &Materials,
    cost: &Materials,
    time: i32,
    score_to_beat: i32,
) -> i32 {
    let time_needed = time_to_buy(&materials, &robots, &cost);
    let time_needed = match time_needed {
        None => time,
        Some(value) => value,
    };
    if (time - time_needed) <= 1 {
        let final_materials = materials.clone() + robots.clone() * time;
        return final_materials.geode;
    }
    // Skip to when the robot is bought
    let time = time - (time_needed + 1);
    let materials_new = (materials.clone() + robots.clone() * (time_needed + 1)) - cost.clone();
    let robots_new = robots.clone() + next_robot.clone();
    return get_max_geodes(&blueprint, &robots_new, &materials_new, time, score_to_beat);
}

const ORE_ROBOT: Materials = Materials {
    ore: 1,
    clay: 0,
    obsidian: 0,
    geode: 0,
};
const CLAY_ROBOT: Materials = Materials {
    ore: 0,
    clay: 1,
    obsidian: 0,
    geode: 0,
};
const OBSIDIAN_ROBOT: Materials = Materials {
    ore: 0,
    clay: 0,
    obsidian: 1,
    geode: 0,
};
const GEODE_ROBOT: Materials = Materials {
    ore: 0,
    clay: 0,
    obsidian: 0,
    geode: 1,
};

fn get_max_geodes(
    blueprint: &Blueprint,
    robots: &Materials,
    materials: &Materials,
    time: i32,
    score_to_beat: i32,
) -> i32 {
    let mut max_score = score_to_beat;
    let optimistic_estimate =
        get_optimistic_estimate(&blueprint, time, &materials, &robots, &robots);
    if optimistic_estimate <= max_score {
        return max_score;
    }

    let geode_step_score = buy_next_robot(
        &blueprint,
        &materials,
        &robots,
        &GEODE_ROBOT,
        &blueprint.geode_robot_cost,
        time,
        max_score,
    );
    max_score = std::cmp::max(max_score, geode_step_score);
    if optimistic_estimate <= max_score {
        return max_score;
    }
    let obsidian_step_score = buy_next_robot(
        &blueprint,
        &materials,
        &robots,
        &OBSIDIAN_ROBOT,
        &blueprint.obsidian_robot_cost,
        time,
        max_score,
    );
    max_score = std::cmp::max(max_score, obsidian_step_score);
    if optimistic_estimate <= max_score {
        return max_score;
    }
    let clay_step_score = buy_next_robot(
        &blueprint,
        &materials,
        &robots,
        &CLAY_ROBOT,
        &blueprint.clay_robot_cost,
        time,
        max_score,
    );
    max_score = std::cmp::max(max_score, clay_step_score);
    if optimistic_estimate <= max_score {
        return max_score;
    }
    let ore_step_score = buy_next_robot(
        &blueprint,
        &materials,
        &robots,
        &ORE_ROBOT,
        &blueprint.ore_robot_cost,
        time,
        max_score,
    );
    max_score = std::cmp::max(max_score, ore_step_score);
    return max_score;
}

fn main() {
    let input = get_input();
    let blueprints = parse_input(&input);

    let mut quality_level = 0;
    for (i, bp) in blueprints.iter().enumerate() {
        let score = get_max_geodes(
            &bp,
            &Materials {
                ore: 1,
                clay: 0,
                obsidian: 0,
                geode: 0,
            },
            &Materials {
                ore: 0,
                clay: 0,
                obsidian: 0,
                geode: 0,
            },
            24,
            0,
        );
        println!("Blueprint score: {:?}", score);
        quality_level += ((i + 1) as i32) * score;
    }
    println!("Total Quality Level: {:?}", quality_level);

    let mut res_p2 = 1;
    for (i, bp) in blueprints.iter().enumerate() {
        if i >= 3 {
            break;
        }
        let score = get_max_geodes(
            &bp,
            &Materials {
                ore: 1,
                clay: 0,
                obsidian: 0,
                geode: 0,
            },
            &Materials {
                ore: 0,
                clay: 0,
                obsidian: 0,
                geode: 0,
            },
            32,
            0,
        );
        println!("Blueprint score: {:?}", score);
        res_p2 *= score;
    }
    println!("P2: {:?}", res_p2);

}
