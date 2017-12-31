fn process_part1(offsets: &str) -> i32 {
    let mut memory: Vec<i32> = offsets
        .split('\n')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let mut counter = 0;
    let mut pos: i32 = 0;
    while pos >= 0 && pos < memory.len() as i32 {
        counter += 1;
        let idx = pos as usize;
        pos += memory[idx];
        memory[idx] += 1;
    }

    counter
}

fn process_part2(offsets: &str) -> i32 {
    let mut memory: Vec<i32> = offsets
        .split('\n')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let mut counter = 0;
    let mut pos: i32 = 0;
    while pos >= 0 && pos < memory.len() as i32 {
        counter += 1;
        let idx = pos as usize;
        let offset = memory[idx];

        pos += offset;
        if offset >= 3 {
            memory[idx] -= 1;
        } else {
            memory[idx] += 1;
        }
    }

    counter
}

#[test]
fn test_part1() {
    assert_eq!(process_part1("0\n3\n0\n1\n-3"), 5);
}

#[test]
fn test_part2() {
    assert_eq!(process_part2("0\n3\n0\n1\n-3"), 10);
}

#[macro_use]
extern crate adventofcode;

fn main() {
    let input = get_puzzle_input!(day5);

    let part1_answer = process_part1(input);
    println!("Part 1: {}", part1_answer);

    let part2_answer = process_part2(input);
    println!("Part 2: {}", part2_answer);
}