use aoc_derive::build_days;
use std::io::BufRead;

pub trait AocDay {
    fn run<R: BufRead>(reader: R);
}

build_days!(1, 5);
