use aoc_derive::build_days;
use std::io::BufRead;

pub trait AocDay {
    fn run<R: BufRead>(reader: R);

    fn run_timed<R: BufRead>(reader: R) {
        let start = std::time::Instant::now();
        Self::run(reader);
        println!("Took: {} ms", start.elapsed().as_secs_f64() * 1000.);
    }
}

build_days!(1, 10, run_timed);
