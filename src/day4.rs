use std::{cmp::Ordering};

#[derive(Debug)]
struct Range(u8, u8);

fn one_or_two_digits_to_number(s: &str) -> u8 {
    match s.as_bytes() {
        &[a] => a & 0b1111,
        &[a, b] => (a << 4) | (b & 0b1111),
        _ => unreachable!()
    }
}

impl Range {
    fn parse(s: &str) -> Range {
        let (a, b) = s.split_once('-').unwrap();
        Range(
            one_or_two_digits_to_number(a),
            one_or_two_digits_to_number(b),
        )
    }

    fn contains_or_is_contained_by(&self, other: &Range) -> bool {
        // True, except if both are equal and not both zero
        match (self.0.cmp(&other.0), self.1.cmp(&other.1)) {
            (Ordering::Greater, Ordering::Greater) => false,
            (Ordering::Less, Ordering::Less) => false,
            _ => true
        }
    }

    fn overlaps_with(&self, other: &Range) -> bool {
        self.0 <= other.1 && other.0 <= self.1
    }
}

pub fn part_1(input: &str) -> usize {
    input
        .split_terminator('\n')
        .map(|line| line.split_once(',').unwrap())
        .map(|(range_1, range_2)| (Range::parse(range_1), Range::parse(range_2)))
        .filter(|(range_1, range_2)| range_1.contains_or_is_contained_by(range_2))
        .count()
}

pub fn part_2(input: &str) -> usize {
    input
        .split_terminator('\n')
        .map(|line| line.split_once(',').unwrap())
        .map(|(range_1, range_2)| (Range::parse(range_1), Range::parse(range_2)))
        .filter(|(range_1, range_2)| range_1.overlaps_with(range_2))
        .count()
}

#[test]
fn test_part_1_example() {
    let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
    assert_eq!(part_1(input), 2);
}

#[test]
fn test_part_2_example() {
    let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
    assert_eq!(part_2(input), 4);
}

#[test]
fn test_more_ranges() {
    assert!(Range::parse("71-71").contains_or_is_contained_by(&Range::parse("42-72")));
    assert!(Range::parse("27-28").contains_or_is_contained_by(&Range::parse("27-99")));
    assert!(!Range::parse("15-79").contains_or_is_contained_by(&Range::parse("14-78")));
    assert!(Range::parse("2-34").contains_or_is_contained_by(&Range::parse("1-92")));
    assert!(!Range::parse("4-94").contains_or_is_contained_by(&Range::parse("2-93")));
}

#[test]
fn test_part_1() {
    let input = include_str!("../input/2022/day4.txt");
    assert_eq!(part_1(input), 571);
}
