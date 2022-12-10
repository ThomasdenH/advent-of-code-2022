fn all_distinct(window: &[u8]) -> bool {
    for i in 0..window.len() {
        for j in (i + 1)..window.len() {
            if window[i] == window[j] {
                return false;
            }
        }
    }
    true
}

fn find_marker(s: &str, size: usize) -> usize {
    s.as_bytes()
        .windows(size)
        .enumerate()
        .filter(|(_, window)| all_distinct(window))
        .next()
        .unwrap()
        .0
        + size
}

pub fn part_1(s: &str) -> usize {
    find_marker(s, 4)
}

pub fn part_2(s: &str) -> usize {
    find_marker(s, 14)
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
