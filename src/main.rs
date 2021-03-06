mod days;

use days::get_runner;
use std::env::args;
use std::fs::File;
use std::io::{BufReader, Error as ioError};

fn main() -> Result<(), ioError> {
    let args = args().collect::<Vec<String>>();

    if args.len() < 2 {
        eprintln!("USAGE: {} <day>", args[0]);
    } else if let Ok(day) = args[1].parse::<usize>() {
        let day_runner = get_runner(day);

        let filename = format!("inputs/{:02}.txt", day);
        day_runner(BufReader::new(File::open(filename)?))
    } else {
        eprintln!("Please input a valid number for the day");
    }

    Ok(())
}
