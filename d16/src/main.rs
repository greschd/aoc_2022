use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug)]
struct ValveSpec {
    // name: String,
    flow_rate: i32,
    tunnels: Vec<String>,
}

fn get_input() -> String {
    let args: Vec<String> = std::env::args().collect();
    let path = args.get(1).expect("No file given!");
    String::from(std::fs::read_to_string(path).expect("Could not read file!"))
}

fn parse_input(input: &str) -> HashMap<String, ValveSpec> {
    let mut res = HashMap::<String, ValveSpec>::new();
    for line in input.lines() {
        if line.len() > 0 {
            let (_, tail) = line.split_once("Valve ").unwrap();
            let (valve_name, tail) = tail.split_once(" has flow rate=").unwrap();
            let (flow_rate, tail) = tail.split_once(";").unwrap();
            let flow_rate = flow_rate.parse::<i32>().unwrap();
            let (_, tail) = match tail.split_once(" tunnels lead to valves ") {
                Some(value) => value,
                None => tail.split_once(" tunnel leads to valve ").unwrap(),
            };

            let mut targets: Vec<String> = vec![];
            for t in tail.split(", ") {
                targets.push(String::from(t));
            }
            res.insert(
                String::from(valve_name),
                ValveSpec {
                    // name: String::from(valve_name),
                    flow_rate: flow_rate,
                    tunnels: targets,
                },
            );
        }
    }
    res
}

fn get_times(valves: &HashMap<String, ValveSpec>, start: &str) -> HashMap<String, i32> {
    let mut res = HashMap::<String, i32>::new();
    res.insert(String::from(start), 0);
    let mut newly_encountered: Vec<String> = vec![String::from(start)];
    let mut iter = 0;
    loop {
        iter += 1;
        let prev = newly_encountered.clone();
        newly_encountered.clear();
        if prev.len() == 0 {
            break;
        }
        for valve_key in prev {
            let neighbors = &valves.get(&valve_key).unwrap().tunnels;
            for n in neighbors {
                if !res.contains_key(n) {
                    res.insert(n.clone(), iter);
                    newly_encountered.push(n.clone());
                }
            }
        }
    }

    res
}

fn get_important_valves(valves: &HashMap<String, ValveSpec>) -> HashMap<String, i32> {
    let mut res = HashMap::<String, i32>::new();
    for (key, value) in valves {
        if (key == "AA") || (value.flow_rate > 0) {
            res.insert(key.clone(), value.flow_rate);
        }
    }
    res
}

fn get_switch_times(
    valves: &HashMap<String, ValveSpec>,
    valuable_valves: &HashMap<String, i32>,
) -> HashMap<String, HashMap<String, i32>> {
    let mut res = HashMap::<String, HashMap<String, i32>>::new();
    for key in valuable_valves.keys() {
        res.insert(key.clone(), get_times(&valves, &key));
    }
    res
}

fn get_best_score(
    possible_valves: &HashMap<String, i32>,
    switch_times: &HashMap<String, HashMap<String, i32>>,
    current_position: &String,
    current_time: i32,
    score_to_beat: i32,
) -> i32 {
    let mut best_score = 0;
    let mut remaining_unopened_flowrate = 0;
    for (_, value) in possible_valves {
        remaining_unopened_flowrate += value;
    }
    for (valve_name, flow_rate) in possible_valves {
        let time = current_time - (switch_times[current_position][valve_name] + 1);
        if (time * remaining_unopened_flowrate) <= score_to_beat {
            continue;
        }
        let step_score = time * flow_rate;
        let mut reduced_possible_valves = possible_valves.clone();
        reduced_possible_valves.remove(valve_name);
        let sub_score = get_best_score(
            &reduced_possible_valves,
            switch_times,
            valve_name,
            time,
            best_score - step_score,
        );
        best_score = std::cmp::max(best_score, sub_score + step_score);
    }
    best_score
}

fn main() {
    let input = get_input();
    let valves = parse_input(&input);
    let mut important_valves = get_important_valves(&valves);
    let switch_times = get_switch_times(&valves, &important_valves);

    important_valves.remove("AA");
    let best_score = get_best_score(&important_valves, &switch_times, &String::from("AA"), 30, 0);

    dbg!(best_score);

    let mut best_score_p2 = best_score;
    let mut last_len = 0;
    for my_targets in important_valves.iter().powerset() {
        if my_targets.len() > last_len {
            dbg!(my_targets.len());
            last_len = my_targets.len();
        }
        if my_targets.len() < 4 {
            continue;
        }
        if my_targets.len() > (important_valves.len() / 2) {
            break;
        }

        let mut my_targets_map = HashMap::<String, i32>::new();
        let mut elephant_targets = important_valves.clone();
        for (target, val) in my_targets {
            my_targets_map.insert(target.clone(), *val);
            elephant_targets.remove(target);
        }

        let score_mine = get_best_score(&my_targets_map, &switch_times, &String::from("AA"), 26, 0);
        let score_elephant = get_best_score(
            &elephant_targets,
            &switch_times,
            &String::from("AA"),
            26,
            best_score_p2 - score_mine,
        );

        let last_best = best_score_p2;
        best_score_p2 = std::cmp::max(score_mine + score_elephant, best_score_p2);
        if best_score_p2 > last_best {
            dbg!(best_score_p2);
        }
    }
    dbg!(best_score_p2);
}
