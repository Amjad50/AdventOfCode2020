use super::AocDay;
use std::io::BufRead;

pub struct Day1;

impl AocDay for Day1 {
    fn run<R: BufRead>(reader: R) {
        let mut inputs = reader
            .lines()
            .filter_map(|s| s.ok().map(|s| s.parse::<u32>().ok()))
            .filter_map(|e| e)
            .collect::<Vec<u32>>();

        inputs.sort();

        let mut part1 = 0;
        let mut part2 = 0;

        for i in 0..inputs.len() {
            for j in 0..inputs.len() {
                let sum = inputs[i] + inputs[j];
                if sum == 2020 {
                    part1 = inputs[i] * inputs[j];
                }

                // part 2
                if let Ok(value) = inputs.binary_search(&2020u32.wrapping_sub(sum)) {
                    part2 = inputs[value] * inputs[i] * inputs[j];
                }
            }
        }

        println!("Part1: {}", part1);
        println!("Part2: {}", part2);
    }
}
