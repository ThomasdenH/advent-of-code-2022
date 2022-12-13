#[derive(Copy, Clone, Debug)]
enum Operation {
    Add(u64),
    Mul(u64),
    Square,
}

impl Operation {
    fn apply_with_division(self, num: &mut u64) {
        match self {
            Operation::Add(a) => *num += a,
            Operation::Mul(a) => *num *= a,
            Operation::Square => *num *= *num,
        };
        *num /= 3;
    }

    fn apply(self, num: &mut u64) {
        match self {
            Operation::Add(a) => *num += a,
            Operation::Mul(a) => *num *= a,
            Operation::Square => *num *= *num,
        };
    }
}

#[derive(Debug, Copy, Clone)]
struct Test(u64);

impl Test {
    fn test(self, num: u64) -> bool {
        num % self.0 == 0
    }
}

#[derive(Debug, Clone, Copy)]
struct Item(u64);

#[derive(Debug)]
struct Monkey {
    operation: Operation,
    test: Test,
    monkey_condition: [usize; 2],
}

const MAX_ITEMS: usize = 32;

mod parse {
    use arrayvec::ArrayVec;

    use crate::util;

    use super::{Monkey, Operation, Test, MAX_ITEMS};

    fn read_monkey_line(bytes: &mut impl Iterator<Item = u8>) {
        bytes.advance_by("Monkey _:\n".len()).unwrap();
    }

    fn read_starting_items(bytes: &mut impl Iterator<Item = u8>) -> ArrayVec<u64, MAX_ITEMS> {
        bytes.advance_by("  Starting items: ".len()).unwrap();
        let mut numbers = ArrayVec::new();
        loop {
            numbers.push(util::read_two_digit_number(bytes).into());
            if bytes.next() == Some(b',') {
                bytes.next();
            } else {
                break;
            }
        }
        numbers
    }

    fn read_operation(bytes: &mut impl Iterator<Item = u8>) -> Operation {
        bytes.advance_by("  Operation: new = old ".len()).unwrap();
        match bytes.next().unwrap() {
            b'*' => {
                bytes.next();
                let mut num = bytes.next().unwrap();
                if num == b'o' {
                    bytes.advance_by("ld\n".len()).unwrap();
                    Operation::Square
                } else {
                    num &= 0b1111;
                    let next = bytes.next().unwrap();
                    if next != b'\n' {
                        num *= 10;
                        num += next & 0b1111;
                        bytes.next();
                    }
                    Operation::Mul(num.into())
                }
            }
            b'+' => {
                bytes.next();
                let num = util::read_number_one_or_two_digits(bytes);
                Operation::Add(num.into())
            }
            _ => unreachable!(),
        }
    }

    fn read_test(bytes: &mut impl Iterator<Item = u8>) -> (Test, [usize; 2]) {
        bytes.advance_by("  Test: divisible by ".len()).unwrap();
        let test = Test(util::read_number_one_or_two_digits(bytes).into());
        bytes
            .advance_by("    If true: throw to monkey ".len())
            .unwrap();
        let true_monkey = bytes.next().unwrap() & 0b1111;
        bytes
            .advance_by("\n    If false: throw to monkey ".len())
            .unwrap();
        let false_monkey = bytes.next().unwrap() & 0b1111;
        let monkey_condition = [false_monkey.into(), true_monkey.into()];
        bytes.next();
        (test, monkey_condition)
    }

    pub(super) fn parse_monkeys<const AMOUNT: usize>(
        s: &str,
    ) -> [(Monkey, ArrayVec<u64, MAX_ITEMS>); AMOUNT] {
        let mut bytes = s.as_bytes().iter().copied();
        core::array::from_fn::<_, AMOUNT, _>(move |_| {
            read_monkey_line(&mut bytes);
            let numbers = read_starting_items(&mut bytes);
            let operation = read_operation(&mut bytes);
            let (test, monkey_condition) = read_test(&mut bytes);
            bytes.next();
            (
                Monkey {
                    monkey_condition,
                    operation,
                    test,
                },
                numbers,
            )
        })
    }
}

pub fn part_1(s: &str) -> usize {
    part_1_generic::<8>(s)
}

pub fn part_2(s: &str) -> usize {
    part_2_generic::<8>(s)
}

const PART_ONE_ROUNDS: usize = 20;
const PART_TWO_ROUNDS: usize = 10_000;

fn part_1_generic<const MONKEY_COUNT: usize>(s: &str) -> usize {
    let mut monkeys: [_; MONKEY_COUNT] = parse::parse_monkeys(s);
    let mut inspections = [0usize; MONKEY_COUNT];
    for _ in 0..PART_ONE_ROUNDS {
        for (monkey_index, inspection_count) in
            inspections.iter_mut().enumerate().take(monkeys.len())
        {
            while let Some((monkey, mut item)) = {
                let (monkey, items) = monkeys.get_mut(monkey_index).unwrap();
                items.pop_at(0).map(|item| (monkey, item))
            } {
                *inspection_count += 1;
                monkey.operation.apply_with_division(&mut item);
                if monkey.test.test(item) {
                    monkeys[monkey.monkey_condition[1]].1.push(item);
                } else {
                    monkeys[monkey.monkey_condition[0]].1.push(item);
                }
            }
        }
    }
    let mut max_inspections = [0usize; 2];
    for mut monkey_inspection in inspections {
        for existing_max in max_inspections.iter_mut() {
            if *existing_max < monkey_inspection {
                std::mem::swap(existing_max, &mut monkey_inspection);
            }
        }
    }
    max_inspections.iter().product()
}

fn part_2_generic<const MONKEY_COUNT: usize>(s: &str) -> usize {
    let mut monkeys: [_; MONKEY_COUNT] = parse::parse_monkeys(s);
    let mut inspections = [0usize; MONKEY_COUNT];
    let common_multiple: u64 = monkeys.iter().map(|monkey| monkey.0.test.0).product();
    for _ in 0..PART_TWO_ROUNDS {
        for (monkey_index, inspection_count) in
            inspections.iter_mut().enumerate().take(monkeys.len())
        {
            while let Some((monkey, mut item)) = {
                let (monkey, items) = monkeys.get_mut(monkey_index).unwrap();
                items.pop_at(0).map(|item| (monkey, item))
            } {
                *inspection_count += 1;
                monkey.operation.apply(&mut item);
                item %= common_multiple;
                if monkey.test.test(item) {
                    monkeys[monkey.monkey_condition[1]].1.push(item);
                } else {
                    monkeys[monkey.monkey_condition[0]].1.push(item);
                }
            }
        }
    }
    let mut max_inspections = [0usize; 2];
    for mut monkey_inspection in inspections {
        for existing_max in max_inspections.iter_mut() {
            if *existing_max < monkey_inspection {
                std::mem::swap(existing_max, &mut monkey_inspection);
            }
        }
    }
    max_inspections.iter().product()
}

#[cfg(test)]
const EXAMPLE_INPUT: &str = "\
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

#[test]
fn test_part_1_example() {
    assert_eq!(part_1_generic::<4>(EXAMPLE_INPUT), 10605);
}

#[test]
fn test_part_2_example() {
    assert_eq!(part_2_generic::<4>(EXAMPLE_INPUT), 2713310158);
}

#[test]
fn test_part_1() {
    let input = include_str!("../input/2022/day11.txt");
    assert_eq!(part_1(input), 51075);
}

#[test]
fn test_part_2() {
    let input = include_str!("../input/2022/day11.txt");
    assert_eq!(part_2(input), 11741456163);
}
