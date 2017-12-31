#[derive(Debug)]
enum DanceMove {
    Spin(i32),
    Exchange(i32, i32),
    Partner(char, char),
}

fn get_moves(input: &str) -> Vec<DanceMove> {
    let mut moves = Vec::new();

    for move_str in input.split(',') {
        let move_string = move_str.to_string();
        let dance_move = match move_string[0] {
            's' => DanceMove::Spin(move_string[1..].parse::<i32>().unwrap()),
            'x' => {
                let positions: Vec<i32> = move_string[1..].split('/').map(|x| x.parse::<i32>().unwrap()).collect();
                DanceMove::Exchange(positions[0], positions[1])
            }
            'p' => DanceMove::Partner(move_string[1], move_string[3]),
            _ => panic!(),
        };
        moves.push(dance_move);
    }

    println!("{:#?}", moves);

    moves
}

fn part1(input: &str, programs: &String) -> String {
    get_moves(input);

    *programs
}

#[test]
fn test_part1() {
    let test_input = "s1,x3/4,pe/b";
    assert_eq!(part1(test_input, &"abcde".to_string()), "baedc");
}

fn part2(_input: &str) -> i32 {
    unimplemented!();
}

#[test]
fn test_part2() {}

#[macro_use]
extern crate adventofcode;

fn main() {
    let input = get_puzzle_input!(day16);

    let part1_answer = part1(input, &"abcdefghijklmnop".to_string());
    println!("Part 1: {}", part1_answer);

    let part2_answer = part2(input);
    println!("Part 2: {}", part2_answer);
}