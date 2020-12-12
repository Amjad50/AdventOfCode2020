use aoc_derive::impl_day;
use regex::Regex;

fn is_password_valid_p1(min: usize, max: usize, ch: char, password: &str) -> bool {
    let mut password_bytes = password.to_string().into_bytes();
    password_bytes.sort_unstable();

    let password_bytes_iter = password_bytes.iter().enumerate();

    let c = ch as u8;

    let first = password_bytes_iter
        .clone()
        .find(|&(_, a)| a == &c)
        .map(|(i, _)| i);

    if let Some(first) = first {
        let last = password_bytes_iter
            .clone()
            .rfind(|&(_, a)| a == &c)
            .map(|(i, _)| i)
            .unwrap();

        let length = last - first + 1;

        length >= min && length <= max
    } else {
        min == 0
    }
}

fn is_password_valid_p2(first: usize, second: usize, ch: char, password: &str) -> bool {
    let chars = password.chars().collect::<Vec<char>>();

    (chars.get(first - 1) == Some(&ch)) ^ (chars.get(second - 1) == Some(&ch))
}

impl_day!(2, |mut reader| {
    let mut input = String::new();
    reader.read_to_string(&mut input).unwrap();

    let re = Regex::new(r"(\d{1,5})-(\d{1,5}) ([a-z]): ([a-z]*)").unwrap();

    let mut p1_counter = 0;
    let mut p2_counter = 0;

    for cap in re.captures_iter(&input) {
        let (first, second, ch, password) = (
            cap[1].parse::<usize>().unwrap(),
            cap[2].parse::<usize>().unwrap(),
            cap[3].chars().next().unwrap(),
            &cap[4],
        );

        p1_counter += is_password_valid_p1(first, second, ch, password) as u32;
        p2_counter += is_password_valid_p2(first, second, ch, password) as u32;
    }

    println!("Part1: {}", p1_counter);
    println!("Part2: {}", p2_counter);
});
