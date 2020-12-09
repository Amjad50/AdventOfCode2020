use super::AocDay;
use std::collections::HashSet;
use std::io::BufRead;

pub struct Day6;
impl AocDay for Day6 {
    fn run<R: BufRead>(reader: R) {
        let mut p1_set = HashSet::<char>::new();
        let mut p2_set = HashSet::<char>::new();

        let mut p1 = 0;
        let mut p2 = 0;

        let mut start_person = true;

        macro_rules! end_of_group {
            () => {
                p1 += p1_set.len();
                p2 += p2_set.len();
                p1_set.clear();
                p2_set.clear();
            };
        }

        for line in reader.lines().filter_map(|l| l.ok()) {
            if line.is_empty() {
                end_of_group!();
                start_person = true;
            } else {
                let person_set = line.chars().collect::<HashSet<char>>();

                p1_set = p1_set
                    .union(&person_set)
                    .copied()
                    .collect::<HashSet<char>>();

                if start_person {
                    p2_set = p2_set
                        .union(&person_set)
                        .copied()
                        .collect::<HashSet<char>>();
                } else {
                    p2_set = p2_set
                        .intersection(&person_set)
                        .copied()
                        .collect::<HashSet<char>>();
                }

                start_person = false;
            }
        }
        end_of_group!();

        println!("Part1: {}", p1);
        println!("Part2: {}", p2);
    }
}
