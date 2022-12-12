use arrayvec::ArrayVec;

use crate::util;

#[derive(Copy, Clone, Debug)]
enum Operation {
    Add(u32),
    Mul(u32),
    Square,
}

impl Operation {
    fn apply(self, num: &mut u32) {
        match self {
            Operation::Add(a) => *num += a,
            Operation::Mul(a) => *num *= a,
            Operation::Square => *num *= *num,
        };
        *num /= 3;
    }
}

#[derive(Debug, Copy, Clone)]
struct Test(u32);

impl Test {
    fn test(self, num: u32) -> bool {
        num % self.0 == 0
    }
}

#[derive(Debug, Clone, Copy)]
struct Item(u32);

#[derive(Debug)]
struct Monkey {
    operation: Operation,
    test: Test,
    monkey_condition: [usize; 2],
}

const MAX_ITEMS: usize = 32;

fn parse_monkeys<const AMOUNT: usize>(s: &str) -> [(Monkey, ArrayVec<u32, MAX_ITEMS>); AMOUNT] {
    let mut bytes = s.as_bytes().iter().copied();
    core::array::from_fn::<_, AMOUNT, _>(move |_| {
        bytes.advance_by("Monkey _:\n".len());
        bytes.advance_by("  Starting items: ".len());
        let mut numbers = ArrayVec::new();
        loop {
            numbers.push(util::read_two_digit_number(&mut bytes).into());
            if bytes.next() == Some(b',') {
                bytes.next();
            } else {
                break;
            }
        }
        bytes.advance_by("  Operation: new = old ".len());
        let operation = match bytes.next().unwrap() {
            b'*' => {
                bytes.next();
                let mut num = bytes.next().unwrap();
                if num == b'o' {
                    bytes.advance_by("ld\n".len());
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
                let num = util::read_number_one_or_two_digits(&mut bytes);
                Operation::Add(num.into())
            }
            _ => unreachable!(),
        };
        bytes.advance_by("  Test: divisible by ".len());
        let test = Test(util::read_number_one_or_two_digits(&mut bytes).into());
        bytes.advance_by("    If true: throw to monkey ".len());
        let true_monkey = bytes.next().unwrap() & 0b1111;
        bytes.advance_by("\n    If false: throw to monkey ".len());
        let false_monkey = bytes.next().unwrap() & 0b1111;
        let monkey_condition = [false_monkey.into(), true_monkey.into()];
        bytes.next();
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

pub fn part_1(s: &str) -> usize {
    part_1_generic::<8>(s)
}

const PART_ONE_ROUNDS: usize = 20;

fn part_1_generic<const MONKEY_COUNT: usize>(s: &str) -> usize {
    let mut monkeys: [_; MONKEY_COUNT] = parse_monkeys(s);
    let mut inspections = [0usize; MONKEY_COUNT];
    for _ in 0..PART_ONE_ROUNDS {
        for i in 0..monkeys.len() {
            while let Some((monkey, mut item)) = {
                let (monkey, items) = monkeys.get_mut(i).unwrap();
                items.pop_at(0).map(|item| (monkey, item))
            } {
                inspections[i] += 1;
                monkey.operation.apply(&mut item);
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

#[test]
fn test_part_1_example() {
    let input = "Monkey 0:
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
    assert_eq!(part_1_generic::<4>(input), 10605);
}

#[test]
fn test_part_1() {
    let input = include_str!("../input/2022/day10.txt");
    assert_eq!(part_1(input), 51075);
}
