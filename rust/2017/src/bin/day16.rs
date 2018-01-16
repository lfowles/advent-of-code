// error_chain boilerplate
#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;

mod errors {
    error_chain! {}
}

use errors::*;

use std::str::FromStr;

#[derive(Debug)]
enum DanceMove {
    Spin(i32),
    Exchange(i32, i32),
    Partner(u8, u8),
}

impl DanceMove {
    fn parse_spin(s: &str) -> Result<Self> {
        assert_eq!(s.chars().nth(0), Some('s'));
        let arg = s.get(1..).ok_or("No spin number found")?;
        let spin_move = DanceMove::Spin(arg.parse().chain_err(|| "failed to parse spin arg")?);
        Ok(spin_move)
    }

    fn parse_exchange(s: &str) -> Result<Self> {
        assert_eq!(s.chars().nth(0), Some('x'));
        let mut args = s.get(1..).ok_or("No exchange args found")?.split('/');
        let pos1 = args.next().ok_or("Exchange position 1 missing")?.parse().chain_err(|| "Failed to parse exchange pos 1")?;
        let pos2 = args.next().ok_or("Exchange position 2 missing")?.parse().chain_err(|| "Failed to parse exchange pos 2")?;
        let exchange_move = DanceMove::Exchange(pos1, pos2);
        Ok(exchange_move)
    }

    fn parse_partner(s: &str) -> Result<Self> {
        assert_eq!(s.chars().nth(0), Some('p'));
        let mut args = s.get(1..).ok_or("No partner args found")?.split('/');

        let partner1 = args.next().ok_or("Partner 1 missing")?;
        if partner1.len() != 1 {
            bail!("Partner 1 wrong size: '{}'", partner1);
        }

        let partner2 = args.next().ok_or("Partner 2 missing")?;
        if partner2.len() != 1 {
            bail!("Partner 2 wrong size: '{}'", partner2);
        }

        if !s.is_ascii() {
            bail!("Partner args must be ascii");
        }

        let partner_move = DanceMove::Partner(partner1.chars().nth(0).unwrap() as u8, partner2.chars().nth(0).unwrap() as u8);
        Ok(partner_move)
    }
}

impl FromStr for DanceMove {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let move_type = s.chars().nth(0).ok_or("No dance move in empty string")?;

        match move_type {
            's' => DanceMove::parse_spin(s),
            'x' => DanceMove::parse_exchange(s),
            'p' => DanceMove::parse_partner(s),
            m => bail!(format!("{}: '{}'", "Unknown dance move", m)),
        }
    }
}

fn get_moves(input: &str) -> Result<Vec<DanceMove>> {
    input
        .split(',')
        .map(str::parse::<DanceMove>)
        .collect()
}

fn part1(input: &str, programs: &String) -> Result<String> {
    let moves = get_moves(input)?;
    if !programs.is_ascii() {
        bail!("Program line must be ascii");
    }

    let mut program_line = programs.clone().into_bytes();
    let line_size = program_line.len();

    for m in moves {
        match m {
            DanceMove::Spin(spin) => {
                let start = line_size - spin as usize;
                let new_program_line: Vec<_> = program_line.iter().cycle().skip(start).take(line_size).map(|&x| x).collect();
                program_line.clone_from(&new_program_line);
            }
            DanceMove::Exchange(pos1, pos2) => {
                if pos1 < 0 || pos1 >= line_size as i32 || pos2 < 0 || pos2 >= line_size as i32 {
                    bail!("Invalid exchange {}<->{}", pos1, pos2);
                }
                program_line.swap(pos1 as usize, pos2 as usize)
            }
            DanceMove::Partner(program1, program2) => {
                let pos1 = program_line.iter().position(|&c| c == program1).ok_or(format!("Program '{}' doesn't exist in the line.", program1))?;
                let pos2 = program_line.iter().position(|&c| c == program2).ok_or(format!("Program '{}' doesn't exist in the line.", program2))?;
                program_line.swap(pos1 as usize, pos2 as usize);
            }
        }
    }

    Ok(String::from_utf8(program_line).unwrap())
}

#[test]
fn test_part1() {
    let test_input = "s1,x3/4,pe/b";
    assert_eq!(part1(test_input, &"abcde".to_string()).expect("error occurred"), "baedc");
}

fn part2(input: &str, programs: &String) -> Result<String> {
    let mut program_line = part1(&input, &programs)?;

    let mut seen_set = std::collections::HashSet::new();
    let mut cycle = 0;

    while seen_set.insert(program_line.clone().into_bytes()) {
        program_line = part1(&input, &program_line)?;
        cycle += 1;
    }

    let remaining = 1000000000 % cycle;

    program_line = programs.clone();

    for _ in 0..remaining {
        program_line = part1(&input, &program_line)?;
    }

    Ok(program_line)
}

#[test]
fn test_part2() {
    let test_input = "s1,x3/4,pe/b";
    assert_eq!(part2(test_input, &"abcde".to_string()).expect("error occurred"), "ceadb");
}

#[macro_use]
extern crate adventofcode;

fn main() {
    let input = get_puzzle_input!(day16);

    let part1_answer = part1(input, &"abcdefghijklmnop".to_string()).expect("error occurred");
    println!("Part 1: {}", part1_answer);

    let part2_answer = part2(input, &"abcdefghijklmnop".to_string()).expect("error occurred");
    println!("Part 2: {}", part2_answer);
}
