use std::io::BufRead;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;

pub use day1::Day1;
pub use day2::Day2;
pub use day3::Day3;
pub use day4::Day4;
pub use day5::Day5;

pub trait AocDay {
    fn run<R: BufRead>(reader: R);
}

pub fn get_runner<R: BufRead>(day: usize) -> fn(R) {
    match day {
        1 => Day1::run,
        2 => Day2::run,
        3 => Day3::run,
        4 => Day4::run,
        5 => Day5::run,
        _ => todo!("This day is not implemented yet"),
    }
}
