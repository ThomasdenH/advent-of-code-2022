use std::{cmp::Ordering, collections::BinaryHeap};

fn directions<const WIDTH: usize>() -> impl Iterator<Item = isize> {
    use std::iter::once;
    once(1isize)
        .chain(once(-1))
        .chain(once(WIDTH as isize))
        .chain(once(-(WIDTH as isize)))
}

#[derive(Eq, PartialEq, Copy, Clone)]
struct HeapEntry {
    pos: usize,
    score: usize,
    value: u8,
}

impl Ord for HeapEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        other.score.cmp(&self.score)
    }
}

impl PartialOrd for HeapEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn is_walk_possible(from: u8, to: u8) -> bool {
    // We can walk up to one higher and up to 25 lower.
    (from).wrapping_sub(to).wrapping_add(1) < 27
}

fn breadth_first_search<const WIDTH: usize, const TOTAL_SIZE: usize>(
    s: &str,
    starting_positions: impl Iterator<Item = usize>,
) -> usize {
    let grid = s.as_bytes();
    assert_eq!(grid.len(), TOTAL_SIZE);
    let mut visited = [false; TOTAL_SIZE];
    let mut heap: BinaryHeap<_> = starting_positions
        .map(|pos| HeapEntry {
            pos,
            score: 0,
            value: b'a',
        })
        .collect();
    while let Some(HeapEntry { pos, score, value }) = heap.pop() {
        if visited[pos] {
            continue;
        }
        visited[pos] = true;
        for direction in directions::<WIDTH>() {
            // We don't have to worry about left and right edges as we will
            // reach '\n' first. The top edge will wrap around and the bottom
            // edge lays outside of the array.
            let new_pos = pos.wrapping_add(direction as usize);
            if let Some(&to_value) = grid.get(new_pos) {
                if !visited[new_pos] {
                    if is_walk_possible(value, to_value) {
                        heap.push(HeapEntry {
                            pos: new_pos,
                            score: score + 1,
                            value: to_value,
                        })
                    } else if to_value == b'E' && value >= b'y' {
                        return score + 1;
                    }
                }
            }
        }
    }
    unreachable!()
}

pub fn part_1_generic<const WIDTH: usize, const TOTAL_SIZE: usize>(s: &str) -> usize {
    breadth_first_search::<WIDTH, TOTAL_SIZE>(s, memchr::memchr(b'S', s.as_bytes()).iter().copied())
}

pub fn part_2_generic<const WIDTH: usize, const TOTAL_SIZE: usize>(s: &str) -> usize {
    breadth_first_search::<WIDTH, TOTAL_SIZE>(s, memchr::memchr2_iter(b'S', b'a', s.as_bytes()))
}

pub fn part_1(s: &str) -> usize {
    part_1_generic::<162, 6641>(s)
}

pub fn part_2(s: &str) -> usize {
    part_2_generic::<162, 6641>(s)
}

#[test]
fn test_is_walk_possible() {
    for a in b'a'..=b'z' {
        for b in b'a'..=(a + 1).min(b'z') {
            assert!(is_walk_possible(a, b));
        }
        for b in (a + 2)..=b'z' {
            assert!(!is_walk_possible(a, b));
        }
        assert!(!is_walk_possible(a, b'\n'));
        assert!(!is_walk_possible(a, b'E'));
    }
}

#[test]
fn test_part_1_example() {
    let input = "\
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
    assert_eq!(part_1_generic::<9, 44>(input), 31);
}

#[test]
fn test_part_2_example() {
    let input = "\
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
    assert_eq!(part_2_generic::<9, 44>(input), 29);
}

#[test]
fn test_part_1() {
    let input = include_str!("../input/2022/day12.txt");
    assert_eq!(part_1(input), 481);
}

#[test]
fn test_part_2() {
    let input = include_str!("../input/2022/day12.txt");
    assert_eq!(part_2(input), 480);
}
