#![feature(iter_array_chunks)]
#![feature(iter_advance_by)]
#![feature(array_windows)]

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

aoc_main::main! {
    year 2022;
    day1 => part_1, part_2;
    day2 => part_1, part_2;
    day3 => part_1, part_2;
    day4 => part_1, part_2;
    day5 => part_1, part_2;
    day6 => part_1, part_2;
    day7 => part_1, part_2;
    day8 => part_1, part_2;
    day9 => part_1;
}
