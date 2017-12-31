#[derive(Debug, PartialEq)]
enum Argument {
    Immediate(i32),
    Register(String),
}

#[derive(Debug, PartialEq)]
enum Operation {
    Inc(Argument),
    Dec(Argument),
}

#[derive(Debug, PartialEq)]
enum Condition {
    LessThan(Argument, Argument),
    LessThanEqual(Argument, Argument),
    GreaterThan(Argument, Argument),
    GreaterThanEqual(Argument, Argument),
    Equal(Argument, Argument),
    NotEqual(Argument, Argument),
}

#[derive(Debug, PartialEq)]
struct Instruction {
    register: String,
    operation: Operation,
    condition: Condition,
}

fn parse_instruction(s: &str) -> Instruction {
    // 0 1   2 3  4 5 6
    // b inc 5 if a > 1
    let tokens: Vec<&str> = s.trim().split(' ').collect();
    assert_eq!("if", tokens[3]);

    let register = tokens[0].to_string();

    let operation = match tokens[1] {
        "inc" => Operation::Inc,
        "dec" => Operation::Dec,
        _ => panic!(),
    }(parse_argument(tokens[2]));

    let condition = match tokens[5] {
        "<" => Condition::LessThan,
        "<=" => Condition::LessThanEqual,
        ">" => Condition::GreaterThan,
        ">=" => Condition::GreaterThanEqual,
        "==" => Condition::Equal,
        "!=" => Condition::NotEqual,
        _ => panic!(),
    }(parse_argument(tokens[4]), parse_argument(tokens[6]));

    Instruction {
        register,
        operation,
        condition,
    }
}

fn parse_argument(s: &str) -> Argument {
    match s.parse::<i32>() {
        Ok(value) => Argument::Immediate(value),
        Err(_) => Argument::Register(s.to_string()),
    }
}

fn read_instructions(s: &str) -> Vec<Instruction> {
    s.split('\n').map(|x| parse_instruction(x)).collect()
}

#[test]
fn test_read_instructions() {
    assert_eq!(
        parse_instruction("b inc 5 if a > 1"),
        Instruction {
            register: "b".to_string(),
            operation: Operation::Inc(Argument::Immediate(5)),
            condition: Condition::GreaterThan(Argument::Register("a".to_string()),
                                              Argument::Immediate(1)),
        }
    );

    assert_eq!(
        parse_instruction("a inc 1 if b < 5"),
        Instruction {
            register: "a".to_string(),
            operation: Operation::Inc(Argument::Immediate(1)),
            condition: Condition::LessThan(Argument::Register("b".to_string()),
                                           Argument::Immediate(5)),
        }
    );

    assert_eq!(
        parse_instruction("c dec -10 if a >= 1"),
        Instruction {
            register: "c".to_string(),
            operation: Operation::Dec(Argument::Immediate(-10)),
            condition: Condition::GreaterThanEqual(Argument::Register("a".to_string()),
                                                   Argument::Immediate(1)),
        }
    );

    assert_eq!(
        parse_instruction("c inc -20 if c == 10"),
        Instruction {
            register: "c".to_string(),
            operation: Operation::Inc(Argument::Immediate(-20)),
            condition: Condition::Equal(Argument::Register("c".to_string()),
                                        Argument::Immediate(10)),
        }
    );
}

use std::collections::HashMap;

fn get_register<'a>(register_map: &'a mut HashMap<String, i32>, register: &String) -> &'a mut i32 {
    register_map.entry(register.clone()).or_insert(0)
}

fn get_argument(register_map: &mut HashMap<String, i32>, argument: Argument) -> i32 {
    match argument {
        Argument::Register(register) => *get_register(register_map, &register),
        Argument::Immediate(value) => value
    }
}

fn process_part1(input: &str) -> i32 {
    let instructions = read_instructions(input);

    let mut register_map = HashMap::new();

    for instruction in instructions {
        // evaluate condition
        let condition_result = match instruction.condition {
            Condition::GreaterThan(lhs, rhs) => get_argument(&mut register_map, lhs) > get_argument(&mut register_map, rhs),
            Condition::GreaterThanEqual(lhs, rhs) => get_argument(&mut register_map, lhs) >= get_argument(&mut register_map, rhs),
            Condition::LessThan(lhs, rhs) => get_argument(&mut register_map, lhs) < get_argument(&mut register_map, rhs),
            Condition::LessThanEqual(lhs, rhs) => get_argument(&mut register_map, lhs) <= get_argument(&mut register_map, rhs),
            Condition::Equal(lhs, rhs) => get_argument(&mut register_map, lhs) == get_argument(&mut register_map, rhs),
            Condition::NotEqual(lhs, rhs) => get_argument(&mut register_map, lhs) != get_argument(&mut register_map, rhs),
        };

        // Modify register
        if condition_result {
            let modification = match instruction.operation {
                Operation::Inc(arg) => get_argument(&mut register_map, arg),
                Operation::Dec(arg) => -get_argument(&mut register_map, arg),
            };
            let reg = get_register(&mut register_map, &instruction.register);
            *reg += modification;
        }
    }

    *register_map.values().max().unwrap()
}

fn process_part2(input: &str) -> i32 {
    let instructions = read_instructions(input);

    let mut register_map = HashMap::new();
    let mut max = 0;

    for instruction in instructions {
        // evaluate condition
        let condition_result = match instruction.condition {
            Condition::GreaterThan(lhs, rhs) => get_argument(&mut register_map, lhs) > get_argument(&mut register_map, rhs),
            Condition::GreaterThanEqual(lhs, rhs) => get_argument(&mut register_map, lhs) >= get_argument(&mut register_map, rhs),
            Condition::LessThan(lhs, rhs) => get_argument(&mut register_map, lhs) < get_argument(&mut register_map, rhs),
            Condition::LessThanEqual(lhs, rhs) => get_argument(&mut register_map, lhs) <= get_argument(&mut register_map, rhs),
            Condition::Equal(lhs, rhs) => get_argument(&mut register_map, lhs) == get_argument(&mut register_map, rhs),
            Condition::NotEqual(lhs, rhs) => get_argument(&mut register_map, lhs) != get_argument(&mut register_map, rhs),
        };

        // Modify register
        if condition_result {
            let modification = match instruction.operation {
                Operation::Inc(arg) => get_argument(&mut register_map, arg),
                Operation::Dec(arg) => -get_argument(&mut register_map, arg),
            };
            let reg = get_register(&mut register_map, &instruction.register);
            *reg += modification;
            max = std::cmp::max(*reg, max);
        }
    }

    max
}

#[test]
fn test_part1() {
    let s = r#"
b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10
"#.trim();

    assert_eq!(process_part1(s), 1);
}

#[test]
fn test_part2() {
    let s = r#"
b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10
"#.trim();

    assert_eq!(process_part2(s), 10);
}

#[macro_use]
extern crate adventofcode;

fn main() {
    let input = get_puzzle_input!(day8);

    let part1_answer = process_part1(input);
    println!("Part 1: {}", part1_answer);

    let part2_answer = process_part2(input);
    println!("Part 2: {}", part2_answer);
}