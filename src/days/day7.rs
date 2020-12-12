use aoc_derive::impl_day;
use std::collections::{HashMap, HashSet, VecDeque};

type InnerBags = Vec<(u32, String)>;

fn compute_p1(bags_map: &HashMap<String, InnerBags>) -> usize {
    let mut search_space = VecDeque::new();
    let mut result = HashSet::new();

    search_space.push_back("shiny gold".to_string());

    while let Some(current_search) = search_space.pop_front() {
        for (outer, inner) in bags_map {
            // This can be improved
            if inner.iter().any(|b| b.1 == current_search) {
                search_space.push_back(outer.to_string());
                result.insert(outer);
            }
        }
    }

    result.len()
}

fn get_bag_inners_size(
    bags_map: &HashMap<String, InnerBags>,
    bag: &str,
    memoize_map: &mut HashMap<String, usize>,
) -> usize {
    if let Some(&result) = memoize_map.get(bag) {
        return result;
    }

    let inner = &bags_map[bag];

    let result = 1 + inner.iter().fold(0, |acc, (n, bag_name)| {
        acc + *n as usize * get_bag_inners_size(bags_map, bag_name, memoize_map)
    });

    memoize_map.insert(bag.to_string(), result);

    result
}

fn compute_p2(bags_map: &HashMap<String, InnerBags>) -> usize {
    let mut memoize_map: HashMap<String, usize> = HashMap::new();
    get_bag_inners_size(bags_map, &"shiny gold".to_string(), &mut memoize_map) - 1
}

impl_day!(7, |reader| {
    let mut bags_map: HashMap<String, InnerBags> = HashMap::new();

    for line in reader.lines().filter_map(|l| l.ok()) {
        let outer_bag_last_index = line.find(" bags contain ").unwrap();
        let outer_bag_name = &line[..outer_bag_last_index];

        let inner_bags_start_index = outer_bag_last_index + " bags contain ".len();
        let inner_bags_description = &line[inner_bags_start_index..(line.len() - 1)];
        let inner_bags_description = inner_bags_description.split(", ");

        let inner_bags = inner_bags_description
            .filter_map(|b| {
                if b.starts_with('n') {
                    None
                } else {
                    let b = b.trim_end_matches(" bags").trim_end_matches(" bag");
                    let b = b.split(' ').collect::<Vec<&str>>();
                    let n = b[0].parse::<u32>().unwrap();
                    let bag_name = b[1..].join(" ");

                    Some((n, bag_name))
                }
            })
            .collect::<InnerBags>();

        bags_map.insert(outer_bag_name.to_string(), inner_bags);
    }

    let p1 = compute_p1(&bags_map);
    let p2 = compute_p2(&bags_map);

    println!("Part1: {}", p1);
    println!("Part2: {}", p2);
});
