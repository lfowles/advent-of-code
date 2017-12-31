use std::collections::HashMap;

fn part1(input: &str) -> i32 {
    let mut path: HashMap<&str, i32> = HashMap::new();

    let directions = ["nw", "n", "ne", "se", "s", "sw"];

    for dir in directions.iter() {
        path.insert(dir, 0);
    }

    for dir in input.split(',') {
        *path.get_mut(dir).unwrap() += 1;
    }


    // Every other direction can be merged to create the direction inbetween
    let mut modified = true;


    while modified {
        modified = false;
        for i in 0..directions.len() {
            let dir1 = directions[i];
            let dir2 = directions[(i + 2) % directions.len()];
            let new_dir = directions[(i + 1) % directions.len()];

            let num_merged = std::cmp::min(path[dir1], path[dir2]);

            modified |= num_merged > 0;

            *path.get_mut(dir1).unwrap() -= num_merged;
            *path.get_mut(dir2).unwrap() -= num_merged;
            *path.get_mut(new_dir).unwrap() += num_merged;
        }
    }

    // opposites cancel
    for &(dir1, dir2) in [("n", "s"), ("ne", "sw"), ("nw", "se")].iter() {
        let min = std::cmp::min(path[dir1], path[dir2]);
        *path.get_mut(dir1).unwrap() -= min;
        *path.get_mut(dir2).unwrap() -= min;
    }

    path.values().sum()
}

#[test]
fn test_part1() {
    assert_eq!(part1("ne,ne,ne"), 3);
    assert_eq!(part1("ne,ne,sw,sw"), 0);
    assert_eq!(part1("ne,ne,s,s"), 2);
    assert_eq!(part1("se,sw,se,sw,sw"), 3);
}

fn get_opposite(dir: &str) -> &str {
    let directions = ["nw", "n", "ne", "se", "s", "sw"];
    let pos = directions.iter().position(|s| *s == dir).unwrap();
    directions[(pos + 3) % directions.len()]
}

fn get_merges(dir: &str) -> ((&str, &str), (&str, &str)) {
    let directions = ["nw", "n", "ne", "se", "s", "sw"];
    let pos = directions.iter().position(|s| *s == dir).unwrap();
    (
        (directions[(pos + 2) % directions.len()], directions[(pos + 1) % directions.len()]),
        (directions[(pos + 4) % directions.len()], directions[(pos + 5) % directions.len()])
    )
}

fn part2(input: &str) -> i32 {
    let mut path: HashMap<&str, i32> = HashMap::new();

    let mut max_distance = 0;

    let directions = ["nw", "n", "ne", "se", "s", "sw"];

    for dir in directions.iter() {
        path.insert(dir, 0);
    }

    for dir in input.split(',') {
        // Cancel opposites
        let opposite = get_opposite(dir);
        let ((left_merge, left),(right_merge, right)) = get_merges(dir);

        if path[opposite] > 0 {
            *path.get_mut(opposite).unwrap() -= 1;
        } else if path[left_merge] > 0 {
            *path.get_mut(left_merge).unwrap() -= 1;
            *path.get_mut(left).unwrap() += 1;
        } else if path[right_merge] > 0 {
            *path.get_mut(right_merge).unwrap() -= 1;
            *path.get_mut(right).unwrap() += 1;
        } else {
            *path.get_mut(dir).unwrap() += 1;
        }

        max_distance = std::cmp::max(max_distance, path.values().sum());
    }

    max_distance
}

#[test]
fn test_part2() {}

#[macro_use]
extern crate adventofcode;

fn main() {
    let input = get_puzzle_input!(day11);

    let part1_answer = part1(input);
    println!("Part 1: {}", part1_answer);

    let part2_answer = part2(input);
    println!("Part 2: {}", part2_answer);
}