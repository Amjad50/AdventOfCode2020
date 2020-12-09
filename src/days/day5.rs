use super::AocDay;
use std::io::BufRead;

pub struct Day5;
impl AocDay for Day5 {
    fn run<R: BufRead>(reader: R) {
        let mut ids = reader
            .lines()
            .filter_map(|l| l.ok())
            .filter_map(|line| {
                let id = &line
                    .replace(&['F', 'L'][..], "0")
                    .replace(&['B', 'R'][..], "1");

                u16::from_str_radix(id, 2).ok()
            })
            .collect::<Vec<u16>>();

        ids.sort();

        println!("Part1: {}", ids.last().unwrap());

        let mut last = &ids[0];
        for item in &ids[1..] {
            if item - last != 1 {
                println!("Part2: {}", item - 1);
                break;
            }
            last = item;
        }
    }
}
