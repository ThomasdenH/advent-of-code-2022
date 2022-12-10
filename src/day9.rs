use std::collections::HashSet;

#[derive(Default, Hash, Eq, PartialEq, Copy, Clone, Debug)]
struct TailPosition(i16, i16);

impl TailPosition {
    fn move_to_head(&mut self, head_position: &mut HeadPosition, direction: Direction) {
        match direction {
            Direction::Down => {
                if head_position.vertical_diff == -1 {
                    // The head is already one down, so move the tail down.
                    // ...T... -> .......
                    // ...H... -> ...T...
                    // ....... -> ...H...
                    self.1 = self.1.wrapping_sub(1);
                    // If there is a horizontal difference, correct it.
                    // ..T.... -> .......
                    // ...H... -> ...T...
                    // ....... -> ...H...
                    self.0 = self.0.wrapping_add(head_position.horizontal_diff);
                    head_position.horizontal_diff = 0;
                } else {
                    head_position.vertical_diff = head_position.vertical_diff.wrapping_sub(1);
                }
            }
            Direction::Up => {
                if head_position.vertical_diff == 1 {
                    self.1 = self.1.wrapping_add(1);
                    self.0 = self.0.wrapping_add(head_position.horizontal_diff);
                    head_position.horizontal_diff = 0;
                } else {
                    head_position.vertical_diff = head_position.vertical_diff.wrapping_add(1);
                }
            }
            Direction::Left => {
                if head_position.horizontal_diff == -1 {
                    self.0 = self.0.wrapping_sub(1);
                    self.1 = self.1.wrapping_add(head_position.vertical_diff);
                    head_position.vertical_diff = 0;
                } else {
                    head_position.horizontal_diff = head_position.horizontal_diff.wrapping_sub(1);
                }
            }
            Direction::Right => {
                if head_position.horizontal_diff == 1 {
                    self.0 = self.0.wrapping_add(1);
                    self.1 = self.1.wrapping_add(head_position.vertical_diff);
                    head_position.vertical_diff = 0;
                } else {
                    head_position.horizontal_diff = head_position.horizontal_diff.wrapping_add(1);
                }
            }
        }
    }
}

/// Denotes the head position relative to the tail.
#[derive(Default, Copy, Clone, Debug)]
struct HeadPosition {
    horizontal_diff: i16,
    vertical_diff: i16,
}

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn parse_instructions(s: &str) -> impl Iterator<Item = (Direction, u8)> + '_ {
    let mut bytes = s.as_bytes().iter();
    std::iter::from_fn(move || {
        bytes
            .next()
            .map(|dir| match *dir {
                b'U' => Direction::Up,
                b'D' => Direction::Down,
                b'L' => Direction::Left,
                b'R' => Direction::Right,
                _ => unreachable!(),
            })
            .map(|direction| {
                // Skip space
                bytes.next();

                // Read first digit
                let mut num = bytes.next().unwrap() & 0b1111;
                if let Some(another_digit) = bytes.next() {
                    if *another_digit != b'\n' {
                        num *= 10;
                        num += another_digit & 0b1111;
                        // Skip line ending (if present)
                        bytes.next();
                    }
                }
                (direction, num)
            })
    })
}

pub fn part_1(s: &str) -> usize {
    let mut positions = HashSet::new();
    let mut tail_position = TailPosition::default();
    let mut relative_head_position = HeadPosition::default();
    for (direction, distance) in parse_instructions(s) {
        for _ in 0..distance {
            debug_assert!(relative_head_position.vertical_diff.abs() <= 1 && relative_head_position.horizontal_diff.abs() <= 1);
            positions.insert(tail_position);
            tail_position.move_to_head(&mut relative_head_position, direction);
        }
    }
    positions.insert(tail_position);
    positions.len()
}

#[test]
fn test_part_1_example() {
    let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
    assert_eq!(part_1(input), 13);
}

#[test]
fn test_part_1() {
    let input = include_str!("../input/2022/day9.txt");
    assert_eq!(part_1(input), 6384);
}
/*
#[test]
fn test_part_2() {
    let input = include_str!("../input/2022/day9.txt");
    assert_eq!(part_2(input), );
}
*/
