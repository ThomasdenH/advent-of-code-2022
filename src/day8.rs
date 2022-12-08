pub fn part_1_generic<const WIDTH: usize, const HEIGHT: usize, const TOTAL_SIZE: usize>(s: &str) -> usize {
    let trees = s.as_bytes();
    assert!(trees.len() >= WIDTH * HEIGHT);
    debug_assert!(TOTAL_SIZE == (WIDTH + 1) * HEIGHT);
    let mut marked = [false; TOTAL_SIZE];

    let mut test_and_mark = |x: usize, y: usize, current_smallest: &mut u8| {
        let idx = x + y * (WIDTH + 1);
        let tree = trees[idx];
        if tree > *current_smallest {
            marked[idx] = true;
            *current_smallest = tree;
        }
    };

    for x in 0..WIDTH {
        let mut current_smallest = b'0' - 1;
        for y in (0..HEIGHT).rev() {
            test_and_mark(x, y, &mut current_smallest);
        }
        let mut current_smallest = b'0' - 1;
        for y in 0..HEIGHT {
            test_and_mark(x, y, &mut current_smallest);
        }
    }

    for y in 0..HEIGHT {
        let mut current_smallest = b'0' - 1;
        for x in 0..WIDTH {
            test_and_mark(x, y, &mut current_smallest);
        }
        let mut current_smallest = b'0' - 1;
        for x in (0..WIDTH).rev() {
            test_and_mark(x, y, &mut current_smallest);
        }
    }
    marked.iter().filter(|b| **b).count()
}

enum Direction {
    IncreasingY,
    IncreasingX,
    DecreasingY,
    DecreasingX
}

use Direction::*;

pub fn part_2_generic<const WIDTH: u8, const HEIGHT: u8, const TOTAL_SIZE: usize>(s: &str) -> u32 {
    let to_index = |x: u8, y: u8| usize::from(x) + usize::from(y) * (usize::from(WIDTH) + 1);

    let trees = s.as_bytes();

    // Set Indexed as x + y * (WIDTH + 1), and then through direction.
    let mut scenic_score = [1u32; TOTAL_SIZE];

    let mut test_and_mark = |x: u8, y: u8, direction: Direction, last_encounter_of_tree_of_at_least_height: &mut [u8; 10]| {
        let idx = to_index(x, y);
        let tree = usize::from(trees[idx] & 0b1111);
        let (view, direction_coordinate) = match direction {
            DecreasingX => (last_encounter_of_tree_of_at_least_height[tree] - x, x),
            IncreasingX => (x - last_encounter_of_tree_of_at_least_height[tree], x),
            DecreasingY => (last_encounter_of_tree_of_at_least_height[tree] - y, y),
            IncreasingY => (y - last_encounter_of_tree_of_at_least_height[tree], y),
        };
        scenic_score[idx] *= u32::from(view);
        for height in 0..=tree {
            last_encounter_of_tree_of_at_least_height[height] = direction_coordinate;
        }
    };

    for x in 0..WIDTH {
        let mut last_encounter_of_tree_of_at_least_height = [0; 10];
        for y in (0..HEIGHT).rev() {
            test_and_mark(x, y, DecreasingY, &mut last_encounter_of_tree_of_at_least_height);
        }
        let mut last_encounter_of_tree_of_at_least_height = [0; 10];
        for y in 0..HEIGHT {
            test_and_mark(x, y, IncreasingY, &mut last_encounter_of_tree_of_at_least_height);
        }
    }

    for y in 0..HEIGHT {
        let mut last_encounter_of_tree_of_at_least_height = [0; 10];
        for x in 0..WIDTH {
            test_and_mark(x, y, IncreasingX, &mut last_encounter_of_tree_of_at_least_height);
        }
        let mut last_encounter_of_tree_of_at_least_height = [0; 10];
        for x in (0..WIDTH).rev() {
            test_and_mark(x, y, DecreasingX, &mut last_encounter_of_tree_of_at_least_height);
        }
    }
    scenic_score.iter().copied().max().unwrap_or_default()
}

pub fn part_1(s: &str) -> usize {
    part_1_generic::<99, 99, 9_900>(s)
}

pub fn part_2(s: &str) -> u32 {
    part_2_generic::<99, 99, 9_900>(s)
}

#[test]
fn test_part_1_example() {
    let input = "30373
25512
65332
33549
35390";
    assert_eq!(part_1_generic::<5, 5, 30>(input), 21);
}

#[test]
fn test_part_2_example() {
    let input = "30373
25512
65332
33549
35390";
    assert_eq!(part_2_generic::<5, 5, 30>(input), 8);
}

#[test]
fn test_part_1() {
    let input = include_str!("../input/2022/day8.txt");
    assert_eq!(part_1(input), 1814);
}

#[test]
fn test_part_2() {
    let input = include_str!("../input/2022/day8.txt");
    assert_eq!(part_2(input), 330786);
}
