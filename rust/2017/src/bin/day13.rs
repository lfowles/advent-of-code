use std::collections::HashMap;

fn read_input(input: &str) -> HashMap<i32, i32> {
    let mut firewall = HashMap::new();

    for layer in input.split('\n') {
        let info: Vec<i32> = layer.split(": ").map(|s| s.parse::<i32>().unwrap()).collect();
        firewall.insert(info[0], info[1]);
    }

    firewall
}

fn part1(input: &str) -> i32 {
    let firewall = read_input(input);

    let mut severity = 0;
    for (&layer, &depth) in firewall.iter() {
        let scanner_cycle = depth * 2 - 2;
        if layer % scanner_cycle == 0 {
            severity += layer * depth;
        }
    }

    severity
}

#[test]
fn test_part1() {
    let test_input = r#"
0: 3
1: 2
4: 4
6: 4
"#.trim();

    assert_eq!(part1(test_input), 24);
}

fn is_caught(firewall: &HashMap<i32, i32>, delay: i32) -> bool {
    for (&layer, &depth) in firewall.iter() {
        let scanner_cycle = depth * 2 - 2;
        if (layer + delay) % scanner_cycle == 0 {
            return true;
        }
    }

    false
}

fn part2(input: &str) -> i32 {
    let firewall = read_input(input);
    let mut delay = 0;

    while is_caught(&firewall, delay) {
        delay += 1;
    }

    delay
}

#[test]
fn test_part2() {
    let test_input = r#"
0: 3
1: 2
4: 4
6: 4
"#.trim();

    assert_eq!(part2(test_input), 10);
}

#[macro_use]
extern crate adventofcode;

fn main() {
    let input = get_puzzle_input!(day13);

    let part1_answer = part1(input);
    println!("Part 1: {}", part1_answer);

    let part2_answer = part2(input);
    println!("Part 2: {}", part2_answer);
}