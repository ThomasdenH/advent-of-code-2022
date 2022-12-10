use std::collections::HashSet;

/// Denotes the relative position of the leading part of the rope.
#[derive(Default, Copy, Clone, Debug, Hash, Eq, PartialEq)]
struct RopePartPosition {
    horizontal_diff: i16,
    vertical_diff: i16,
}

impl RopePartPosition {
    /// Move this rope part towards the parent.
    fn move_towards_parent(&mut self, parent: &mut RopePartPosition) {
        // Map:
        // * 0, 1, -1 -> 0
        // * -2 -> -1
        // * 2 -> 1
        let mut dx = parent.horizontal_diff.signum();
        let mut dy = parent.vertical_diff.signum();
        if dx != parent.horizontal_diff || dy != parent.vertical_diff {
            // Too much distance, so move rope
            self.move_by_delta((dx, dy));
            parent.move_by_delta((-dx, -dy));
        }
    }

    /// Move this rope part by the provided amount.
    fn move_by_delta(&mut self, (dx, dy): (i16, i16)) {
        self.horizontal_diff += dx;
        self.vertical_diff += dy;
    }

    /// Move this rope one step in the given direction.
    fn move_in_direction(&mut self, direction: Direction) {
        match direction {
            Direction::Down => self.move_by_delta((0, -1)),
            Direction::Up => self.move_by_delta((0, 1)),
            Direction::Left => self.move_by_delta((-1, 0)),
            Direction::Right => self.move_by_delta((1, 0)),
        }
    }
}

/// A rope. The positions are of the head towards the tail. Except for the tail,
/// all positions are relative to their child.
struct Rope<const LENGTH: usize>([RopePartPosition; LENGTH]);

impl<const LENGTH: usize> Default for Rope<LENGTH> {
    fn default() -> Self {
        Rope([RopePartPosition::default(); LENGTH])
    }
}

impl<const LENGTH: usize> Rope<LENGTH> {
    /// Move this rope one step in the right direction.
    fn move_in_direction(&mut self, direction: Direction) {
        // Move the head in the right direction.
        self.0[0].move_in_direction(direction);
        // All other parts should follow
        for i in 1..LENGTH {
            let (a, b) = self.0.split_at_mut(i);
            let parent = a.last_mut().unwrap();
            let child = b.first_mut().unwrap();
            child.move_towards_parent(parent);
        }
    }

    fn tail_position(&self) -> RopePartPosition {
        self.0[LENGTH - 1]
    }
}

/// Grid direction.
#[derive(Copy, Clone, Debug)]
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
                    // The next byte is either a new line, then do nothing,
                    // or another (final) digit.
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

pub fn simulate_rope<const LENGTH: usize>(s: &str) -> usize {
    // If the rope is at least two long, we don't have to store the starting
    // position since the tail won't move on the first iteration.
    debug_assert!(LENGTH >= 2);
    let mut positions = HashSet::new();
    let mut rope = Rope::<LENGTH>::default();
    for (direction, distance) in parse_instructions(s) {
        for i in 0..distance {
            rope.move_in_direction(direction);
            positions.insert(rope.tail_position());
        }
    }
    positions.len()
}

pub fn part_1(s: &str) -> usize {
    simulate_rope::<2>(s)
}

pub fn part_2(s: &str) -> usize {
    simulate_rope::<10>(s)
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
fn test_part_2_example_1() {
    let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
    assert_eq!(part_2(input), 1);
}

#[test]
fn test_part_2_example_2() {
    let input = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
    assert_eq!(part_2(input), 36);
}

#[test]
fn test_part_1() {
    let input = include_str!("../input/2022/day9.txt");
    assert_eq!(part_1(input), 6384);
}

#[test]
fn test_part_2() {
    let input = include_str!("../input/2022/day9.txt");
    assert_eq!(part_2(input), 2734);
}
