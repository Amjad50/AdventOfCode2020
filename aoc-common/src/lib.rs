use std::io::BufRead;

pub trait AocDay {
    fn run<R: BufRead>(reader: R);

    fn run_timed<R: BufRead>(reader: R) {
        let start = std::time::Instant::now();
        Self::run(reader);
        println!("Took: {} ms", start.elapsed().as_secs_f64() * 1000.);
    }
}
