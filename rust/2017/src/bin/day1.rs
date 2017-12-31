pub fn captcha(captcha: &str, offset: i32) -> i32
{
    let mut sum = 0;

    let zero = b'0';
    let chars = captcha.as_bytes();
    for i in 0..chars.len() {
        let paired_index = ((i as i32 + offset) % chars.len() as i32) as usize;
        if chars[i] == chars[paired_index] {
            sum += (chars[i] - zero) as i32;
        }
    }

    sum
}

#[test]
fn test_part1() {
    assert_eq!(captcha("1122", 1), 3);
    assert_eq!(captcha("1111", 1), 4);
    assert_eq!(captcha("1234", 1), 0);
    assert_eq!(captcha("91212129", 1), 9);
}

#[test]
fn test_part2() {
    assert_eq!(captcha("1212", 2), 6);
    assert_eq!(captcha("1221", 2), 0);
    assert_eq!(captcha("123425", 3), 4);
    assert_eq!(captcha("123123", 3), 12);
    assert_eq!(captcha("12131415", 4), 4);
}

#[macro_use]
extern crate adventofcode;

fn main() {
    let input = get_puzzle_input!(day1);

    let part1_answer = captcha(input, 1);
    println!("Part 1: {}", part1_answer);

    let part2_answer = captcha(input, (input.len() / 2) as i32);
    println!("Part 2: {}", part2_answer);
}