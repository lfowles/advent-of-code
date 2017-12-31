enum State {
    Garbage,
    Ignore,
    Group,
}

fn group_score(stream: &str) -> i32 {
    let mut state = State::Group;
    let mut score = 0;
    let mut depth = 0;
    for c in stream.chars() {
        state = match state {
            State::Garbage => match c {
                '>' => State::Group,
                '!' => State::Ignore,
                _ => State::Garbage,
            },
            State::Ignore => State::Garbage,
            State::Group => match c {
                '{' => {
                    depth += 1;
                    State::Group
                }
                '}' => {
                    score += depth;
                    depth -= 1;
                    State::Group
                }
                '<' => State::Garbage,
                _ => State::Group,
            },
        };
    }

    score
}

fn garbage_count(stream: &str) -> i32 {
    let mut state = State::Group;
    let mut garbage_count = 0;
    for c in stream.chars() {
        state = match state {
            State::Garbage => match c {
                '>' => State::Group,
                '!' => State::Ignore,
                _ => {
                    garbage_count += 1;
                    State::Garbage
                }
            },
            State::Ignore => State::Garbage,
            State::Group => match c {
                '{' | '}' => State::Group,
                '<' => State::Garbage,
                _ => State::Group,
            },
        };
    }

    garbage_count
}

#[test]
fn test_group_score() {
    assert_eq!(group_score("{}"), 1);
    assert_eq!(group_score("{{{}}}"), 6);
    assert_eq!(group_score("{{},{}}"), 5);
    assert_eq!(group_score("{{{},{},{{}}}}"), 16);
    assert_eq!(group_score("{<a>,<a>,<a>,<a>}"), 1);
    assert_eq!(group_score("{{<ab>},{<ab>},{<ab>},{<ab>}}"), 9);
    assert_eq!(group_score("{{<!!>},{<!!>},{<!!>},{<!!>}}"), 9);
    assert_eq!(group_score("{{<a!>},{<a!>},{<a!>},{<ab>}}"), 3);
}

#[test]
fn test_garbage_count() {
    assert_eq!(garbage_count("<>"), 0);
    assert_eq!(garbage_count("<random characters>"), 17);
    assert_eq!(garbage_count("<<<<>"), 3);
    assert_eq!(garbage_count("<{!>}>"),2);
    assert_eq!(garbage_count("<!!>"), 0);
    assert_eq!(garbage_count("<!!!>>"), 0);
    assert_eq!(garbage_count(r#"<{o"i!a,<{i<a>"#), 10);
}

#[macro_use]
extern crate adventofcode;

fn main() {
    let input = get_puzzle_input!(day9);

    let part1_answer = group_score(input);
    println!("Part 1: {}", part1_answer);

    let part2_answer = garbage_count(input);
    println!("Part 2: {}", part2_answer);
}