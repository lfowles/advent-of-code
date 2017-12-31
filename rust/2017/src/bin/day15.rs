fn generatorA(value: u32, picky: bool) -> u32 {
    let factor: u64 = 16807;
    let mut new_value = ((value as u64).wrapping_mul(factor) % 0x7FFFFFFF) as u32;
    if picky {
        while new_value % 4 != 0 {
            new_value = ((new_value as u64).wrapping_mul(factor) % 0x7FFFFFFF) as u32;
        };
    }

    new_value
}

fn generatorB(value: u32, picky: bool) -> u32 {
    let factor: u64 = 48271;
    let mut new_value = ((value as u64).wrapping_mul(factor) % 0x7FFFFFFF) as u32;
    if picky {
        while new_value % 8 != 0 {
            new_value = ((new_value as u64).wrapping_mul(factor) % 0x7FFFFFFF) as u32;
        };
    }

    new_value
}

fn part1(init_a: u32, init_b: u32) -> i32 {
    let mut value1 = init_a;
    let mut value2 = init_b;
    let mut matches = 0;

    for i in 0..40000000 {
        value1 = generatorA(value1, false);
        value2 = generatorB(value2, false);
        if value1 & 0xFFFF == value2 & 0xFFFF {
            matches += 1;
        }
    }
    matches
}


#[test]
fn test_part1() {
    assert_eq!(part1(65, 8921), 588);
}

fn part2(init_a: u32, init_b: u32) -> i32 {
    let mut value1 = init_a;
    let mut value2 = init_b;
    let mut matches = 0;

    for i in 0..5000000 {
        value1 = generatorA(value1, true);
        value2 = generatorB(value2, true);
        if value1 & 0xFFFF == value2 & 0xFFFF {
            matches += 1;
        }
    }
    matches
}

#[test]
fn test_part2() {
    assert_eq!(part2(65, 8921), 309);
}

fn main() {
    let init_a = 116;
    let init_b = 299;

    let part1_answer = part1(init_a, init_b);
    println!("Part 1: {}", part1_answer);

    let part2_answer = part2(init_a, init_b);
    println!("Part 2: {}", part2_answer);
}