fn letter_score(letter: u8) -> usize {
    usize::from((letter & 0b0001_1111) + (1 ^ ((letter & 0b0010_0000) >> 5)) * 26)
}

pub fn part_1(input: &str) -> usize {
    input.lines()
        .map(|l| l.as_bytes())
        .map(|line| {
            // Split line into two
            let mid = line.len() / 2;
            (&line[0..mid], &line[mid..])
        })
        .map(|(compartment_1, compartment_2)| {
            // Mark which have been seen
            let mut seen_in_first_compartment = [false; 256];
            for &i in compartment_1.iter() {
                seen_in_first_compartment[usize::from(i)] = true;
            }
            // Find first entry already seen
            compartment_2.into_iter()
                .copied()
                .filter(|b| seen_in_first_compartment[usize::from(*b)])
                .map(letter_score)
                .next()
                .unwrap_or_default()
        }).sum()
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
