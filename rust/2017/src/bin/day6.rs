use std::collections::{HashSet, HashMap};

fn redistribute_blocks(banks: &mut Vec<i32>) {
    let max = *banks.iter().max().unwrap();
    let mut index = banks.iter().position(|x| *x == max).unwrap();
    let mut count = banks[index];
    banks[index] = 0;

    let num_banks = banks.len();
    while count > 0 {
        index += 1;
        banks[index % num_banks] += 1;
        count -= 1;
    }
}

fn parse_banks(banks_str: &str) -> Vec<i32> {
    banks_str.split('\t')
             .map(|s| s.parse::<i32>().unwrap())
             .collect()
}

fn cycles_to_repeat(banks_str: &str) -> i32 {
    let mut banks = parse_banks(banks_str);
    let mut seen_banks = HashSet::new();
    let mut cycle = 0;

    while seen_banks.insert(banks.clone()) {
        cycle += 1;
        redistribute_blocks(banks.as_mut());
    }

    cycle
}

fn cycle_length(banks_str: &str) -> i32 {
    let mut banks = parse_banks(banks_str);
    let mut seen_banks = HashMap::new();
    let mut cycle = 0;

    loop {
        match seen_banks.insert(banks.clone(), cycle) {
            None => (), // Not seen before
            Some(cycle_encountered) => return cycle - cycle_encountered,
        }
        cycle += 1;
        redistribute_blocks(banks.as_mut());
    }
}

#[test]
fn test_redistribute_blocks() {
    let mut banks = vec![0, 2, 7, 0];
    redistribute_blocks(&mut banks);
    assert_eq!(banks, vec![2, 4, 1, 2]);
    redistribute_blocks(&mut banks);
    assert_eq!(banks, vec![3, 1, 2, 3]);
    redistribute_blocks(&mut banks);
    assert_eq!(banks, vec![0, 2, 3, 4]);
    redistribute_blocks(&mut banks);
    assert_eq!(banks, vec![1, 3, 4, 1]);
    redistribute_blocks(&mut banks);
    assert_eq!(banks, vec![2, 4, 1, 2]);
}

#[test]
fn test_cycles_to_repeat() {
    let cycles = cycles_to_repeat("0\t2\t7\t0");
    assert_eq!(cycles, 5);
}

#[test]
fn test_cycle_length() {
    let cycles = cycle_length("0\t2\t7\t0");
    assert_eq!(cycles, 4);
}

#[macro_use]
extern crate adventofcode;

fn main() {
    let input = get_puzzle_input!(day6);

    let part1_answer = cycles_to_repeat(input);
    println!("Part 1: {}", part1_answer);

    let part2_answer = cycle_length(input);
    println!("Part 2: {}", part2_answer);
}