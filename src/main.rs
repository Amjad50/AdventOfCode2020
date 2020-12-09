mod days;
use days::{AocDay, Day1};
use std::env::args;
use std::fs::File;
use std::io::{BufReader, Error as ioError};

fn main() -> Result<(), ioError> {
    let args = args().collect::<Vec<String>>();

    if args.len() < 2 {
        eprintln!("USAGE: {} <day>", args[0]);
    } else {
        if let Ok(day) = args[1].parse::<usize>() {
            let day_runner = match day {
                1 => Day1::run,
                _ => todo!("This day is not implemented yet"),
            };

            let filename = format!("inputs/{}.txt", args[1]);
            day_runner(BufReader::new(File::open(filename)?))
        } else {
            eprintln!("Please input a valid number for the day");
        }
    }

    Ok(())
}
