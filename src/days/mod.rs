use std::io::BufRead;
pub mod day1;

pub use day1::Day1;

pub trait AocDay {
    fn run<R: BufRead>(reader: R);
}
