use aoc_derive::impl_day;

impl_day!(1, |reader| {
    let mut inputs = reader
        .lines()
        .filter_map(|s| s.ok().map(|s| s.parse::<u32>().ok()))
        .filter_map(|e| e)
        .collect::<Vec<u32>>();

    inputs.sort_unstable();

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
});
