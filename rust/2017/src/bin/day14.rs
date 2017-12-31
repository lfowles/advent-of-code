fn knot_hash_round(list: &mut Vec<u8>, lengths: &Vec<i32>, position: &mut usize, skip_size: &mut usize) {
    let list_size = list.len();

    for &length in lengths {
        let reversed = (*position..*position + length as usize).map(|i| i % list_size).rev();
        let new_elems = (*position..*position + length as usize)
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

fn knot_hash(input: &String) -> Vec<u8> {
    let extra_lengths = [17, 31, 73, 47, 23];
    let rounds = 64;
    let mut skip_size = 0;
    let mut position = 0;

    let mut list: Vec<u8> = (0..256).map(|x| x as u8).collect();
    let mut lengths: Vec<i32> = input.bytes().map(|x| x as i32).collect();
    lengths.extend(extra_lengths.iter());

    for _ in 0..rounds {
        knot_hash_round(&mut list, &lengths, &mut position, &mut skip_size);
    }

    let mut dense_hash = Vec::new();

    for i in 0..16 {
        let start = i * 16;
        let end = start + 16;
        let hash = list[start..end].iter().fold(0u8, |acc, &x| acc ^ x);
        dense_hash.push(hash);
    }

    dense_hash
}

fn print_defrag(hash: &Vec<u8>) {
    let mut line = String::new();

    for i in 0..128 {
        let idx = i / 8;
        let bit = i % 8;

        let c = match hash[idx] >> (7 - bit) & 0x1 {
            0 => '.',
            1 => '#',
            _ => unreachable!(),
        };
        line.push(c);
    }

    println!("{}", line);
}

fn part1(input: &str) -> i32 {
    let mut blocks = 0;
    for i in 0..128 {
        let result = knot_hash(&format!("{}-{}", input, i));
        for piece in result {
            blocks += piece.count_ones() as i32;
        }
    }

    blocks
}

#[test]
fn test_part1() {
    let test_input = "flqrgnkx";

    assert_eq!(part1(test_input), 8108);
}

fn flood_fill(row: i32, bit: i32, mut disk: &mut Vec<Vec<u8>>) -> bool {
    if row < 0 || row >= disk.len() as i32
        || bit < 0 || bit >= 128 {
        return false;
    }

    let idx = bit / 8;
    let sub_bit = bit % 8;
    let value = (disk[row as usize][idx as usize] >> (7 - sub_bit)) & 0x1;
    if value == 0 {
        return false;
    }

    // If set, unset and visit neighbors


    let mask = !(1 << (7 - sub_bit));
    disk[row as usize][idx as usize] &= mask;

    let neighbors = [
        (row + 1, bit),
        (row - 1, bit),
        (row, bit + 1),
        (row, bit - 1)
    ];

    for &(neighbor_row, neighbor_idx) in neighbors.iter() {
        flood_fill(neighbor_row, neighbor_idx, &mut disk);
    }

    true
}

fn part2(input: &str) -> i32 {
    let mut disk: Vec<Vec<u8>> = Vec::new();


    for i in 0..128 {
        let result = knot_hash(&format!("{}-{}", input, i));
        disk.push(result);
    }

    let mut region_count = 0;
    for bit in 0..128 {
        for row in 0..128 {
            if flood_fill(row, bit, &mut disk) {
                region_count += 1;
            }
        }
    }

    region_count
}

#[test]
fn test_part2() {
    let test_input = "flqrgnkx";

    assert_eq!(part2(test_input), 1242);
}

fn main() {
    let input = "stpzcrnm";

    let part1_answer = part1(input);
    println!("Part 1: {}", part1_answer);

    let part2_answer = part2(input);
    println!("Part 2: {}", part2_answer);
}