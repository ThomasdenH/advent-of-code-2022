#[derive(Copy, Clone, Debug)]
struct Index<const WIDTH: usize>(usize);

impl<const WIDTH: usize> Index<WIDTH> {
    fn from_coords(x: usize, y: usize) -> Self {
        Index(x + y * (WIDTH + 1))
    }

    fn move_left(&mut self) {
        self.0 -= 1;
    }

    fn move_right(&mut self) {
        self.0 += 1;
    }

    fn move_up(&mut self) {
        self.0 -= WIDTH + 1;
    }

    fn move_down(&mut self) {
        self.0 += WIDTH + 1;
    }
}

pub fn part_1_generic<const WIDTH: usize, const HEIGHT: usize, const TOTAL_SIZE: usize>(
    s: &str,
) -> usize {
    let trees = s.as_bytes();
    assert!(trees.len() >= WIDTH * HEIGHT);
    debug_assert!(TOTAL_SIZE == (WIDTH + 1) * HEIGHT);
    let mut marked = [false; TOTAL_SIZE];
    let mut test_and_mark = |index: Index<WIDTH>, current_smallest: &mut u8| {
        let tree = trees[index.0];
        if tree > *current_smallest {
            marked[index.0] = true;
            *current_smallest = tree;
        }
    };

    for x in 0..WIDTH {
        let mut current_smallest = b'0' - 1;
        let mut index = Index::from_coords(x, 0);
        for _ in 0..HEIGHT {
            test_and_mark(index, &mut current_smallest);
            index.move_down();
        }
        // The index is now one too far down.
        let mut current_smallest = b'0' - 1;
        for _ in 0..HEIGHT {
            index.move_up();
            test_and_mark(index, &mut current_smallest);
        }
    }

    for y in 0..HEIGHT {
        let mut current_smallest = b'0' - 1;
        let mut index = Index::from_coords(0, y);
        for _ in 0..WIDTH {
            test_and_mark(index, &mut current_smallest);
            index.move_right();
        }
        // The index is now one too far to the right
        let mut current_smallest = b'0' - 1;
        for _ in 0..WIDTH {
            index.move_left();
            test_and_mark(index, &mut current_smallest);
        }
    }
    marked.iter().filter(|b| **b).count()
}

pub fn part_2_generic<const WIDTH: usize, const HEIGHT: usize, const TOTAL_SIZE: usize>(
    s: &str,
) -> usize {
    let trees = s.as_bytes();

    // Set indexed like the trees.
    let mut scenic_score = [1usize; TOTAL_SIZE];

    let mut test_and_mark =
        |index: Index<WIDTH>,
         position_along_line: usize,
         last_encounter_of_tree_of_at_least_height: &mut [usize; 10]| {
            let tree = usize::from(trees[index.0] & 0b1111);
            // Compute the view
            let view = position_along_line - last_encounter_of_tree_of_at_least_height[tree];
            scenic_score[index.0] *= view;
            for last_encounter_of_tree in last_encounter_of_tree_of_at_least_height
                .iter_mut()
                .take(tree + 1)
            {
                *last_encounter_of_tree = position_along_line;
            }
        };

    for x in 0..WIDTH {
        let mut index = Index::from_coords(x, 0);
        let mut last_encounter_of_tree_of_at_least_height = [0; 10];
        for y in 0..HEIGHT {
            test_and_mark(index, y, &mut last_encounter_of_tree_of_at_least_height);
            index.move_down();
        }
        let mut last_encounter_of_tree_of_at_least_height = [0; 10];
        for y in 0..HEIGHT {
            index.move_up();
            test_and_mark(index, y, &mut last_encounter_of_tree_of_at_least_height);
        }
    }

    for y in 0..HEIGHT {
        let mut index = Index::from_coords(0, y);
        let mut last_encounter_of_tree_of_at_least_height = [0; 10];
        for x in 0..WIDTH {
            test_and_mark(index, x, &mut last_encounter_of_tree_of_at_least_height);
            index.move_right();
        }
        let mut last_encounter_of_tree_of_at_least_height = [0; 10];
        for x in 0..WIDTH {
            index.move_left();
            test_and_mark(index, x, &mut last_encounter_of_tree_of_at_least_height);
        }
    }
    scenic_score.iter().copied().max().unwrap_or_default()
}

pub fn part_1(s: &str) -> usize {
    part_1_generic::<99, 99, 9_900>(s)
}

pub fn part_2(s: &str) -> usize {
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
