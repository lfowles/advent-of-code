fn knot_hash_round(list: &mut Vec<i32>, lengths: &Vec<i32>, position: &mut usize, skip_size: &mut usize) {
    let list_size = list.len();

    for &length in lengths {
        let reversed = (*position..*position+length as usize).map(|i| i % list_size).rev();
        let new_elems = (*position..*position+length as usize)
            .map(|i| i % list_size)
            .zip(reversed)
            .filter(|&(pos, rev_pos)| pos < rev_pos);

        for (pos, rev_pos) in new_elems {
            list.swap(pos, rev_pos);
        }

        *position += length as usize + *skip_size;
        *skip_size += 1;
    }
}

fn part1(input: &str, list_size: i32) -> i32 {
    let lengths: Vec<i32> =     input.split(',').map(|s| s.trim().parse::<i32>().unwrap()).collect();

    let mut list: Vec<i32> = (0..list_size).collect();

    let mut position = 0;
    let mut skip_size = 0;
    knot_hash_round(&mut list, &lengths, &mut position, &mut skip_size);

    list[0] * list[1]
}

#[test]
fn test_part1() {
    assert_eq!(part1("3, 4, 1, 5", 5), 12);
}

fn part2(input: &str) -> String {
    let extra_lengths = [17, 31, 73, 47, 23];
    let rounds = 64;
    let mut skip_size = 0;
    let mut position = 0;

    let mut list: Vec<i32> = (0..256).collect();
    let mut lengths: Vec<i32> = input.bytes().map(|x| x as i32).collect();
    lengths.extend(extra_lengths.iter());

    for _ in 0..rounds {
        knot_hash_round(&mut list, &lengths, &mut position, &mut skip_size);
    }

    let mut dense_hash = Vec::new();

    for i in 0..16 {
        let start = i * 16;
        let end = start + 16;
        let hash = list[start..end].iter().fold(0, |acc, &x| acc ^ x );
        dense_hash.push(hash);

    }

    let mut final_hash = String::new();
    for hash in &dense_hash {
        final_hash.extend(format!("{:02x}", *hash as u8).chars());
    }

    final_hash
}

#[test]
fn test_part2() {
    assert_eq!(part2(""), "a2582a3a0e66e6e86e3812dcb672a272");
    assert_eq!(part2("AoC 2017"), "33efeb34ea91902bb2f59c9920caa6cd");
    assert_eq!(part2("1,2,3"), "3efbe78a8d82f29979031a4aa0b16a9d");
    assert_eq!(part2("1,2,4"), "63960835bcdc130f0b66d7ff4f6a5a8e");
}

#[macro_use]
extern crate adventofcode;

fn main() {
    let input = get_puzzle_input!(day10);

    let part1_answer = part1(input, 256);
    println!("Part 1: {}", part1_answer);

    let part2_answer = part2(input);
    println!("Part 2: {}", part2_answer);
}