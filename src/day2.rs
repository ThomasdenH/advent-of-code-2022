#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Play([u8; 2]);

impl Play {
    const fn points(&self) -> u8 {
        const DIFF: u8 = b'A'
            .wrapping_sub(b'X')
            .wrapping_add(1)
            .wrapping_mul(3)
            .wrapping_sub(b'X')
            .wrapping_add(1);
        let [a, b] = self.0;
        let points = b
            .wrapping_sub(a)
            .wrapping_mul(3)
            .wrapping_add(b)
            .wrapping_add(DIFF);
        if (points as i8).is_negative() {
            points.wrapping_add(9)
        } else if points > 9 {
            points.wrapping_sub(9)
        } else {
            points
        }
    }

    #[cfg(test)]
    fn points_explicit(&self) -> u8 {
        let [a, b] = self.0;
        let points = ((b - b'X') + 10 - (a - b'A')) % 3;
        points * 3 + (b - b'X' + 1)
    }
}

fn plays(input: &str) -> impl Iterator<Item = Play> + '_ {
    input.bytes().step_by(2).array_chunks().map(|arr| Play(arr))
}

pub fn part_1(input: &str) -> usize {
    plays(input).map(|a| a.points()).map(usize::from).sum()
}

#[test]
fn test_parse() {
    let input = "A Y
B X
C Z";
    assert_eq!(
        plays(input).collect::<Vec<_>>(),
        vec![Play([b'A', b'Y']), Play([b'B', b'X']), Play([b'C', b'Z'])]
    );
}

#[test]
fn test_points_equivalence() {
    for a in b'A'..=b'C' {
        for b in b'X'..=b'Z' {
            let play = Play([a, b]);
            assert_eq!(play.points_explicit(), play.points());
        }
    }
}

#[test]
fn test_points() {
    assert_eq!(Play([b'A', b'Y']).points(), 8);
    assert_eq!(Play([b'B', b'X']).points(), 1);
    assert_eq!(Play([b'C', b'Z']).points(), 6);
}

#[test]
fn test_example_part_1() {
    let input = "A Y
B X
C Z";
    assert_eq!(plays(input).map(|play| play.points()).sum::<u8>(), 15u8);
}

#[test]
fn test_part_1() {
    let input = include_str!("../input/2022/day2.txt");
    assert_eq!(part_1(input), 10816);
}
