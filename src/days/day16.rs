use aoc_derive::impl_day;
use std::collections::BTreeSet;
use std::ops::RangeInclusive;

#[derive(Debug, Clone)]
struct Rule {
    name: String,
    range1: RangeInclusive<u32>,
    range2: RangeInclusive<u32>,
}

impl Default for Rule {
    fn default() -> Self {
        Self {
            name: String::new(),
            range1: 0..=0,
            range2: 0..=0,
        }
    }
}

fn parse_range(range: &str) -> RangeInclusive<u32> {
    let mut split = range.split('-').filter_map(|d| d.parse::<u32>().ok());

    split.next().unwrap()..=split.next().unwrap()
}

fn parse_rules(rules: &[String]) -> Vec<Rule> {
    rules
        .iter()
        .map(|r| {
            let name_end_index = r.find(':').unwrap_or(0);
            let name = r[..name_end_index].to_string();

            let mut ranges_split = r[(name_end_index + 2)..].split(" or ");
            let range1 = parse_range(ranges_split.next().unwrap());
            let range2 = parse_range(ranges_split.next().unwrap());

            Rule {
                name,
                range1,
                range2,
            }
        })
        .collect()
}

fn parse_tickets(tickets: &[String]) -> Vec<Vec<u32>> {
    tickets
        .iter()
        .map(|t| t.split(',').filter_map(|d| d.parse::<u32>().ok()).collect())
        .collect()
}

/// returns the error rate and the remaining tickets that are still valid
fn scan_and_filter_tickets(rules: &[Rule], others_tickets: Vec<Vec<u32>>) -> (u32, Vec<Vec<u32>>) {
    let mut sum = 0;

    let mut valid_tickets = Vec::with_capacity(others_tickets.len());

    for ticket in others_tickets {
        let mut ticket_valid = true;

        for n in &ticket {
            let mut invalid = true;
            for r in rules {
                if r.range1.contains(n) || r.range2.contains(n) {
                    invalid = false;
                }
            }

            if invalid {
                ticket_valid = false;
                sum += n;
            }
        }

        if ticket_valid {
            valid_tickets.push(ticket);
        }
    }

    (sum, valid_tickets)
}

/// returns the correct rules indices
fn get_correct_rules_order(rules: &[Rule], valid_tickets: &[Vec<u32>]) -> Vec<u32> {
    let mut sets: Vec<BTreeSet<u32>> = vec![(0..rules.len() as u32).collect(); rules.len()];

    for ticket in valid_tickets {
        for (i, n) in ticket.iter().enumerate() {
            let mut s = BTreeSet::new();
            for (j, r) in rules.iter().enumerate() {
                if r.range1.contains(n) || r.range2.contains(n) {
                    s.insert(j as u32);
                }
            }

            sets[i] = sets[i].intersection(&s).copied().collect();
        }
    }

    let mut sets: Vec<_> = sets.iter().enumerate().collect();
    sets.sort_by(|a, b| a.1.len().cmp(&b.1.len()));

    let mut used_set = BTreeSet::new();
    let mut rules_result = vec![None; rules.len()];

    for (i, set) in sets {
        let rule_index = set.difference(&used_set).next().unwrap();
        rules_result[i] = Some(*rule_index);

        used_set = used_set.union(&set).copied().collect();
    }

    rules_result
        .iter()
        .copied()
        .collect::<Option<Vec<u32>>>()
        .unwrap()
}

fn get_departure_fields_multiplied_p2(
    rules: &[Rule],
    rules_correct_indcies: &[u32],
    our_ticket: &[u32],
) -> u64 {
    let departure_fields_indcies: Vec<_> = rules
        .iter()
        .enumerate()
        .filter_map(|(i, r)| {
            if r.name.contains("departure") {
                Some(i as u32)
            } else {
                None
            }
        })
        .collect();

    let ticket_indcies: Vec<_> = rules_correct_indcies
        .iter()
        .enumerate()
        .filter_map(|(i, r_i)| {
            if departure_fields_indcies.contains(&r_i) {
                Some(i as u32)
            } else {
                None
            }
        })
        .collect();

    our_ticket
        .iter()
        .enumerate()
        .filter_map(|(i, v)| {
            if ticket_indcies.contains(&(i as u32)) {
                Some(*v as u64)
            } else {
                None
            }
        })
        .product()
}

impl_day!(16, |reader| {
    let lines: Vec<_> = reader.lines().filter_map(|l| l.ok()).collect();
    let mut splits = lines.split(|l| l.is_empty());

    let rules = parse_rules(splits.next().unwrap());
    let our_ticket = &parse_tickets(&splits.next().unwrap()[1..=1])[0];
    let their_tickets = parse_tickets(&splits.next().unwrap()[1..]);

    // make sure no more input
    assert!(splits.next().is_none());

    let (p1, valid_tickets) = scan_and_filter_tickets(&rules, their_tickets);

    // part2
    let correct_rules_indcies = get_correct_rules_order(&rules, &valid_tickets);
    let p2 = get_departure_fields_multiplied_p2(&rules, &correct_rules_indcies, &our_ticket);

    println!("Part1: {}", p1);
    println!("Part2: {}", p2);
});
