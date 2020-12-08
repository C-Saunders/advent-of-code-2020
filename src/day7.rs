use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct InnerBag {
    color: String,
    number: usize,
}

#[aoc_generator(day7)]
pub fn get_values(
    input: &str,
) -> (
    HashMap<String, HashSet<String>>,
    HashMap<String, HashSet<InnerBag>>,
) {
    lazy_static! {
        static ref SPLIT_EXPR: Regex =
            Regex::new(r"^(?P<color>[a-z ]+) bags contain (.+)\.").unwrap();
        static ref INNER_BAGS_EXPR: Regex =
            Regex::new(r"(?:, )?(?P<number>\d) (?P<color>[a-z ]+) bags?").unwrap();
    }

    input.lines().fold(
        (HashMap::new(), HashMap::new()),
        |(mut contained_by_map, mut contains_map), line| {
            let parts = SPLIT_EXPR.captures(line).unwrap();
            let outer_bag = parts.get(1).unwrap().as_str();
            let inner_bags = parts.get(2).unwrap().as_str();

            for caps in INNER_BAGS_EXPR.captures_iter(inner_bags) {
                let current_contained_by = contained_by_map
                    .entry(caps["color"].to_string())
                    .or_insert(HashSet::new());
                current_contained_by.insert(outer_bag.to_string());

                let current_contains = contains_map
                    .entry(outer_bag.to_string())
                    .or_insert(HashSet::new());
                current_contains.insert(InnerBag {
                    color: caps["color"].to_string(),
                    number: caps["number"].parse().unwrap(),
                });
            }

            (contained_by_map, contains_map)
        },
    )
}

#[aoc(day7, part1)]
pub fn part1(
    (inputs, _): &(
        HashMap<String, HashSet<String>>,
        HashMap<String, HashSet<InnerBag>>,
    ),
) -> usize {
    let mut unexplored_containers = inputs
        .get(&"shiny gold".to_string())
        .unwrap()
        .into_iter()
        .collect::<Vec<&String>>();
    let mut all_containers: HashSet<String> = HashSet::new();

    while unexplored_containers.len() > 0 {
        let current_container = unexplored_containers.pop().unwrap();
        all_containers.insert(current_container.to_string());
        match inputs.get(current_container) {
            Some(v) => v.iter().for_each(|val| unexplored_containers.push(val)),
            None => {}
        }
    }

    all_containers.len()
}

#[aoc(day7, part2)]
pub fn part2(
    (_, inputs): &(
        HashMap<String, HashSet<String>>,
        HashMap<String, HashSet<InnerBag>>,
    ),
) -> usize {
    get_bag_total(&inputs, "shiny gold")
}

fn get_bag_total(map: &HashMap<String, HashSet<InnerBag>>, color: &str) -> usize {
    let entry = map.get(color);
    if entry.is_none() {
        return 0;
    }

    entry.unwrap().iter().fold(0, |acc, i| {
        acc + i.number + i.number * get_bag_total(map, &i.color)
    })
}
