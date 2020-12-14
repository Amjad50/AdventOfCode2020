use aoc_derive::impl_day;

fn get_min_bus_time_p1(timestamp: u32, buses: &[Option<u32>]) -> u32 {
    let min_bus_wait_time = buses
        .iter()
        .filter_map(|b| *b)
        .map(|b| ((b - timestamp % b) % b, b))
        .min_by(|a, b| a.0.cmp(&b.0))
        .unwrap_or((0, 0));

    min_bus_wait_time.0 * min_bus_wait_time.1
}

fn solve_competition_p2(buses: &[Option<u32>]) -> u64 {
    let buses: Vec<_> = buses
        .iter()
        .enumerate()
        .filter_map(|(i, l)| l.map(|l| (i as u64, l as u64)))
        .collect();

    // increment by the first bus value, so `current` will be divisible by it
    let mut increment = buses[0].1;
    let mut skip = 1;
    let mut current = 0;

    'outer: loop {
        current += increment;

        for &(i, b) in buses.iter().skip(skip) {
            if (current + i) % b != 0 {
                continue 'outer;
            }
            // when finding another bus divisible by the current value, change the increment to
            // become (old_increment * b), so that `current` will be divisible by both
            // values at the same time, and also skip those bus values,
            // because we don't want to overflow increment
            skip += 1;
            increment *= b;
        }

        return current;
    }
}

impl_day!(13, |reader| {
    let lines = reader
        .lines()
        .filter_map(|l| l.ok())
        .collect::<Vec<String>>();

    let timestamp = lines[0].parse::<u32>().unwrap();
    let buses = lines[1]
        .split(',')
        .map(|c| c.parse::<u32>().ok())
        .collect::<Vec<Option<u32>>>();

    let p1 = get_min_bus_time_p1(timestamp, &buses);
    let p2 = solve_competition_p2(&buses);

    println!("Part1: {:?}", p1);
    println!("Part2: {:?}", p2);
});
