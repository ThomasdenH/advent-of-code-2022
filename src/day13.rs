use std::cmp::Ordering;

fn compare(a: &[u8], b: &[u8]) -> Ordering {
    let mut it_a = a.iter().copied().peekable();
    let mut it_b = b.iter().copied().peekable();
    let mut bracket_difference: i8 = 0;
    let mut first_of_list = true;
    loop {
        dbg!((it_a.peek().copied().map(char::from), it_b.peek().copied().map(char::from)));
        match (it_a.peek().copied(), it_b.peek().copied()) {
            (Some(b']'), Some(b']')) => {
                it_a.next();
                it_b.next();
            }
            (Some(b']'), _) => {
                // Close in first, not in second. This is allowed as long as the first was opened more.
                if bracket_difference >= 0 {
                    return Ordering::Less;
                }
                it_a.next();
                bracket_difference += 1;
            }
            (_, Some(b']')) => {
                if bracket_difference <= 0 {
                    return Ordering::Greater;
                }
                it_b.next();
                bracket_difference -= 1;
            }
            (Some(b'['), Some(b'[')) => {
                it_a.next();
                it_b.next();
                first_of_list = true;
            }
            (Some(b','), Some(b',')) => {
                it_a.next();
                it_b.next();
                first_of_list = false;
            }
            (Some(b'['), _) => {
                it_a.next();
                bracket_difference -= 1;
                first_of_list = true;
            }
            (_, Some(b'[')) => {
                it_b.next();
                bracket_difference += 1;
                first_of_list = true;
            }
            (Some(next_a), Some(next_b)) => {
                debug_assert!(next_a.is_ascii_digit());
                debug_assert!(next_b.is_ascii_digit());

                // If we are reading two numbers, but we were emulating a list
                // and this is not the first number in it, the list is closed
                // now, meaning that the side that emulated the list is
                // smaller.
                if !first_of_list && bracket_difference != 0 {
                    return 0.cmp(&bracket_difference);
                }
                // We know the first digits, now read the rest
                it_a.next();
                it_b.next();
                match (it_a.peek().unwrap(), it_b.peek().unwrap()) {
                    (b'0', b'0') => {
                        it_a.next();
                        it_b.next();
                    } // Both 10
                    (b'0', _) => return Ordering::Greater,
                    (_, b'0') => return Ordering::Less,
                    (_, _) => {
                        if let order @ (Ordering::Greater | Ordering::Less) = next_a.cmp(&next_b) {
                            return order;
                        }
                    }
                }
            }
            (None, a) | (a, None) => {
                // If both lists end, they must have been equal, which doesn't happen.
                // If one lists ends and the other doesn't, that means that we
                // should have labeled the shorter list as `Less` already.
                unreachable!()
            }
        }
    }
}

#[test]
fn test_number_order() {
    assert_eq!(compare(b"[0]", b"[1]"), Ordering::Less);
    assert_eq!(compare(b"[3]", b"[5]"), Ordering::Less);
    assert_eq!(compare(b"[8]", b"[7]"), Ordering::Greater);
    assert_eq!(compare(b"[10]", b"[1]"), Ordering::Greater);
    assert_eq!(compare(b"[10]", b"[9]"), Ordering::Greater);
}

#[test]
fn test_simple_list_length_compare() {
    assert_eq!(compare(b"[0,0,0]", b"[0,0,0,0]"), Ordering::Less);
    assert_eq!(compare(b"[0,0,0,0,0,0,0]", b"[0,0,0,0]"), Ordering::Greater);
}

#[test]
fn test_nested_list() {
    assert_eq!(compare(b"[[0]]", b"[1]"), Ordering::Less);
    assert_eq!(compare(b"[[1]]", b"[0]"), Ordering::Greater);
    assert_eq!(compare(b"[[[[5]]]]", b"[7]"), Ordering::Less);
    assert_eq!(compare(b"[[[[5,8]]]]", b"[7]"), Ordering::Less);
    assert_eq!(compare(b"[[1,2]]", b"[1,2]"), Ordering::Greater);
}

#[test]
fn test_part_1_examples() {
    assert_eq!(compare(b"[1,1,3,1,1]", b"[1,1,5,1,1]"), Ordering::Less);
    assert_eq!(compare(b"[[1],[2,3,4]]", b"[[1],4]"), Ordering::Less);
    assert_eq!(compare(b"[9]", b"[[8,7,6]]"), Ordering::Greater);
    assert_eq!(compare(b"[[4,4],4,4]", b"[[4,4],4,4,4]"), Ordering::Less);
    assert_eq!(compare(b"[7,7,7,7]", b"[7,7,7]"), Ordering::Greater);
    assert_eq!(compare(b"[]", b"[3]"), Ordering::Less);
    assert_eq!(compare(b"[[[]]]", b"[[]]"), Ordering::Greater);
    assert_eq!(compare(b"[1,[2,[3,[4,[5,6,7]]]],8,9]", b"[1,[2,[3,[4,[5,6,0]]]],8,9]"), Ordering::Greater);
}
