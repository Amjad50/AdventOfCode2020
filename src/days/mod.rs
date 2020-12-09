use std::io::BufRead;
pub mod day1;
pub mod day2;
pub mod day3;

pub use day1::Day1;
pub use day2::Day2;
pub use day3::Day3;

pub trait AocDay {
    fn run<R: BufRead>(reader: R);
}
