use std::u8;

fn letter_score(letter: u8) -> usize {
    usize::from(letter & 0b0001_1111) +
        // Check if uppercase or lowercase
        if (letter & 0b0010_0000) != 0 { 0 } else { 26 }
}

fn bag_fingerprint(line: &[u8]) -> u64 {
    line.iter().fold(0, |fingerprint, &item| {
        fingerprint | (1 << (item & 0b0011_1111))
    })
}

fn to_letter(fingerprint: u64) -> u8 {
    fingerprint.trailing_zeros() as u8
}

pub fn part_1(input: &str) -> usize {
    input
        // Split lines
        .split_terminator('\n')
        // Map from string to bytes
        .map(str::as_bytes)
        // Split into two
        .map(|line| line.split_at(line.len() / 2))
        // Find duplicates in both parts
        .map(|(compartment_1, compartment_2)| {
            bag_fingerprint(compartment_1) & bag_fingerprint(compartment_2)
        })
        .map(to_letter)
        .map(letter_score)
        .sum()
}

pub fn part_2(input: &str) -> usize {
    input
        .split_terminator('\n')
        .map(str::as_bytes)
        .array_chunks::<3>()
        .map(|arr| {
            arr.into_iter()
                .map(bag_fingerprint)
                .reduce(|acc, new| acc & new)
                .unwrap_or(0)
        })
        .map(to_letter)
        .map(letter_score)
        .sum()
}

#[test]
fn test_letter_scores() {
    for (score, letter) in (1..=26).zip(b'a'..=b'z') {
        assert_eq!(letter_score(letter), score);
    }
    for (score, letter) in (27..=52).zip(b'A'..=b'Z') {
        assert_eq!(letter_score(letter), score);
    }
}

#[test]
fn test_part_1_example() {
    let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
    assert_eq!(part_1(input), 157);
}

#[test]
fn test_part_2_example() {
    let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
    assert_eq!(part_2(input), 70);
}

#[test]
fn test_part_1() {
    let input = include_str!("../input/2022/day3.txt");
    assert_eq!(part_1(input), 8515);
}

#[test]
fn test_part_2() {
    let input = include_str!("../input/2022/day3.txt");
    assert_eq!(part_2(input), 2434);
}
