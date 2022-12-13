use std::fmt::Display;

const CRT_SIZE: usize = 240;
pub struct Crt([bool; CRT_SIZE]);

impl Display for Crt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..6 {
            for x in 0..40 {
                if self.0[x + y * 40] {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<I: Iterator<Item = bool>> From<I> for Crt {
    fn from(iter: I) -> Self {
        let mut arr = [false; CRT_SIZE];
        for (arr_entry, val) in arr.iter_mut().zip(iter) {
            *arr_entry = val;
        }
        Crt(arr)
    }
}

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
    fn register(self) -> i8 {
        self.x
    }

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
        .execute_all_cycles(parse_instructions(s))
        .map(Cpu::register)
        .enumerate()
        .skip(19)
        .step_by(40)
        .take(6)
        .map(|(cycle, val)| (cycle + 1) as isize * isize::from(val))
        .sum()
}

pub fn part_2(s: &str) -> Crt {
    Cpu::default()
        .execute_all_cycles(parse_instructions(s))
        .map(Cpu::register)
        .zip((0i8..40).cycle())
        .map(|(value, crt_index)| value <= crt_index + 1 && value + 1 >= crt_index)
        .into()
}

#[test]
fn test_small_example() {
    let input = "noop
addx 3
addx -5";
    let cycles: Vec<_> = Cpu::default()
        .execute_all_cycles(parse_instructions(input))
        .map(Cpu::register)
        .collect();
    assert_eq!(cycles, vec![1, 1, 1, 4, 4, -1])
}

#[cfg(test)]
const EXAMPLE_INPUT: &str = "addx 15
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

#[test]
fn test_part_1_example() {
    assert_eq!(part_1(EXAMPLE_INPUT), 13140);
}

#[test]
fn test_part_2_example() {
    assert_eq!(
        part_2(EXAMPLE_INPUT).to_string(),
        "\
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"
    );
}

#[test]
fn test_part_1() {
    let input = include_str!("../input/2022/day10.txt");
    assert_eq!(part_1(input), 13680);
}

#[test]
fn test_part_2() {
    let input = include_str!("../input/2022/day10.txt");
    assert_eq!(
        part_2(input).to_string(),
        "\
###..####..##..###..#..#.###..####.###..
#..#....#.#..#.#..#.#.#..#..#.#....#..#.
#..#...#..#....#..#.##...#..#.###..###..
###...#...#.##.###..#.#..###..#....#..#.
#....#....#..#.#....#.#..#....#....#..#.
#....####..###.#....#..#.#....####.###..
"
    );
}
