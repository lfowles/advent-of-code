use std::str::FromStr;

fn read_spreadsheet(spreadsheet: &str) -> Vec<Vec<i32>>
{
    let mut v = Vec::new();
    for row in spreadsheet.trim().split("\n") {
        let mut r = Vec::new();
        for entry in row.split("\t") {
            r.push(i32::from_str(entry).unwrap());
        }
        v.push(r);
    }

    v
}

pub fn checksum(spreadsheet: &str) -> i32
{
    let mut sum = 0;
    for row in read_spreadsheet(spreadsheet) {
        sum += row.iter().max().unwrap() - row.iter().min().unwrap();
    }
    sum
}

fn row_num(row: Vec<i32>) -> Option<i32>
{
    for lhs_i in 0..row.len() {
        for rhs_i in lhs_i + 1..row.len() {
            match (row[lhs_i], row[rhs_i]) {
                (num1, num2) if num1 % num2 == 0 => return Some(num1 / num2),
                (num1, num2) if num2 % num1 == 0 => return Some(num2 / num1),
                _ => continue,
            }
        }
    }

    None
}

pub fn sum_of_divisibles(spreadsheet: &str) -> i32
{
    let mut sum = 0;
    for row in read_spreadsheet(spreadsheet) {
        sum += row_num(row).unwrap();
    }
    sum
}

#[test]
fn test_checksum()
{
    let s = "5\t1\t9\t5\n\
                   7\t5\t3\n\
                   2\t4\t6\t8\n";

    assert_eq!(checksum(s), 18);
}

#[test]
fn test_sum_of_divisibles()
{
    let s = "5\t9\t2\t8\n\
                   9\t4\t7\t3\n\
                   3\t8\t6\t5\n";

    assert_eq!(sum_of_divisibles(s), 9)
}

#[macro_use]
extern crate adventofcode;

fn main() {
    let input = get_puzzle_input!(day2);

    let part1_answer = checksum(input);
    println!("Part 1: {}", part1_answer);

    let part2_answer = sum_of_divisibles(input);
    println!("Part 2: {}", part2_answer);
}