use super::AocDay;
use std::io::BufRead;

fn get_jolts_diff_p1(jolts: &Vec<u64>) -> usize {
    let mut diffs = [0; 3];

    for i in 1..jolts.len() {
        let diff = jolts[i] - jolts[i - 1];

        diffs[diff as usize - 1] += 1;
    }

    diffs[2] * diffs[0]
}

fn number_of_arrangements_p2(jolts: &Vec<u64>) -> u64 {
    let mut number_of_arrangements = vec![0; jolts.len()];

    // build the number of arrangmenets possible for each jolt
    for i in 1..jolts.len() {
        for j in (i.saturating_sub(3)..i).rev() {
            if jolts[i] - jolts[j] <= 3 {
                number_of_arrangements[i] += 1;
            } else {
                break;
            }
        }
    }

    // go through all arrangmenets and reduce it
    //
    // example:
    // [1, 2, 3] => [1, 1, 4] (as there are some duplicates between them, so instead of 6 (2*3) we
    // get 4 arrangmenets).
    //
    // [1, 2, 3, 3] => [1, 1, 4, 3] => [1, 1, 1, 7] (Here instead of doing 4+3-1, like the above
    // we need to do 4+3, tbh not sure why is that).
    //
    // [1, 2, 3, 2] => [1, 1, 4, 2] => [1, 1, 1, 6] (same as above)
    for i in 1..number_of_arrangements.len() {
        if number_of_arrangements[i] > 1 {
            let should_minus = number_of_arrangements[i - 1] < number_of_arrangements[i];

            number_of_arrangements[i] += number_of_arrangements[i - 1] - should_minus as u64;
            number_of_arrangements[i - 1] = 1;
        }
    }

    // skip the 0 at the beginning
    number_of_arrangements
        .iter()
        .skip(1)
        .fold(1, |acc, d| acc * d)
}

pub struct Day10;
impl AocDay for Day10 {
    fn run<R: BufRead>(reader: R) {
        let mut jolts: Vec<_> = reader
            .lines()
            .filter_map(|l| l.ok())
            .take_while(|l| !l.is_empty())
            .filter_map(|l| l.parse::<u64>().ok())
            .collect();

        jolts.sort();

        // add the first and last nodes
        jolts.insert(0, 0);
        jolts.push(jolts.last().unwrap() + 3);

        let p1 = get_jolts_diff_p1(&jolts);
        let p2 = number_of_arrangements_p2(&jolts);

        println!("Part1: {}", p1);
        println!("Part2: {}", p2);
    }
}
