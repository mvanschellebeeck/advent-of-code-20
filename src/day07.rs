use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;

type Graph = HashMap<String, Vec<(i64, String)>>;

pub fn solve(part: u8) -> Option<i64> {
    match part {
        1 => part_1(),
        2 => part_2(),
        _ => None,
    }
}

fn input_to_graph() -> Graph {
    let input: Vec<String> = fs::read_to_string("input/day07.txt")
        .unwrap()
        .lines()
        .map(|s| s.to_string())
        .collect();

    let regex = Regex::new(r"(\d+) (.*) bags*").unwrap();
    let mut map: HashMap<String, Vec<(i64, String)>> = HashMap::new();

    for line in input {
        let splitter: Vec<&str> = line.split(" contain ").collect();
        let outer_bag = splitter.get(0).unwrap().replace(" bags", "");
        let inner_bags: Vec<(i64, String)> = splitter
            .get(1)
            .unwrap()
            .split(",")
            .filter(|s| !s.contains("no other bags"))
            .map(|s| {
                let captures = regex.captures(s).unwrap();
                let count = captures[1].parse::<i64>().unwrap();
                let colour = captures[2].parse::<String>().unwrap();
                (count, colour)
            })
            .collect();
        map.insert(outer_bag.to_string(), inner_bags);
    }
    map
}

fn dfs(graph: &Graph, src: &String, dst: &String, seen: &HashSet<String>) -> bool {
    for (_, colour) in &graph[src] {
        // check colour==dst here so first gold bag doesn't return true
        if colour == dst || (!seen.contains(colour) && dfs(graph, colour, dst, seen)) {
            return true;
        }
    }
    return false;
}

fn count_bags(graph: &Graph, src: &String) -> i64 {
    let mut total = 0;
    for (count, colour) in graph.get(src).unwrap() {
        total += count;
        total += count * count_bags(graph, colour);
    }
    total
}

fn part_1() -> Option<i64> {
    let graph = input_to_graph();
    let mut count = 0i64;

    // check if there is a path to a gold bag for each node
    for node in graph.keys() {
        if dfs(&graph, node, &"shiny gold".to_string(), &HashSet::new()) {
            count += 1;
        }
    }
    Some(count)
}

fn part_2() -> Option<i64> {
    let graph = input_to_graph();
    Some(count_bags(&graph, &"shiny gold".to_string()))
}
