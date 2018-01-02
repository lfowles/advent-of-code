fn ring_size(ring_num: i32) -> i32 {
    assert!(ring_num >= 0, "Ring number must be positive");
    match ring_num {
        0 => 1,
        _ => 8 * ring_num,
    }
}

fn ring_max(ring_num: i32) -> i32 {
    (2 * ring_num + 1).pow(2)
}

fn ring_min(ring_num: i32) -> i32 {
    ring_max(ring_num - 1) + 1
}

fn edge_distance(ring_num: i32) -> i32 {
    ring_size(ring_num) / 4
}

#[derive(Copy, Clone)]
enum Direction {
    Right = 0,
    Up = 1,
    Left = 2,
    Down = 3,
}

fn ring_cardinal_index(ring_num: i32, direction: Direction) -> i32 {
    let right_cardinal = ring_min(ring_num) + ring_num - 1;
    right_cardinal + direction as i32 * edge_distance(ring_num)
}

// Part 1
fn distance_from_center(square_num: i32) -> i32 {
    // memory index is 1-based
    let ring_num = (((square_num as f32).sqrt() - 1.0) / 2.0).ceil() as i32;

    use Direction::*;
    let ring_distance = [Up, Down, Left, Right].iter()
                                               .map(|dir| ring_cardinal_index(ring_num, *dir))
                                               .map(|cardinal_index| (cardinal_index - square_num).abs())
                                               .min().unwrap();

    ring_num + ring_distance
}

// Part 2
// Todo: Add predicate
fn stress_test(_value: i32) -> i32 {
    unimplemented!();
}

#[test]
fn test_part1() {
    assert_eq!(distance_from_center(1), 0);
    assert_eq!(distance_from_center(12), 3);
    assert_eq!(distance_from_center(23), 2);
    assert_eq!(distance_from_center(1024), 31);
}

/*
#[test]
fn test_part2() {
    assert_eq!(stress_test_value(1), 1);
    assert_eq!(stress_test_value(2), 1);
    assert_eq!(stress_test_value(3), 2);
    assert_eq!(stress_test_value(4), 4);
    assert_eq!(stress_test_value(5), 5);
    assert_eq!(stress_test_value(6), 10);
    assert_eq!(stress_test_value(7), 11);
    assert_eq!(stress_test_value(8), 23);
    assert_eq!(stress_test_value(9), 25);
    assert_eq!(stress_test_value(10), 26);
    assert_eq!(stress_test_value(11), 54);
    assert_eq!(stress_test_value(12), 57);
    assert_eq!(stress_test_value(13), 59);
    assert_eq!(stress_test_value(14), 122);
    assert_eq!(stress_test_value(15), 133);
    assert_eq!(stress_test_value(16), 142);
    assert_eq!(stress_test_value(17), 147);
    assert_eq!(stress_test_value(18), 304);
    assert_eq!(stress_test_value(19), 330);
    assert_eq!(stress_test_value(20), 351);
    assert_eq!(stress_test_value(21), 362);
    assert_eq!(stress_test_value(22), 747);
    assert_eq!(stress_test_value(23), 806);
}
*/

fn main() {
    let input = 361527;

    let part1_answer = distance_from_center(input);
    println!("Part 1: {}", part1_answer);

    let part2_answer = stress_test(input);
    println!("Part 2: {}", part2_answer);
}