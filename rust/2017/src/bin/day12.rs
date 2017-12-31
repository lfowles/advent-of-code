struct Node {
    id: i32,
    connections: Vec<i32>,
}

use std::collections::{HashMap, HashSet};

fn read_graph(input: &str) -> HashMap<i32, Node> {
    let mut graph = HashMap::new();
    for line in input.split('\n') {
        let contents: Vec<&str> = line.split("<->").collect();
        let id = contents[0].trim().parse::<i32>().unwrap();
        let connections: Vec<i32> = contents[1].trim().split(',').map(|s| s.trim().parse::<i32>().unwrap() ).collect();
        graph.insert(id, Node{ id, connections });
    }

    graph
}

fn flood_fill(id: i32, graph: &HashMap<i32, Node>, mut visited: &mut HashSet<i32>) {
    if !visited.insert(id) { return; }

    for connection_id in &graph[&id].connections {
        flood_fill(*connection_id, &graph, &mut visited);
    }
}

fn part1(input: &str) -> i32 {
    let graph = read_graph(input);
    let mut connected = HashSet::new();

    flood_fill(0, &graph, &mut connected);

    connected.len() as i32
}

#[test]
fn test_part1() {
    let test_input = r#"
0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5
"#.trim();
    assert_eq!(part1(test_input), 6);
}

fn part2(input: &str) -> i32 {
    let graph = read_graph(input);
    let mut connected = HashSet::new();

    let mut group_count = 0;

    for id in graph.keys() {
        if !connected.contains(id) {
            group_count += 1;
            flood_fill(*id, &graph, &mut connected);
        }
    }

    group_count
}

#[test]
fn test_part2() {
    let test_input = r#"
0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5
"#.trim();
    assert_eq!(part2(test_input), 2);
}

#[macro_use]
extern crate adventofcode;

fn main() {
    let input = get_puzzle_input!(day12);

    let part1_answer = part1(input);
    println!("Part 1: {}", part1_answer);

    let part2_answer = part2(input);
    println!("Part 2: {}", part2_answer);
}