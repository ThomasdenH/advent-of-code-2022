use primitive_types::U256;

pub struct PrintableArray<const SIZE: usize>([u8; SIZE]);

impl<const SIZE: usize> std::fmt::Display for PrintableArray<SIZE> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for c in self.0.into_iter() {
            write!(f, "{}", char::from(c))?;
        }
        Ok(())
    }
}

#[derive(Clone)]
struct Warehouse<const STACKS: usize>([U256; STACKS]);

impl<const STACKS: usize> std::fmt::Debug for Warehouse<STACKS> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut warehouse = self.clone();
        for stack in 0..STACKS {
            loop {
                let letter = warehouse.pop_crate(stack);
                if letter == 0 {
                    break;
                }
                write!(f, "{} ", char::from(0b0100_0000 | letter))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<const STACKS: usize> Warehouse<STACKS> {
    fn parse<'a>(lines: &mut impl Iterator<Item = &'a [u8]>) -> Self {
        let mut warehouse = Warehouse([U256::zero(); STACKS]);
        for line in lines.take_while(|s| s[1] != b'1') {
            for (letter, dest) in line
                .into_iter()
                .skip(1)
                .step_by(4)
                .map(|ascii| ascii & 0b1_1111)
                .zip(warehouse.0.iter_mut())
                .filter(|(letter, _)| *letter != 0)
            {
                *dest = (*dest << 5) | U256::from(letter);
            }
        }
        warehouse.invert_stacks();
        warehouse
    }

    fn invert_stacks(&mut self) {
        let mut new_warehouse = Warehouse([U256::zero(); STACKS]);
        for i in 0..STACKS {
            loop {
                let letter = self.pop_crate(i);
                if letter == 0 {
                    break;
                }
                new_warehouse.push_crate(i, letter);
            }
        }
        *self = new_warehouse;
    }

    fn pop_crate(&mut self, from: usize) -> u8 {
        let letter = self.0[from] & U256::from(0b1_1111);
        self.0[from] >>= 5;
        letter.as_u32() as u8
    }

    fn push_crate(&mut self, to: usize, letter: u8) {
        debug_assert!((1..=26).contains(&letter), "pushing invalid letter!");
        self.0[to] = (self.0[to] << 5) | U256::from(letter);
    }

    fn move_crate(&mut self, from: usize, to: usize) {
        let letter = self.pop_crate(from);
        self.push_crate(to, letter);
    }

    fn move_multiple_crates(&mut self, count: u8, from: usize, to: usize) {
        let bits = 5 * u32::from(count);
        let mask = U256::MAX >> (256 - bits);
        let letters = self.0[from] & mask;
        self.0[from] >>= bits;
        self.0[to] <<= bits;
        self.0[to] |= letters;
    }
}

fn parse_number(s: &mut &[u8]) -> u8 {
    let mut acc = s[0] & 0b1111;
    *s = &s[1..];
    if s[0] & 0b0001_0000 != 0 {
        acc = acc * 10 + (s[0] & 0b1111);
        *s = &s[1..];
    }
    acc
}

pub fn part_1(s: &str) -> PrintableArray<9> {
    solve_generic::<9, true>(s)
}

pub fn part_2(s: &str) -> PrintableArray<9> {
    solve_generic::<9, false>(s)
}

pub fn solve_generic<const STACKS: usize, const CHANGE_ORDER: bool>(s: &str) -> PrintableArray<STACKS> {
    let mut lines = s.split_terminator('\n').map(str::as_bytes);
    let mut warehouse = Warehouse::<STACKS>::parse(&mut lines);
    for line in lines.skip(1) {
        let mut line = &line[5..];
        let count = parse_number(&mut line);
        let from = usize::from(line[6] & 0b1111);
        let to = usize::from(line[11] & 0b1111);
        if CHANGE_ORDER {
            for _ in 0..count {
                warehouse.move_crate(from - 1, to - 1);
            }
        } else {
            warehouse.move_multiple_crates(count, from - 1, to - 1);
        }
    }
    let mut output = [0b0100_0000; STACKS];
    for (from, out) in output.iter_mut().enumerate() {
        *out = *out | warehouse.pop_crate(from);
    }
    PrintableArray(output)
}

#[test]
fn test_part_1_example() {
    let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
    assert_eq!(solve_generic::<3, true>(input).to_string(), "CMZ");
}

#[test]
fn test_part_2_example() {
    let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
    assert_eq!(solve_generic::<3, false>(input).to_string(), "MCD");
}

#[test]
fn test_part_1() {
    let input = include_str!("../input/2022/day5.txt");
    assert_eq!(part_1(input).to_string(), "ZSQVCCJLL");
}

#[test]
fn test_part_2() {
    let input = include_str!("../input/2022/day5.txt");
    assert_eq!(part_2(input).to_string(), "QZFJRWHGS");
}
