fn all_distinct<const SIZE: usize>(window: &[u8; SIZE]) -> bool {
    for i in 0..SIZE {
        for j in (i + 1)..SIZE {
            if window[i] == window[j] {
                return false;
            }
        }
    }
    true
}

fn find_marker<const SIZE: usize>(s: &str) -> usize {
    s.as_bytes()
        .array_windows::<SIZE>()
        .enumerate()
        .find(|(_, window)| all_distinct(window))
        .unwrap()
        .0
        + SIZE
}

pub fn part_1(s: &str) -> usize {
    find_marker::<4>(s)
}

pub fn part_2(s: &str) -> usize {
    find_marker::<14>(s)
}

#[test]
fn test_part_1_example() {
    assert_eq!(part_1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
    assert_eq!(part_1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
    assert_eq!(part_1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
    assert_eq!(part_1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
}

#[test]
fn test_part_1_extra_example() {
    assert_eq!(part_1("abcd"), 4);
    assert_eq!(part_1("lrgrvgvttzmtmtgglmgk"), 20);
}

#[test]
fn test_part_2_example() {
    assert_eq!(part_2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
    assert_eq!(part_2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
    assert_eq!(part_2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
    assert_eq!(part_2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
    assert_eq!(part_2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
}

#[test]
fn test_part_1() {
    let input = include_str!("../input/2022/day6.txt");
    assert_eq!(part_1(input), 1804);
}

#[test]
fn test_part_2() {
    let input = include_str!("../input/2022/day6.txt");
    assert_eq!(part_2(input), 2508);
}
