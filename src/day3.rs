fn letter_score(letter: u8) -> usize {
    usize::from(letter & 0b0001_1111) +
        // Check if uppercase or lowercase
        if (letter & 0b0010_0000) != 0 { 0 } else { 26 }
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
            // Mark which have been seen
            let mut seen_in_first_compartment = [false; 256];
            for &item in compartment_1.iter() {
                seen_in_first_compartment[usize::from(item)] = true;
            }
            for &item in compartment_2.iter() {
                if seen_in_first_compartment[usize::from(item)] {
                    return letter_score(item);
                }
            }
            unreachable!()
        })
        .sum()
}

const FIRST_COMPARTMENT: u8 = 0b1;
const SECOND_COMPARTMENT: u8 = 0b10;
pub fn part_2(input: &str) -> usize {
    input
        .split_terminator('\n')
        .map(str::as_bytes)
        .array_chunks::<3>()
        .map(|[compartment_1, compartment_2, compartment_3]| {
            // Mark which have been seen
            let mut seen_in_compartment = [0u8; u8::MAX as usize];
            for &i in compartment_1.iter() {
                seen_in_compartment[usize::from(i)] = FIRST_COMPARTMENT;
            }
            for &i in compartment_2.iter() {
                seen_in_compartment[usize::from(i)] |= SECOND_COMPARTMENT;
            }
            for &i in compartment_3.iter() {
                if seen_in_compartment[usize::from(i)] == FIRST_COMPARTMENT | SECOND_COMPARTMENT {
                    return letter_score(i);
                }
            }
            unreachable!()
        })
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
