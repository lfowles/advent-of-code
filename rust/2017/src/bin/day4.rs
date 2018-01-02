use std::collections::HashSet;

pub fn num_valid_passphrases(passphrases: &str, check_fn: fn(&str) -> bool) -> i32 {
    let result = passphrases.trim()
                            .split('\n')
                            .filter(|passphrase| check_fn(*passphrase))
                            .count();
    result as i32
}

pub fn part1_check(passphrase: &str) -> bool {
    let mut unique_words = HashSet::new();
    for word in passphrase.split(' ') {
        if !unique_words.insert(word) {
            return false;
        }
    }
    true
}

pub fn part2_check(passphrase: &str) -> bool {
    let mut unique_words = HashSet::new();
    for word in passphrase.split(' ') {
        let mut canonical_word = word.bytes().collect::<Vec<_>>();
        canonical_word.sort();
        if !unique_words.insert(canonical_word) {
            return false;
        }
    }
    true
}

#[test]
fn test_part1() {
    assert!(part1_check("aa bb cc dd ee"));
    assert!(!part1_check("aa bb cc dd aa"));
    assert!(part1_check("aa bb cc dd aaa"));
}

#[test]
fn test_numvalid_part1() {
    let passphrases = r#"
aa bb cc dd ee
aa bb cc dd aa
aa bb cc dd aaa
"#;
    assert_eq!(num_valid_passphrases(passphrases, part1_check), 2);
}

#[test]
fn test_part2() {
    assert!(part2_check("abcde fghij"));
    assert!(!part2_check("abcde xyz ecdab"));
    assert!(part2_check("a ab abc abd abf abj"));
    assert!(part2_check("iiii oiii ooii oooi oooo"));
    assert!(!part2_check("oiii ioii iioi iiio"));
}

#[test]
fn test_numvalid_part2()
{
    let passphrases = r#"
abcde fghij
abcde xyz ecdab
a ab abc abd abf abj
iiii oiii ooii oooi oooo
oiii ioii iioi iiio
"#;
    assert_eq!(num_valid_passphrases(passphrases, part2_check), 3);
}

#[macro_use]
extern crate adventofcode;

fn main() {
    let input = get_puzzle_input!(day4);

    let part1_answer = num_valid_passphrases(input, part1_check);
    println!("Part 1: {}", part1_answer);

    let part2_answer = num_valid_passphrases(input, part2_check);
    println!("Part 2: {}", part2_answer);
}