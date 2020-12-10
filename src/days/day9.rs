use super::AocDay;
use std::io::BufRead;

// for testing between 5(sample input) and 25
const SIZE: usize = 25;

fn find_invalid_p1(nums: &[usize]) -> Option<usize> {
    for i in SIZE..nums.len() {
        let mut pre = nums[(i - SIZE)..i].to_vec();
        pre.sort_unstable();

        let current_num = nums[i];

        let mut found = false;
        for &inner in &pre {
            if let Some(diff) = current_num.checked_sub(inner) {
                if pre.binary_search(&diff).is_ok() {
                    found = true;
                }
            }
        }

        if !found {
            return Some(current_num);
        }
    }
    None
}

fn find_weakness_p2(nums: &[usize], invalid: usize) -> Option<usize> {
    assert!(!nums.is_empty());

    let mut start_i = 0;
    let mut end_i = 1;

    let mut sum = nums[0];

    while end_i <= nums.len() && start_i <= nums.len() {
        match sum.cmp(&invalid) {
            std::cmp::Ordering::Less => {
                sum += nums[end_i];
                end_i += 1;
            }
            std::cmp::Ordering::Equal => {
                let mut result = nums[start_i..end_i].to_vec();
                result.sort_unstable();

                return Some(result.first()? + result.last()?);
            }
            std::cmp::Ordering::Greater => {
                sum -= nums[start_i];
                start_i += 1;

                // flush remaining
                while sum > invalid {
                    sum -= nums[end_i - 1];
                    end_i -= 1;
                }
            }
        }
    }

    None
}

pub struct Day9;
impl AocDay for Day9 {
    fn run<R: BufRead>(reader: R) {
        let nums: Vec<_> = reader
            .lines()
            .filter_map(|l| l.ok())
            .take_while(|l| !l.is_empty())
            .filter_map(|l| l.parse::<usize>().ok())
            .collect();

        let p1 = find_invalid_p1(&nums).unwrap();
        let p2 = find_weakness_p2(&nums, p1).unwrap();

        println!("Part1: {}", p1);
        println!("Part2: {}", p2);
    }
}
