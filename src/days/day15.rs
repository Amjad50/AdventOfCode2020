use aoc_derive::impl_day;
use std::collections::HashMap;

fn solve(starting_nums: &[u32], number_to_predict: u32) -> u32 {
    let mut map = HashMap::<u32, (u32, u32)>::new();

    let mut last = 0;

    for (i, &num) in starting_nums.iter().enumerate() {
        map.insert(num, (0, i as u32 + 1));
        last = num;
    }

    for i in (starting_nums.len() + 1)..(number_to_predict as usize + 1) {
        let (time_before, last_time) = map[&last];

        if time_before == 0 {
            last = 0;
        } else {
            last = last_time - time_before;
        }
        println!("{}", last);

        if let Some((_old_time_before, old_last_time)) = map.insert(last, (0, i as u32)) {
            map.insert(last, (old_last_time, i as u32));
        }
    }

    last
}

impl_day!(15, |reader| {
    let nums: Vec<_> = reader
        .split(',' as u8)
        .filter_map(|d| d.ok())
        .filter_map(|d| {
            let mut result = 0u32;
            for &c in d
                .iter()
                .skip_while(|&c| (*c as char).is_whitespace())
                .take_while(|&c| !(*c as char).is_whitespace())
            {
                if c >= '0' as u8 && c <= '9' as u8 {
                    result *= 10;
                    result += (c - '0' as u8) as u32;
                } else {
                    return None;
                }
            }
            Some(result)
        })
        .collect();

    let p1 = solve(&nums, 2020);
    let p2 = solve(&nums, 30000000);

    println!("Part1: {}", p1);
    println!("Part2: {}", p2);
});
