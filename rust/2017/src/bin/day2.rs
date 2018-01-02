fn read_spreadsheet(spreadsheet: &str) -> Vec<Vec<i32>>
{
    let mut v = Vec::new();
    for row in spreadsheet.trim().split('\n') {
        let row_vec: Vec<i32> = row.trim().split('\t')
                                   .map(|x| x.parse::<i32>().unwrap())
                                   .collect();

        v.push(row_vec);
    }

    v
}

fn spreadsheet_checksum(spreadsheet: &str, row_checksum_fn: fn(&[i32]) -> Option<i32>) -> i32 {
    read_spreadsheet(spreadsheet).iter().fold(0, |sum, row| sum + row_checksum_fn(row).unwrap())
}

fn row_checksum_part1(row: &[i32]) -> Option<i32> {
    match (row.iter().min(), row.iter().max()) {
        (Some(min), Some(max)) => Some(max - min),
        _ => None
    }
}

fn row_checksum_part2(row: &[i32]) -> Option<i32> {
    for lhs_i in 0..row.len() {
        for rhs_i in lhs_i + 1..row.len() {
            match (row[lhs_i], row[rhs_i]) {
                (num1, num2) if num1 % num2 == 0 => return Some(num1 / num2),
                (num1, num2) if num2 % num1 == 0 => return Some(num2 / num1),
                _ => (),
            }
        }
    }

    None
}


#[test]
fn test_spreadsheet_checksum() {
    let test_input = r#"
5\t9\t2\t8\n\
9\t4\t7\t3\n\
3\t8\t6\t5\n"#.trim();

    assert_eq!(spreadsheet_checksum(test_input, row_checksum_part1), 18);
}

#[test]
fn test_sum_of_divisibles() {
    let test_input = r#"
5\t9\t2\t8\n\
9\t4\t7\t3\n\
3\t8\t6\t5\n"#.trim();

    assert_eq!(spreadsheet_checksum(test_input, row_checksum_part2), 9)
}

#[macro_use]
extern crate adventofcode;

fn main() {
    let input = get_puzzle_input!(day2);

    let part1_answer = spreadsheet_checksum(input, row_checksum_part1);
    println!("Part 1: {}", part1_answer);

    let part2_answer = spreadsheet_checksum(input, row_checksum_part2);
    println!("Part 2: {}", part2_answer);
}