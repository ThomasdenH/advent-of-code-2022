pub fn parse_numbers(s: &str) -> impl Iterator<Item = Option<usize>> + '_ {
    let mut iter = s.as_bytes().iter().copied();
    std::iter::from_fn(move || {
        iter.next().map(|b| {
            if b == b'\n' {
                None
            } else {
                Some(
                    (&mut iter)
                        .take_while(|b| *b != b'\n')
                        .fold(usize::from(b - b'0'), |acc, digit| {
                            acc * 10 + usize::from(digit - b'0')
                        }),
                )
            }
        })
    })
}

pub fn part_1(input: &str) -> usize {
    let numbers = parse_numbers(input);
    let mut max = 0;
    let mut current = 0;
    for d in numbers {
        if let Some(a) = d {
            current += a;
        } else {
            max = max.max(current);
            current = 0;
        }
    }
    max
}

pub fn part_2(input: &str) -> usize {
    let mut numbers = parse_numbers(input);
    // Take sum of numbers up to `None`
    std::iter::from_fn(|| numbers.next()
        .map(|first_number| first_number + (&mut numbers).take_while(|b| b).sum()))
        .fold([0usize; 3], |mut acc: [usize; 3], mut new| {
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
