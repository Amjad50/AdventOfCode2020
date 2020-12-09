use super::AocDay;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::io::BufRead;

lazy_static! {
    static ref HEXRE: Regex = Regex::new(r"[0-9a-f]{6}").unwrap();
}

const EYE_COLORS: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

pub struct Day4;

fn is_passport_valid_p1(map: &HashMap<String, String>) -> bool {
    map.len() == 8 || (map.len() == 7 && !map.contains_key("cid"))
}

fn number_check(value: &str, start: u16, end: u16) -> bool {
    if let Ok(value) = value.parse::<u16>() {
        if value > end || value < start {
            return false;
        }
    } else {
        return false;
    }

    true
}

fn is_passport_valid_p2(map: &HashMap<String, String>) -> bool {
    if is_passport_valid_p1(map) {
        for (id, value) in map {
            match &id[..] {
                "byr" => {
                    if !number_check(value, 1920, 2002) {
                        return false;
                    }
                }
                "iyr" => {
                    if !number_check(value, 2010, 2020) {
                        return false;
                    }
                }
                "eyr" => {
                    if !number_check(value, 2020, 2030) {
                        return false;
                    }
                }
                "hgt" => {
                    if let Some(value) = value.strip_suffix("cm") {
                        if !number_check(value, 150, 193) {
                            return false;
                        }
                    } else if let Some(value) = value.strip_suffix("in") {
                        if !number_check(value, 59, 76) {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                "hcl" => {
                    if let Some(value) = value.strip_prefix('#') {
                        if !HEXRE.is_match(value) {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                "ecl" => {
                    if !EYE_COLORS.contains(&&value[..]) {
                        return false;
                    }
                }
                "pid" => {
                    if value.len() != 9 {
                        return false;
                    }
                    if value.parse::<u32>().is_err() {
                        return false;
                    }
                }
                _ => {}
            }
        }
        true
    } else {
        false
    }
}

impl AocDay for Day4 {
    fn run<R: BufRead>(reader: R) {
        let mut map = HashMap::new();
        let mut p1 = 0u32;
        let mut p2 = 0u32;

        // for easier duplication of code
        macro_rules! validate_map {
            () => {
                p1 += is_passport_valid_p1(&map) as u32;
                p2 += is_passport_valid_p2(&map) as u32;
                map.clear();
            };
        }

        for line in reader.lines().filter_map(|l| l.ok()) {
            if line.is_empty() {
                validate_map!();
            } else {
                for split in line.split(' ') {
                    let field = split.split(':').take(2).collect::<Vec<&str>>();

                    assert_eq!(field.len(), 2);

                    map.insert(field[0].to_string(), field[1].to_string());
                }
            }
        }

        validate_map!();

        println!("Part1: {}", p1);
        println!("Part2: {}", p2);
    }
}
