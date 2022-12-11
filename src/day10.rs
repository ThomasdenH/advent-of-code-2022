#[derive(Debug)]
enum Instruction {
    Noop,
    AddX(i8),
}

#[derive(Copy, Clone)]
struct Cpu {
    x: i8,
}

impl Default for Cpu {
    fn default() -> Self {
        Cpu { x: 1 }
    }
}

impl Cpu {
    /// Execute instruction. Returns how many instructions it took.
    fn execute_instruction(&mut self, inst: Instruction) -> usize {
        match inst {
            Instruction::Noop => 1,
            Instruction::AddX(a) => {
                self.x += a;
                2
            }
        }
    }

    fn execute_cycles(&mut self, inst: Instruction) -> impl Iterator<Item = Cpu> {
        let cpu = *self;
        let times = self.execute_instruction(inst);
        std::iter::repeat(cpu).take(times)
    }

    fn execute_all_cycles<'a>(
        &'a mut self,
        instructions: impl Iterator<Item = Instruction> + 'a,
    ) -> impl Iterator<Item = Cpu> + 'a {
        instructions
            .chain(std::iter::once(Instruction::Noop))
            .flat_map(|inst| self.execute_cycles(inst))
    }
}

/// Parse a number of 1 or 2 digits, and potentially a `-` sign.
fn parse_num(s: &[u8]) -> i8 {
    if s[0] == b'-' {
        -parse_num(&s[1..])
    } else {
        let mut num = s[0] & 0b1111;
        if let Some(other_digit) = s.get(1) {
            num *= 10;
            num += other_digit & 0b1111;
        }
        num as i8
    }
}

fn parse_instructions(s: &str) -> impl Iterator<Item = Instruction> + '_ {
    s.split_terminator('\n')
        .map(str::as_bytes)
        .map(|inst| match inst[0] {
            b'n' => Instruction::Noop,
            b'a' => Instruction::AddX(parse_num(&inst[5..])),
            _ => unreachable!(),
        })
}

pub fn part_1(s: &str) -> isize {
    Cpu::default()
        .execute_all_cycles(parse_instructions(s).inspect(|inst| {
            dbg!(inst);
        }))
        .map(|cpu| cpu.x)
        .enumerate()
        .skip(19)
        .step_by(40)
        .take(6)
        .map(|(cycle, val)| (cycle + 1) as isize * isize::from(val))
        .sum()
}

#[test]
fn test_small_example() {
    let input = "noop
addx 3
addx -5";
    let cycles: Vec<_> = Cpu::default()
        .execute_all_cycles(parse_instructions(input))
        .map(|cpu| cpu.x)
        .collect();
    assert_eq!(cycles, vec![1, 1, 1, 4, 4, -1])
}

#[test]
fn test_part_1_example() {
    let input = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
    assert_eq!(part_1(input), 13140);
}

#[test]
fn test_part_1() {
    let input = include_str!("../input/2022/day10.txt");
    assert_eq!(part_1(input), 13680);
}

/*
#[test]
fn test_part_2() {
    let input = include_str!("../input/2022/day9.txt");
    assert_eq!(part_2(input), 2734);
}
*/
