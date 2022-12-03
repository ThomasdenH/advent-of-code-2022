/// Parse a number from an iterator, up to the first newline. Returns `None` if
/// no digits are found.
fn parse_number(it: &mut impl Iterator<Item = u8>) -> Option<usize> {
    // Read until the first newline
    it.take_while(|b| *b != b'\n')
        // Fold, using `None` in case there are no digits.
        .fold(None, |acc, digit| {
            Some(acc.unwrap_or_default() * 10 + usize::from(digit) - usize::from(b'0'))
        })
}

/// Parse all numbers from input.
fn parse_numbers(s: &str) -> impl Iterator<Item = Option<usize>> + '_ {
    let mut iter = s.as_bytes().iter().copied().peekable();
    std::iter::from_fn(move || {
        if iter.peek().is_some() {
            Some(parse_number(&mut iter))
        } else {
            None
        }
    })
}

fn totals(s: &str) -> impl Iterator<Item = usize> + '_ {
    let mut numbers = parse_numbers(s);
    std::iter::from_fn(move ||
        // Take numbers up to the first `None`, unwrap Option
        (&mut numbers).map_while(|b| b)
        // Sum, returning None if there were no numbers
        .fold(None, |acc, d| Some(acc.unwrap_or_default() + d)))
}

fn sum_of_max<const AMOUNT: usize>(input: &str) -> usize {
    totals(input)
        .fold([0usize; AMOUNT], |mut acc: [usize; AMOUNT], mut new| {
            for max in acc.iter_mut() {
                if new > *max {
                    std::mem::swap(&mut new, max)
                }
            }
            acc
        })
        .iter()
        .sum()
}

pub fn part_1(input: &str) -> usize {
    sum_of_max::<1>(input)
}

pub fn part_2(input: &str) -> usize {
    sum_of_max::<3>(input)
}

#[test]
fn test_example_part_1() {
    let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
    assert_eq!(part_1(input), 24000);
}

#[test]
fn test_example_part_2() {
    let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
    assert_eq!(part_2(input), 45000);
}

#[test]
fn test_part_1() {
    let input = include_str!("../input/2022/day1.txt");
    assert_eq!(part_1(input), 70374);
}

#[test]
fn test_part_2() {
    let input = include_str!("../input/2022/day1.txt");
    assert_eq!(part_2(input), 204610);
}
